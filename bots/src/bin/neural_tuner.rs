use std::cmp::Ordering;
use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use bots::neural_bot::{NeuralBot, NeuralWeights};
use bots::{available_bots, get_bot_names, ml_bot::MlBot};
use game::bot::bot::{Bot, BotConstructor, BotController};
use game::game::game::Game;
use game::map::structs::map_config::MapConfig;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rayon::prelude::*;

const DEFAULT_ITERATIONS: usize = 100;
const DEFAULT_GAMES_PER_EVAL: usize = 48;
const DEFAULT_SIGMA: f32 = 0.25;
const MIN_SIGMA: f32 = 0.03;
const SIGMA_DECAY: f32 = 0.95;
const SIGMA_GROWTH: f32 = 1.05;
const MAX_SIGMA: f32 = 0.40;
const MAP_SIZES: [usize; 7] = [7, 9, 11, 13, 15, 17, 19];
const BOMB_TIMER: usize = 4;
const BOMB_RADIUS: usize = 3;
const ENDGAME_TURN: usize = 500;
const CANDIDATE_NAME: &str = "NeuralCandidate";
const MIN_PLAYERS: usize = 2;
const MAX_PLAYERS: usize = 4;
const DEFAULT_BATCH_SIZE: usize = 12;
const DEFAULT_ELITE_COUNT: usize = 3;
const DEFAULT_CELEBRATE_WEIGHT: f32 = 1.2;
const DEFAULT_PUNISH_WEIGHT: f32 = 0.4;
const DEFAULT_AGGREGATE_STEP: f32 = 0.8;
const DEFAULT_QUICK_GAMES: usize = 0;
const DEFAULT_QUICK_TOP: usize = 2;
const DEFAULT_VERIFY_EVERY: usize = 1;
const MOMENTUM_BASE_SCALE: f32 = 1.0;
const MOMENTUM_GROWTH: f32 = 1.3;
const MOMENTUM_DECAY: f32 = 0.5;
const MOMENTUM_MIN_SCALE: f32 = 0.05;
const MOMENTUM_MAX_SCALE: f32 = 5.0;

#[derive(Clone, Copy)]
struct GameConfig {
    num_players: usize,
    size: usize,
}

fn tournament_configs() -> Vec<GameConfig> {
    let player_counts = [2, 3, 4];
    let mut configs = Vec::new();
    for size in MAP_SIZES {
        for &players in &player_counts {
            configs.push(GameConfig {
                num_players: players,
                size,
            });
        }
    }
    configs
}

struct NamedFactory {
    constructor: BotConstructor,
}

impl NamedFactory {
    fn build(&self) -> BotController {
        (self.constructor.as_ref())()
    }
}

#[derive(Copy, Clone, Debug)]
enum StartMode {
    Random,
    Default,
}

#[derive(Clone)]
struct ScoredCandidate {
    weights: NeuralWeights,
    score: f32,
    prelim_score: Option<f32>,
    label: CandidateLabel,
}

struct BestSnapshot {
    iteration: usize,
    label: String,
    score: f32,
    sigma: f32,
    literal: String,
}

#[derive(Clone, Copy)]
enum CandidateLabel {
    Mutation(usize),
    Aggregate,
    Momentum,
}

