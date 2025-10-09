use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use rand::prelude::SliceRandom;
use game::bot::bot::{Bot, BotConstructor};
use game::game::game::Game;
use game::game::game_result::GameResult;
use game::game::replay_engine::{GameReplaySnapshot, ReplayEngine};
use crate::tournament_result::{Score, TournamentResult};
use js_sys::Date;

pub fn run_tournament(bot_constructors: &[BotConstructor], round_counter: Option<Arc<AtomicUsize>>, duration: Duration) -> TournamentResult {
    let mut tournament_result = TournamentResult::new();
    let start = Instant::now();

    while start.elapsed() < duration {
        run_tournament_game(&mut tournament_result, bot_constructors, &round_counter);
    }

    tournament_result
}

pub fn run_tournament_wasm(bot_constructors: &[BotConstructor], round_counter: Option<Arc<AtomicUsize>>, duration_ms: f64) -> TournamentResult {
    let mut tournament_result = TournamentResult::new();
    let start = Date::now();

    while Date::now() - start < duration_ms {
        run_tournament_game(&mut tournament_result, bot_constructors, &round_counter);
    }

    tournament_result
}

pub fn run_tournament_game(tournament_result: &mut TournamentResult, bot_constructors: &[BotConstructor], round_counter: &Option<Arc<AtomicUsize>>) {
    let game_bots = prepare_bots(bot_constructors);
    let names: Vec<String> = game_bots.iter().map(|b| b.name()).collect();

    let game_result = run_game(game_bots); // run_game takes ownership
    let scores_vec = update_scores(&game_result, &names);

    if tournament_result.most_interesting.is_none() || game_result.replay_data[0].len() > tournament_result.most_interesting.as_ref().unwrap().replay_data[0].len() {
        tournament_result.most_interesting = Some(game_result);
    }

    for (name, score) in names.clone().iter().zip(scores_vec.iter()) {
        tournament_result.add_score(name, *score);
    }

    if let Some(counter) = &round_counter {
        counter.fetch_add(1, Ordering::Relaxed);
    }

    tournament_result.total_games += 1;
}

pub fn prepare_bots(bot_constructors: &[BotConstructor]) -> Vec<Box<dyn Bot>> {
    let botcount = bot_constructors.len();
    let mut rng = rand::thread_rng();

    let mut indices: Vec<usize> = (0..botcount).collect();
    indices.shuffle(&mut rng);
    let idx1 = indices[0];
    let idx2 = indices[1];

    // pick two bots at random
    let bot1 = bot_constructors[idx1]();
    let bot2 = bot_constructors[idx2]();

    vec![bot1, bot2]
}

pub fn run_game(bots: Vec<Box<dyn Bot>>) -> GameResult {
    Game::build(Some(11), Some(11), bots, None).run()
}

pub fn replay(game_result: &GameResult) -> GameReplaySnapshot {
    let mut game = Game::build(None, None, Vec::new(), Some(game_result.game_settings.clone()));
    let mut replay_engine = ReplayEngine::new(&mut game);

    return replay_engine.to_snapshot(&game_result.replay_data);
}

pub fn update_scores(game_result: &GameResult, bot_names: &[String]) -> Vec<Score> {
    bot_names.iter().map(|name| Score {
        wins: if game_result.winner == *name { 1 } else { 0 },
        losses: if game_result.winner == *name { 0 } else { 1 },
        total_games: 1,
    }).collect()
}
