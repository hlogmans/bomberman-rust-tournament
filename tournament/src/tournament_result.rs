use std::collections::HashMap;
use game::game::game_result::GameResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Score {
    pub wins: usize,
    pub losses: usize,
    pub total_games: usize,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TournamentResult {
    pub most_interesting: Option<GameResult>,
    pub scores: HashMap<String, Score>,
    pub total_games: usize,
}


impl TournamentResult {
    pub fn new() -> Self {
        Self { most_interesting: None, scores: HashMap::new(), total_games: 0, }
    }
    pub fn add_score(&mut self, botname: &String, score_to_add: Score) {
        self.scores
            .entry(botname.to_string())
            .and_modify(|score| {
                score.wins += score_to_add.wins;
                score.losses += score_to_add.losses;
                score.total_games += score_to_add.total_games;
            })
            .or_insert(score_to_add);
    }

    pub fn merge_with(&mut self, other: &mut TournamentResult) {
        for (botname, score) in other.scores.iter() {
            self.add_score(botname, *score);
        }

        if self.most_interesting.is_none() ||
            (other.most_interesting.is_some() && self.most_interesting.as_ref().unwrap().replay_data[0].len() < other.most_interesting.as_ref().unwrap().replay_data[0].len())
        {
            self.most_interesting = other.most_interesting.take();
        }


        self.total_games += other.total_games;
    }
}