use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use rand::seq::index::sample;
use rand::{thread_rng};
use game::bot::bot::{Bot, BotConstructor};
use game::game::game::Game;
use game::game::game_result::GameResult;
use game::game::replay_engine::{GameReplaySnapshot, ReplayEngine};
use crate::factories::game_config_factory::{GameConfig};
use crate::tournament_result::{Score, TournamentResult};


/// Runs a tournament for a given duration with the specified number of players per game.
pub fn run_tournament(bot_constructors: &[BotConstructor], round_counter: Option<Arc<AtomicUsize>>, duration: Duration, game_config: Vec<GameConfig>) -> TournamentResult {
    let mut tournament_result = TournamentResult::new();
    let start = Instant::now();
    let mut config_iter = game_config.iter().cycle();

    while start.elapsed() < duration {
        let config = config_iter.next().unwrap();

        let game_bots = prepare_bots(bot_constructors, config.num_players);

        // Collect names as Strings (we own them)
        let names: Vec<String> = game_bots.iter().map(|b| b.name()).collect();

        let game_result = run_game(game_bots, config.size);
        let scores_vec = update_scores(&game_result, &names);

        if tournament_result.most_interesting.is_none() || game_result.replay_data[0].len() > tournament_result.most_interesting.as_ref().unwrap().replay_data[0].len() {
            tournament_result.most_interesting = Some(game_result);
        }

        for (name, score) in names.iter().zip(scores_vec.iter()) {
            tournament_result.add_score(name, *score);
        }

        if let Some(counter) = &round_counter {
            counter.fetch_add(1, Ordering::Relaxed);
        }

        tournament_result.total_games += 1;
    }

    tournament_result
}

pub fn prepare_bots(bot_constructors: &[BotConstructor], player_count: usize) -> Vec<Box<dyn Bot>> {
    let mut rng = thread_rng();
    let indices = sample(&mut rng, bot_constructors.len(), player_count);
    indices.iter().map(|i| bot_constructors[i]()).collect()
}

/// Runs a single game with the given bots
pub fn run_game(bots: Vec<Box<dyn Bot>>, size: usize) -> GameResult {
    Game::build(Some(size), Some(size), bots, None).run()
}

/// Generates a replay snapshot from a game result
pub fn replay(game_result: &GameResult) -> GameReplaySnapshot {
    let mut game = Game::build(None, None, Vec::new(), Some(game_result.game_settings.clone()));
    let mut replay_engine = ReplayEngine::new(&mut game);
    replay_engine.to_snapshot(&game_result.replay_data)
}

/// Updates scores based on the game result
pub fn update_scores(game_result: &GameResult, bot_names: &[String]) -> Vec<Score> {
    // Borrow the winner directly if it's a String
    let winner_name: &String = &game_result.winner;

    bot_names
        .iter()
        .map(|name| {
            let is_winner = winner_name == name.as_str();
            Score {
                wins: if is_winner { 1 } else { 0 },
                losses: if is_winner { 0 } else { 1 },
                total_games: 1,
            }
        })
        .collect()
}