impl CandidateLabel {
    fn describe(self, batch: usize) -> String {
        match self {
            CandidateLabel::Mutation(idx) => format!("mut {}/{}", idx + 1, batch),
            CandidateLabel::Aggregate => "aggregate".to_string(),
            CandidateLabel::Momentum => "momentum".to_string(),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let iterations = args
        .get(1)
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(DEFAULT_ITERATIONS);
    let games_per_eval = args
        .get(2)
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(DEFAULT_GAMES_PER_EVAL);
    let mut sigma = args
        .get(3)
        .and_then(|value| value.parse::<f32>().ok())
        .unwrap_or(DEFAULT_SIGMA);
    let seed = args
        .get(4)
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(0xA51CE5EED5EED_u64);

    let mut start_mode = StartMode::Random;
    let mut duel_mode = false;
    let mut show_intermediate_weights = false;
    let mut tournament_sampling = false;
    let mut verification_games: usize = 0;
    let mut verification_threshold: f32 = 0.0;
    let mut batch_size = DEFAULT_BATCH_SIZE;
    let mut elite_count = DEFAULT_ELITE_COUNT;
    let mut celebrate_weight = DEFAULT_CELEBRATE_WEIGHT;
    let mut punish_weight = DEFAULT_PUNISH_WEIGHT;
    let mut aggregate_step = DEFAULT_AGGREGATE_STEP;
    let mut quick_games = DEFAULT_QUICK_GAMES;
    let mut quick_top = DEFAULT_QUICK_TOP;
    let mut verify_every = DEFAULT_VERIFY_EVERY;

    for arg in args.iter().skip(5) {
        match arg.as_str() {
            "--start=default" | "--default-start" | "--start-default" => {
                start_mode = StartMode::Default
            }
            "--start=random" | "--random-start" | "--start-random" => {
                start_mode = StartMode::Random
            }
            "--duel" | "--1v1" => duel_mode = true,
            "--show-weights" | "--verbose" => show_intermediate_weights = true,
            "--tournament" | "--match-tournament" => tournament_sampling = true,
            _ if arg.starts_with("--batch-size=") => {
                match arg.trim_start_matches("--batch-size=").parse::<usize>() {
                    Ok(value) => batch_size = value,
                    Err(_) => eprintln!(
                        "Warning: could not parse batch size from '{arg}', ignoring"
                    ),
                }
            }
            _ if arg.starts_with("--elite-count=") => {
                match arg.trim_start_matches("--elite-count=").parse::<usize>() {
                    Ok(value) => elite_count = value,
                    Err(_) => eprintln!(
                        "Warning: could not parse elite count from '{arg}', ignoring"
                    ),
                }
            }
            _ if arg.starts_with("--celebrate=") => {
                match arg.trim_start_matches("--celebrate=").parse::<f32>() {
                    Ok(value) => celebrate_weight = value,
                    Err(_) => eprintln!(
                        "Warning: could not parse celebrate weight from '{arg}', ignoring"
                    ),
                }
            }
            _ if arg.starts_with("--punish=") => {
                match arg.trim_start_matches("--punish=").parse::<f32>() {
                    Ok(value) => punish_weight = value,
                    Err(_) => eprintln!(
                        "Warning: could not parse punish weight from '{arg}', ignoring"
                    ),
                }
            }
            _ if arg.starts_with("--aggregate-step=") => {
                match arg.trim_start_matches("--aggregate-step=").parse::<f32>() {
                    Ok(value) => aggregate_step = value,
                    Err(_) => eprintln!(
                        "Warning: could not parse aggregate step from '{arg}', ignoring"
                    ),
                }
            }
            _ if arg.starts_with("--quick-games=") => {
                match arg.trim_start_matches("--quick-games=").parse::<usize>() {
                    Ok(value) => quick_games = value,
                    Err(_) => eprintln!(
                        "Warning: could not parse quick-games from '{arg}', ignoring"
                    ),
                }
            }
            _ if arg.starts_with("--quick-top=") => {
                match arg.trim_start_matches("--quick-top=").parse::<usize>() {
                    Ok(value) => quick_top = value,
                    Err(_) => eprintln!(
                        "Warning: could not parse quick-top from '{arg}', ignoring"
                    ),
                }
            }
            _ if arg.starts_with("--verify-every=") => {
                match arg.trim_start_matches("--verify-every=").parse::<usize>() {
                    Ok(value) => verify_every = value,
                    Err(_) => eprintln!(
                        "Warning: could not parse verify-every from '{arg}', ignoring"
                    ),
                }
            }
            _ if arg.starts_with("--verify-games=") => {
                match arg.trim_start_matches("--verify-games=").parse::<usize>() {
                    Ok(value) => verification_games = value,
                    Err(_) => eprintln!(
                        "Warning: could not parse verification game count from '{arg}', ignoring"
                    ),
                }
            }
            _ if arg.starts_with("--verify-threshold=") => {
                match arg.trim_start_matches("--verify-threshold=").parse::<f32>() {
                    Ok(value) => verification_threshold = value,
                    Err(_) => eprintln!(
                        "Warning: could not parse verification threshold from '{arg}', ignoring"
                    ),
                }
            }
            other => {
                eprintln!("Warning: unrecognized option '{other}', ignoring");
            }
        }
    }

    if batch_size == 0 {
        batch_size = 1;
    }
    if elite_count > batch_size {
        elite_count = batch_size;
    }
    if quick_top == 0 {
        quick_top = 1;
    }
    if quick_top > batch_size {
        quick_top = batch_size;
    }
    if quick_games > games_per_eval {
        quick_games = games_per_eval;
    }
    if quick_games == 0 || quick_games >= games_per_eval {
        quick_games = 0;
    }
    if verify_every == 0 {
        verify_every = 1;
    }
    if celebrate_weight < 0.0 {
        celebrate_weight = 0.0;
    }
    if punish_weight < 0.0 {
        punish_weight = 0.0;
    }

    println!(
        "Iterations: {iterations}, games/eval: {games_per_eval}, initial sigma: {sigma:.3}, seed: {seed}"
    );
    println!("Start mode: {:?}, duel mode: {}", start_mode, duel_mode);
    if verification_games > 0 {
        println!(
            "Verification enabled: {verification_games} games, threshold {verification_threshold:.3}, every {verify_every} iteration(s)"
        );
    }
    println!(
        "Batch size {batch_size}, elite count {elite_count}, celebrate {celebrate_weight:.3}, punish {punish_weight:.3}, aggregate step {aggregate_step:.3}"
    );
    if quick_games > 0 {
        println!(
            "Quick evaluation enabled: {quick_games} games (top {quick_top} re-evaluated with {games_per_eval})"
        );
    }

    let mut rng = StdRng::seed_from_u64(seed);
    let constructors = available_bots();
    let bot_names = get_bot_names();
    let opponent_pool: Vec<NamedFactory> = bot_names
        .into_iter()
        .zip(constructors.into_iter())
        .filter_map(|(name, constructor)| {
            if name == "NeuralBot" {
                None
            } else {
                Some(NamedFactory { constructor })
            }
        })
        .collect();
    let opponent_factories = Arc::new(opponent_pool);
    let configs = if tournament_sampling {
        Some(Arc::new(tournament_configs()))
    } else {
        None
    };
    let mut best_weights = match start_mode {
        StartMode::Random => NeuralWeights::random(&mut rng),
        StartMode::Default => NeuralWeights::default(),
    };
    let mut best_score = evaluate(
        &best_weights,
        games_per_eval,
        &mut rng,
        duel_mode,
        &opponent_factories,
        configs.as_deref().map(|list| &list[..]),
        None,
    );
    let mut best_literal = best_weights.format_as_rust();
    let mut momentum_delta: Option<NeuralWeights> = None;
    let mut momentum_scale = 0.0_f32;

    let shared_best = Arc::new(Mutex::new(Some(BestSnapshot {
        iteration: 0,
        label: "baseline".to_string(),
        score: best_score,
        sigma,
        literal: best_literal.clone(),
    })));
    let terminate_requested = Arc::new(AtomicBool::new(false));
    {
        let shared_best = Arc::clone(&shared_best);
        let terminate_requested = Arc::clone(&terminate_requested);
        ctrlc::set_handler(move || {
            eprintln!("Interrupt received. Attempting graceful shutdown...");
            if let Ok(guard) = shared_best.lock() {
                if let Some(snapshot) = guard.as_ref() {
                    eprintln!(
                        "Best candidate so far (iteration {}, {}) with win-rate {:.3} (sigma {:.3})",
                        snapshot.iteration,
                        snapshot.label,
                        snapshot.score,
                        snapshot.sigma
                    );
                    eprintln!("{}", snapshot.literal);
                } else {
                    eprintln!("No candidate evaluated yet.");
                }
            }
            terminate_requested.store(true, AtomicOrdering::SeqCst);
        })
        .expect("Failed to install Ctrl+C handler");
    }

    println!("Baseline win-rate: {:.3}", best_score);

    let start = Instant::now();

    for iteration in 0..iterations {
        if terminate_requested.load(AtomicOrdering::SeqCst) {
            println!("Termination requested, stopping optimization loop.");
            break;
        }
        let iteration_number = iteration + 1;
        let configs_slice = configs.as_deref().map(|list| &list[..]);
        let quick_enabled = quick_games > 0;
        let quick_seeds = if quick_enabled {
            Some((0..quick_games).map(|_| rng.random::<u64>()).collect::<Vec<_>>())
        } else {
            None
        };
        let quick_seed_slice = quick_seeds.as_deref();
        let mut candidates: Vec<ScoredCandidate> = Vec::with_capacity(batch_size + 2);

        if let Some(delta) = momentum_delta.as_ref() {
            if momentum_scale >= MOMENTUM_MIN_SCALE {
                let mut momentum_candidate = best_weights.clone();
                momentum_candidate.add_scaled(delta, momentum_scale);
                let eval_games = if quick_enabled {
                    quick_games
                } else {
                    games_per_eval
                };
                let score = evaluate(
                    &momentum_candidate,
                    eval_games,
                    &mut rng,
                    duel_mode,
                    &opponent_factories,
                    configs_slice,
                    quick_seed_slice,
                );
                candidates.push(ScoredCandidate {
                    weights: momentum_candidate,
                    score,
                    prelim_score: if quick_enabled { Some(score) } else { None },
                    label: CandidateLabel::Momentum,
                });
            }
        }

        for idx in 0..batch_size {
            let candidate = best_weights.perturb(&mut rng, sigma);
            let eval_games = if quick_enabled {
                quick_games
            } else {
                games_per_eval
            };
            let score = evaluate(
                &candidate,
                eval_games,
                &mut rng,
                duel_mode,
                &opponent_factories,
                configs_slice,
                quick_seed_slice,
            );
            candidates.push(ScoredCandidate {
                weights: candidate,
                score,
                prelim_score: if quick_enabled { Some(score) } else { None },
                label: CandidateLabel::Mutation(idx),
            });
        }

        candidates.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(Ordering::Equal)
        });

        if quick_enabled {
            let reevaluate = quick_top.min(candidates.len()).max(1);
            for idx in 0..reevaluate {
                let stage1_score = candidates[idx].score;
                let full_score = evaluate(
                    &candidates[idx].weights,
                    games_per_eval,
                    &mut rng,
                    duel_mode,
                    &opponent_factories,
                    configs_slice,
                    None,
                );
                candidates[idx].prelim_score = Some(stage1_score);
                candidates[idx].score = full_score;
            }
            candidates.sort_by(|a, b| {
                b.score
                    .partial_cmp(&a.score)
                    .unwrap_or(Ordering::Equal)
            });
        }

        let celebrate_count = elite_count.min(candidates.len() / 2);
        if celebrate_count > 0
            && aggregate_step != 0.0
            && (celebrate_weight != 0.0 || punish_weight != 0.0)
        {
            let mut delta = NeuralWeights::zeros();
            if celebrate_weight != 0.0 {
                let celebrate_scale = celebrate_weight / celebrate_count as f32;
                for entry in candidates.iter().take(celebrate_count) {
                    delta.add_weight_diff(&best_weights, &entry.weights, celebrate_scale);
                }
            }
            if punish_weight != 0.0 {
                let punish_scale = punish_weight / celebrate_count as f32;
                for entry in candidates.iter().rev().take(celebrate_count) {
                    delta.add_weight_diff(&best_weights, &entry.weights, -punish_scale);
                }
            }
            let mut aggregate_candidate = best_weights.clone();
            aggregate_candidate.add_scaled(&delta, aggregate_step);
            let aggregate_score = evaluate(
                &aggregate_candidate,
                games_per_eval,
                &mut rng,
                duel_mode,
                &opponent_factories,
                configs_slice,
                None,
            );
            candidates.push(ScoredCandidate {
                weights: aggregate_candidate,
                score: aggregate_score,
                prelim_score: None,
                label: CandidateLabel::Aggregate,
            });
            candidates.sort_by(|a, b| {
                b.score
                    .partial_cmp(&a.score)
                    .unwrap_or(Ordering::Equal)
            });
        }

        let mut accepted_any = false;

        for entry in candidates.into_iter() {
            if entry.score <= best_score {
                break;
            }

            let label = entry.label.describe(batch_size);
            let prelim_note = entry
                .prelim_score
                .map(|value| format!(" (prelim {:.3})", value))
                .unwrap_or_default();
            println!(
                "Iteration {:>3} [{}]: candidate sample {:.3} exceeded current best {:.3} (sigma {:.3}){}",
                iteration_number,
                label,
                entry.score,
                best_score,
                sigma,
                prelim_note
            );

            let should_verify =
                verification_games > 0 && (iteration_number % verify_every == 0);
            let mut accepted = true;
            let mut final_score = entry.score;

            if should_verify {
                let verify_seeds: Vec<u64> =
                    (0..verification_games).map(|_| rng.random::<u64>()).collect();
                let candidate_verified = evaluate(
                    &entry.weights,
                    verification_games,
                    &mut rng,
                    duel_mode,
                    &opponent_factories,
                    configs_slice,
                    Some(&verify_seeds),
                );
                let best_verified = evaluate(
                    &best_weights,
                    verification_games,
                    &mut rng,
                    duel_mode,
                    &opponent_factories,
                    configs_slice,
                    Some(&verify_seeds),
                );

                println!(
                    "Iteration {:>3} [{}]: verification candidate {:.3} vs current {:.3}",
                    iteration_number,
                    label,
                    candidate_verified,
                    best_verified
                );

                if candidate_verified > best_verified + verification_threshold.max(0.0) {
                    final_score = candidate_verified;
                    println!(
                        "Iteration {:>3} [{}]: accepted after verification (win-rate {:.3})",
                        iteration_number,
                        label,
                        final_score
                    );
                } else {
                    println!(
                        "Iteration {:>3} [{}]: rejected after verification",
                        iteration_number,
                        label
                    );
                    accepted = false;
                    if best_verified < best_score {
                        best_score = best_verified;
                    }
                }
            } else if verification_games > 0 {
                println!(
                    "Iteration {:>3} [{}]: skipped verification (verify-every {})",
                    iteration_number,
                    label,
                    verify_every
                );
            }

            if accepted {
                let previous_best = best_weights.clone();
                let is_momentum_candidate = matches!(entry.label, CandidateLabel::Momentum);
                let new_weights = entry.weights;
                let mut delta = NeuralWeights::zeros();
                delta.add_weight_diff(&previous_best, &new_weights, 1.0);
                let delta_norm = delta.l1_norm();

                best_score = final_score;
                best_weights = new_weights;
                best_literal = best_weights.format_as_rust();
                println!(
                    "Iteration {:>3} [{}]: new best win-rate {:.3} (sigma {:.3})",
                    iteration_number,
                    label,
                    best_score,
                    sigma
                );
                if show_intermediate_weights {
                    println!("New best weights (verbose flag enabled):");
                } else {
                    println!("New best NeuralWeights literal:");
                }
                println!("{}", best_literal);
                {
                    let mut guard = shared_best.lock().unwrap();
                    *guard = Some(BestSnapshot {
                        iteration: iteration_number,
                        label: label.clone(),
                        score: best_score,
                        sigma,
                        literal: best_literal.clone(),
                    });
                }
                if delta_norm > f32::EPSILON {
                    if is_momentum_candidate {
                        momentum_scale = if momentum_scale <= 0.0 {
                            MOMENTUM_BASE_SCALE
                        } else {
                            (momentum_scale * MOMENTUM_GROWTH).min(MOMENTUM_MAX_SCALE)
                        };
                    } else {
                        momentum_scale = MOMENTUM_BASE_SCALE;
                    }
                    momentum_delta = Some(delta);
                } else {
                    momentum_delta = None;
                    momentum_scale = 0.0;
                }
                accepted_any = true;
                break;
            }
        }

        if accepted_any {
            sigma = (sigma * SIGMA_DECAY).max(MIN_SIGMA);
        } else {
            sigma = (sigma * SIGMA_GROWTH).min(MAX_SIGMA);
        }
        if !accepted_any {
            if momentum_scale > 0.0 {
                momentum_scale *= MOMENTUM_DECAY;
                if momentum_scale < MOMENTUM_MIN_SCALE {
                    momentum_delta = None;
                    momentum_scale = 0.0;
                }
            }
        }
        if accepted_any {
            let mut guard = shared_best.lock().unwrap();
            if let Some(snapshot) = guard.as_mut() {
                snapshot.sigma = sigma;
            }
        }
    }

