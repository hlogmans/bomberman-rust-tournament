use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::time::{SystemTime, UNIX_EPOCH};

use bots::{available_bots, get_bot_names};
use bots::neural_bot::{NeuralBot, NeuralWeights};
use game::bot::bot::{BotConstructor, BotController};
use game::game::game::Game;
use game::map::structs::map_config::MapConfig;
use rand::rngs::StdRng;
use rand::seq::index::sample;
use rand::{rng, Rng, RngCore, SeedableRng};
use rayon::prelude::*;

const NEURAL_IDENTIFIER: &str = "NeuralBot";
const LOG_DIR: &str = "training_logs";
const LOG_FILE: &str = "neural_best.log";
const SNAPSHOT_FILE: &str = "neural_best_latest.rs";
const TOURNAMENT_BOMB_TIMER: usize = 4;
const TOURNAMENT_BOMB_RADIUS: usize = 3;
const TOURNAMENT_ENDGAME: usize = 500;
const MAX_ATTEMPT_FACTOR: usize = 20;

fn main() {
    if let Err(err) = run() {
        eprintln!("trainer error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let config = TrainerConfig::default();
    let mut trainer = TrainerState::new(config)?;
    trainer.train()
}

#[derive(Clone, Debug)]
struct GameConfig {
    num_players: usize,
    size: usize,
}

#[derive(Clone, Debug)]
struct EvaluationResult {
    wins: usize,
    losses: usize,
    skipped: usize,
    attempts: usize,
}

impl EvaluationResult {
    fn games(&self) -> usize {
        self.wins + self.losses
    }

    fn win_rate(&self) -> f32 {
        let games = self.games();
        if games == 0 {
            0.0
        } else {
            self.wins as f32 / games as f32
        }
    }
}

struct TrainerConfig {
    population: usize,
    eval_games: usize,
    initial_sigma: f32,
    min_sigma: f32,
    sigma_decay: f32,
    improvement_threshold: f32,
    target_win_rate: f32,
    log_path: PathBuf,
    snapshot_path: PathBuf,
    configs: Arc<[GameConfig]>,
}

impl TrainerConfig {
    fn default() -> Self {
        let log_dir = PathBuf::from(LOG_DIR);
        let log_path = log_dir.join(LOG_FILE);
        let snapshot_path = log_dir.join(SNAPSHOT_FILE);
        let configs: Arc<[GameConfig]> = Arc::from(generate_tournament_configs().into_boxed_slice());

        Self {
            population: env_or("NEURAL_TRAIN_POPULATION", 16),
            eval_games: env_or("NEURAL_TRAIN_EVAL_GAMES", 64),
            initial_sigma: env_or("NEURAL_TRAIN_SIGMA", 0.08_f32),
            min_sigma: env_or("NEURAL_TRAIN_MIN_SIGMA", 0.01_f32),
            sigma_decay: env_or("NEURAL_TRAIN_SIGMA_DECAY", 0.8_f32),
            improvement_threshold: env_or("NEURAL_TRAIN_IMPROVEMENT", 0.01_f32),
            target_win_rate: env_or("NEURAL_TRAIN_TARGET", 0.90_f32),
            log_path,
            snapshot_path,
            configs,
        }
    }
}

struct TrainerState {
    config: TrainerConfig,
    running: Arc<AtomicBool>,
    rng: StdRng,
    best_weights: Arc<NeuralWeights>,
    best_evaluation: EvaluationResult,
    current_sigma: f32,
    stagnation_rounds: usize,
    generation: usize,
}

impl TrainerState {
    fn new(config: TrainerConfig) -> Result<Self, Box<dyn Error>> {
        if let Some(parent) = config.log_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let snapshot_parent = config.snapshot_path.parent().unwrap_or(Path::new("."));
        fs::create_dir_all(snapshot_parent)?;

        let running = Arc::new(AtomicBool::new(true));
        let flag = running.clone();
        ctrlc::set_handler(move || {
            flag.store(false, Ordering::SeqCst);
        })?;

        let mut seed_source = rng();
        let seed: u64 = seed_source.random();
        let mut rng = StdRng::seed_from_u64(seed);
        let best_weights = Arc::new(NeuralWeights::default());
        let best_seed = rng.next_u64();
        let best_evaluation = evaluate(best_weights.clone(), &config, best_seed);
        persist_best(&config, 0, config.initial_sigma, &best_evaluation, best_weights.as_ref())?;

        println!(
            "Initial win rate {:.2}% over {} games ({} skipped)",
            best_evaluation.win_rate() * 100.0,
            best_evaluation.games(),
            best_evaluation.skipped,
        );

        let current_sigma = config.initial_sigma;

        Ok(Self {
            config,
            running,
            rng,
            best_weights,
            best_evaluation,
            current_sigma,
            stagnation_rounds: 0,
            generation: 0,
        })
    }

    fn train(&mut self) -> Result<(), Box<dyn Error>> {
        println!(
            "Starting training: population={} eval_games={} target={:.1}%",
            self.config.population,
            self.config.eval_games,
            self.config.target_win_rate * 100.0,
        );

        while self.running.load(Ordering::SeqCst) {
            if self.best_evaluation.win_rate() >= self.config.target_win_rate {
                println!(
                    "Target reached: {:.2}% win rate.",
                    self.best_evaluation.win_rate() * 100.0,
                );
                break;
            }

            self.generation += 1;
            let evaluation_seed = self.rng.next_u64();
            let perturb_seed = self.rng.next_u64();
            let base_weights = self.best_weights.clone();
            let sigma = self.current_sigma;

            let base_evaluation = evaluate(base_weights.clone(), &self.config, evaluation_seed);
            let mut improved = false;
            let mut best_rate = base_evaluation.win_rate();
            let mut generation_peak = best_rate;
            let mut peak_evaluation: Option<EvaluationResult> = Some(base_evaluation.clone());

            self.best_evaluation = base_evaluation;

            let candidates: Vec<_> = (0..self.config.population)
                .into_par_iter()
                .map(|idx| {
                    let perturb_seed = perturb_seed.wrapping_add(idx as u64 + 1);
                    let mut perturb_rng = StdRng::seed_from_u64(perturb_seed);
                    let candidate_weights = Arc::new(base_weights.as_ref().perturb(&mut perturb_rng, sigma));
                    let evaluation = evaluate(candidate_weights.clone(), &self.config, evaluation_seed);
                    (candidate_weights, evaluation)
                })
                .collect();

            for (candidate_weights, evaluation) in candidates.into_iter() {
                let candidate_rate = evaluation.win_rate();
                if candidate_rate > generation_peak {
                    generation_peak = candidate_rate;
                    peak_evaluation = Some(evaluation.clone());
                }
                if candidate_rate > best_rate + self.config.improvement_threshold {
                    self.best_weights = candidate_weights;
                    self.best_evaluation = evaluation;
                    best_rate = self.best_evaluation.win_rate();
                    self.stagnation_rounds = 0;
                    improved = true;
                    persist_best(&self.config, self.generation, sigma, &self.best_evaluation, self.best_weights.as_ref())?;
                    println!(
                        "Generation {}: new best {:.2}% (wins {} / games {}) with sigma {:.4}",
                        self.generation,
                        best_rate * 100.0,
                        self.best_evaluation.wins,
                        self.best_evaluation.games(),
                        sigma,
                    );
                }
            }

            if !improved {
                if let Some(peak) = peak_evaluation {
                    println!(
                        "Generation {}: best candidate {:.2}% (wins {} / games {}) with sigma {:.4}",
                        self.generation,
                        generation_peak * 100.0,
                        peak.wins,
                        peak.games(),
                        self.current_sigma,
                    );
                }

                self.stagnation_rounds += 1;
                if self.stagnation_rounds >= 3 {
                    self.current_sigma = (self.current_sigma * self.config.sigma_decay).max(self.config.min_sigma);
                    self.stagnation_rounds = 0;
                    println!(
                        "Generation {}: no improvement, adjusting sigma to {:.4}",
                        self.generation,
                        self.current_sigma,
                    );
                }
            }
        }

        if !self.running.load(Ordering::SeqCst) {
            println!(
                "Training interrupted. Best {:.2}% over {} games.",
                self.best_evaluation.win_rate() * 100.0,
                self.best_evaluation.games(),
            );
        } else {
            println!(
                "Training finished. Best {:.2}% over {} games.",
                self.best_evaluation.win_rate() * 100.0,
                self.best_evaluation.games(),
            );
        }

        Ok(())
    }
}

fn evaluate(weights: Arc<NeuralWeights>, config: &TrainerConfig, seed: u64) -> EvaluationResult {
    let (constructors, neural_index) = build_training_constructors(weights);
    let mut rng = StdRng::seed_from_u64(seed);
    let mut config_iter = config.configs.iter().cycle();

    let mut wins = 0;
    let mut losses = 0;
    let skipped = 0;
    let mut attempts = 0;
    let max_attempts = config.eval_games.max(1) * MAX_ATTEMPT_FACTOR;

    while wins + losses < config.eval_games && attempts < max_attempts {
        if let Some(cfg) = config_iter.next() {
            attempts += 1;
            let participants = select_participants(&constructors, neural_index, cfg.num_players, &mut rng);

            let result = run_game(participants, cfg.size);
            if result.winner.starts_with(NEURAL_IDENTIFIER) {
                wins += 1;
            } else {
                losses += 1;
            }
        }
    }

    EvaluationResult {
        wins,
        losses,
        skipped,
        attempts,
    }
}

fn build_training_constructors(weights: Arc<NeuralWeights>) -> (Vec<BotConstructor>, usize) {
    let names = get_bot_names();
    let constructors = available_bots();

    let mut wrapped = Vec::with_capacity(constructors.len());
    let mut neural_index = None;

    for (idx, (constructor, name)) in constructors.into_iter().zip(names.into_iter()).enumerate() {
        if name == NEURAL_IDENTIFIER {
            let weights = weights.clone();
            let label = name.clone();
            neural_index = Some(idx);
            wrapped.push(Box::new(move || {
                let bot = NeuralBot::with_weights(weights.clone(), label.clone());
                BotController::new(Box::new(bot), label.clone())
            }) as BotConstructor);
        } else {
            wrapped.push(constructor);
        }
    }

    let neural_index = neural_index.expect("NeuralBot missing from registry");
    (wrapped, neural_index)
}

fn select_participants(
    constructors: &[BotConstructor],
    neural_index: usize,
    player_count: usize,
    rng: &mut StdRng,
) -> Vec<BotController> {
    let indices = sample(rng, constructors.len(), player_count);
    let mut picked = indices.into_vec();

    if !picked.contains(&neural_index) {
        let replacement = rng.random_range(0..player_count);
        picked[replacement] = neural_index;
    }

    picked
        .into_iter()
        .map(|idx| constructors[idx]())
        .collect()
}

fn run_game(bots: Vec<BotController>, size: usize) -> game::game::game_result::GameResult {
    let settings = MapConfig {
        size,
        bomb_timer: TOURNAMENT_BOMB_TIMER,
        bomb_radius: TOURNAMENT_BOMB_RADIUS,
        endgame: TOURNAMENT_ENDGAME,
    };
    // Mirror the tournament runner settings so the training environment matches live play.
    Game::build(bots, settings, None).run()
}

fn persist_best(
    config: &TrainerConfig,
    generation: usize,
    sigma: f32,
    evaluation: &EvaluationResult,
    weights: &NeuralWeights,
) -> std::io::Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let weights_str = weights.format_as_rust();
    let win_rate = evaluation.win_rate();

    let mut log = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&config.log_path)?;
    writeln!(
        log,
        "ts={timestamp} gen={generation} sigma={:.5} wins={} losses={} skipped={} attempts={} win_rate={:.6}",
        sigma,
        evaluation.wins,
        evaluation.losses,
        evaluation.skipped,
        evaluation.attempts,
        win_rate,
    )?;
    writeln!(log, "{}", weights_str.trim_end())?;
    writeln!(log, "-----")?;
    log.flush()?;

    let mut snapshot = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&config.snapshot_path)?;
    snapshot.write_all(weights_str.as_bytes())?;
    snapshot.flush()?;

    Ok(())
}

fn generate_tournament_configs() -> Vec<GameConfig> {
    let player_counts = [2, 3, 4];
    let map_sizes = odd_numbers_in_range(7, 20);

    let mut configs = Vec::new();
    for size in map_sizes {
        for &players in &player_counts {
            configs.push(GameConfig {
                num_players: players,
                size,
            });
        }
    }

    configs
}

fn odd_numbers_in_range(start: usize, end: usize) -> Vec<usize> {
    (start..=end).filter(|value| value % 2 == 1).collect()
}

fn env_or<T>(key: &str, default: T) -> T
where
    T: std::str::FromStr,
{
    match std::env::var(key) {
        Ok(value) => value.parse().unwrap_or(default),
        Err(_) => default,
    }
}
