use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use rand::prelude::SliceRandom;
use game::bot::bot::{Bot, BotConstructor};
use game::game::game::Game;

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
    round_counter: Option<Arc<AtomicUsize>>,
    duration: Duration,
) -> BotScores {
    let mut bot_scores = BotScores::new();
    let start = Instant::now();

    while start.elapsed() < duration {
        let game_bots = prepare_bots(bot_constructors);
        let names: Vec<String> = game_bots.iter().map(|b| b.name()).collect();

        let scores_vec: Vec<Score> = run_game(game_bots);

        for (name, score) in names.iter().zip(scores_vec.iter()) {
            bot_scores.add_score(name, *score);
        }

        if let Some(counter) = &round_counter {
            counter.fetch_add(1, Ordering::Relaxed);
        }

        bot_scores.total_games += 1;
    }

    bot_scores
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

pub fn run_game(bots: Vec<Box<dyn Bot>>) -> Vec<Score> {
    let bot_names = bots.iter().map(|b| b.name()).collect::<Vec<_>>();
    let gameresult = Game::build(11, 11, bots).run();

    bot_names
        .iter()
        .map(|name| Score {
            wins: if gameresult.winner == *name { 1 } else { 0 },
            losses: if gameresult.winner == *name { 0 } else { 1 },
            total_games: 1,
        })
        .collect()
}