    let elapsed = start.elapsed();
    println!("Done in {:.2?}. Best win-rate: {:.3}", elapsed, best_score);
    println!(
        "Final weights (copy into NeuralWeights::default):\n{}",
        best_literal
    );
}

fn evaluate(
    weights: &NeuralWeights,
    games: usize,
    rng: &mut StdRng,
    duel_mode: bool,
    opponents: &Arc<Vec<NamedFactory>>,
    configs: Option<&[GameConfig]>,
    seeds_override: Option<&[u64]>,
) -> f32 {
    let actual_games = match seeds_override {
        Some(seeds) => seeds.len(),
        None => games,
    };

    if actual_games == 0 {
        return 0.0;
    }

    let shared = Arc::new(weights.clone());
    let seeds: Vec<u64> = match seeds_override {
        Some(override_seeds) => override_seeds.to_vec(),
        None => (0..actual_games).map(|_| rng.random::<u64>()).collect(),
    };
    let factory_pool = Arc::clone(opponents);

    let wins = seeds
        .into_par_iter()
        .enumerate()
        .map(|(idx, seed)| {
            let mut local_rng = StdRng::seed_from_u64(seed);
            let mut controllers = Vec::new();
            controllers.push(candidate_controller(Arc::clone(&shared)));
            if duel_mode {
                controllers.push(ml_factory());
            } else {
                let player_count = configs
                    .map(|list| list[idx % list.len()].num_players)
                    .unwrap_or_else(|| local_rng.random_range(MIN_PLAYERS..=MAX_PLAYERS));
                let opponent_count = player_count.saturating_sub(1);
                controllers.extend(random_opponents(
                    factory_pool.as_ref(),
                    &mut local_rng,
                    opponent_count,
                ));
            }
            controllers.shuffle(&mut local_rng);

            let settings = configs
                .map(|list| random_map_config_from(list[idx % list.len()]))
                .unwrap_or_else(|| random_map_config(&mut local_rng));
            let result = Game::build(controllers, settings, None).run();
            usize::from(result.winner.starts_with(CANDIDATE_NAME))
        })
        .sum::<usize>();

    wins as f32 / actual_games as f32
}

