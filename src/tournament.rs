use std::collections::HashMap;
use std::time::Duration;

use crate::{
    bot::{Bot, BotConstructor},
    game,
};

use std::sync::{Arc, Mutex};
use rand::prelude::SliceRandom;

// This file contains the code for the tournament logic.
// Objectives:
// - Implement a tournament system that allows bots to compete against each other.
// - Ensure that the tournament is fair and balanced.
// - Provide a way to track the results of the tournament.
//
#[derive(Debug, Clone, Copy)]
pub struct Score {
    pub wins: usize,
    pub losses: usize,
    pub total_games: usize,
}

pub struct BotScores {
    pub scores: HashMap<String, Score>,
    pub total_games: usize,
}

impl BotScores {
    pub fn new() -> Self {
        BotScores {
            scores: HashMap::new(),
            total_games: 0,
        }
    }

    pub fn add_score(&mut self, botname: &str, score_to_add: Score) {
        self.scores
            .entry(botname.to_string())
            .and_modify(|score| {
                score.wins += score_to_add.wins;
                score.losses += score_to_add.losses;
                score.total_games += score_to_add.total_games;
            })
            .or_insert(score_to_add);
    }

    pub fn merge_with(&mut self, other: &BotScores) {
        for (botname, score) in other.scores.iter() {
            self.add_score(botname, *score);
        }
        self.total_games += other.total_games;
    }
}

pub fn run_tournament(
    bot_constructors: &[BotConstructor],
    round_counter: Option<(usize, Arc<Mutex<Vec<usize>>>)>,
) -> BotScores {
    // Implement the tournament logic here
    //
    let start_time = std::time::Instant::now();
    let time_limit = Duration::from_secs(10);
    let mut bot_scores = BotScores::new();

    let mut games_played = 0;

    loop {
        games_played += 1;

        // Update round counter if provided
        if let Some((thread_idx, ref counter)) = round_counter {
            let mut vec = counter.lock().unwrap();
            if thread_idx < vec.len() {
                vec[thread_idx] = games_played;
            }
        }

        // break if time limit is reached
        if start_time.elapsed() >= time_limit {
            break;
        }

        let game_bots = prepare_bots(bot_constructors);
        let bot_names = game_bots.iter().map(|bot| bot.name()).collect::<Vec<_>>();

        // run a game and update scores
        let scores = run_game(game_bots);
        for (bot, score) in bot_names.iter().zip(scores) {
            bot_scores.add_score(bot, score);
        }
    }

    // Print the final scores
    // println!("Final Scores after {} games:", games_played);
    // for (bot, score) in bot_scores.scores {
    //     println!("{}: {:?}", bot, score);
    // }
    bot_scores.total_games = games_played;
    bot_scores
}

pub fn prepare_bots(bot_constructors: &[BotConstructor]) -> Vec<Box<dyn Bot>> {
    let mut rand = rand::rng();
    let botcount = bot_constructors.len();

    let idx1 = rand.random_range(0..botcount);
    let mut idx2 = rand.random_range(0..botcount);
    while idx2 == idx1 {
        idx2 = rand.random_range(0..botcount);
    }
    // pick two bots at random
    let bot1 = bot_constructors[idx1]();
    let bot2 = bot_constructors[idx2]();

    vec![bot1, bot2]
}

/// Run a game between two bots
/// The bots must already be instantiated and ready to play.
pub fn run_game(bots: Vec<Box<dyn Bot>>) -> Vec<Score> {
    // Implement the game logic here
    let botnames = bots.iter().map(|bot| bot.name()).collect::<Vec<_>>();
    // Bots zijn al vers, geen clone nodig
    let gameresult = game::Game::build(11, 11, bots).run();
    // in tournament mode, only the winner is tracked, the other players get a loss
    botnames
        .iter()
        .map(|botname| Score {
            wins: if gameresult.winner == *botname { 1 } else { 0 },
            losses: if gameresult.winner == *botname { 0 } else { 1 },
            total_games: 1,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    // use crate::bot::available_bots;

    // use super::*;

    // #[test]
    // fn test_run_game() {
    //     let bot_constructors = available_bots();

    //     let bot1 = bot_constructors.get(1).unwrap()("Bot1");
    //     let bot2 = bot_constructors.get(1).unwrap()("Bot2");

    //     let bots = vec![bot1, bot2];

    //     let scores = run_game(bots.iter().collect());
    //     assert_eq!(scores.len(), 2);
    //     assert_eq!(scores[0].wins + scores[0].losses, 1);
    //     assert_eq!(scores[1].wins + scores[1].losses, 1);
    // }
}
