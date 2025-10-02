use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use rand::Rng;
use game::bot::bot::{Bot, BotConstructor};
use game::game::game::Game;

#[derive(Debug, Clone, Copy)]
pub struct Score {
    pub wins: usize,
    pub losses: usize,
    pub total_games: usize,
}

pub struct BotScores {
    pub scores: Vec<(String, Score)>,
    pub total_games: usize,
}

impl BotScores {
    pub fn new() -> Self {
        BotScores {
            scores: Vec::new(),
            total_games: 0,
        }
    }

    pub fn add_score(&mut self, botname: String, score_to_add: Score) {
        // check if bot already exists in scores
        if let Some((_, score)) = self.scores.iter_mut().find(|(name, _)| name == &botname) {
            score.wins += score_to_add.wins;
            score.losses += score_to_add.losses;
            score.total_games += score_to_add.total_games;
        } else {
            self.scores.push((botname, score_to_add));
        }
    }

    pub fn merge_with(&mut self, other: &BotScores) {
        for (botname, score) in other.scores.iter() {
            self.add_score(botname.clone(), *score);
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
    let botcount = bot_constructors.len();
    let mut rng = rand::thread_rng();
    let start = Instant::now();

    while start.elapsed() < duration {
        // Pick two distinct bots
        let idx1 = rng.gen_range(0..botcount);
        let mut idx2 = rng.gen_range(0..botcount);
        while idx2 == idx1 {
            idx2 = rng.gen_range(0..botcount);
        }

        let bots: Vec<Box<dyn Bot>> = vec![bot_constructors[idx1](), bot_constructors[idx2]()];
        let names: Vec<String> = bots.iter().map(|b| b.name()).collect();

        let scores_vec: Vec<Score> = run_game(bots);

        for (name, score) in names.iter().zip(scores_vec.iter()) {
            bot_scores.add_score(name.clone(), *score);
        }

        if let Some(counter) = &round_counter {
            counter.fetch_add(1, Ordering::Relaxed);
        }

        bot_scores.total_games += 1;
    }

    bot_scores
}

fn run_game(bots: Vec<Box<dyn Bot>>) -> Vec<Score> {
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