fn candidate_controller(weights: Arc<NeuralWeights>) -> BotController {
    BotController::new(
        Box::new(NeuralBot::with_weights(weights, CANDIDATE_NAME.to_string())),
        CANDIDATE_NAME.to_string(),
    )
}

fn ml_factory() -> BotController {
    controller(MlBot::new(), "MlBot")
}

fn controller<B: Bot + 'static>(bot: B, label: &str) -> BotController {
    BotController::new(Box::new(bot), label.to_string())
}

fn random_opponents(
    factories: &[NamedFactory],
    rng: &mut StdRng,
    count: usize,
) -> Vec<BotController> {
    if count == 0 || factories.is_empty() {
        return Vec::new();
    }

    let actual = count.min(factories.len());
    let mut indices: Vec<usize> = (0..factories.len()).collect();
    indices.shuffle(rng);
    indices
        .into_iter()
        .take(actual)
        .map(|idx| factories[idx].build())
        .collect()
}

fn random_map_config(rng: &mut impl Rng) -> MapConfig {
    let idx = rng.random_range(0..MAP_SIZES.len());
    MapConfig {
        size: MAP_SIZES[idx],
        bomb_timer: BOMB_TIMER,
        bomb_radius: BOMB_RADIUS,
        endgame: ENDGAME_TURN,
    }
}

fn random_map_config_from(config: GameConfig) -> MapConfig {
    MapConfig {
        size: config.size,
        bomb_timer: BOMB_TIMER,
        bomb_radius: BOMB_RADIUS,
        endgame: ENDGAME_TURN,
    }
}
