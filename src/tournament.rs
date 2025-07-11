use std::time::Duration;

use rand::Rng;

use crate::{
    bot::{Bot, clone_bot},
    game,
};

// This file contains the code for the tournament logic.
// Objectives:
// - Implement a tournament system that allows bots to compete against each other.
// - Ensure that the tournament is fair and balanced.
// - Provide a way to track the results of the tournament.
//
#[derive(Debug)]
pub struct Score {
    pub wins: usize,
    pub losses: usize,
    pub total_games: usize,
}

pub struct BotScores {
    pub scores: Vec<(String, Score)>,
}

impl BotScores {
    pub fn new() -> Self {
        BotScores { scores: Vec::new() }
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
}

pub struct Tournament {
    bots: Vec<Box<dyn Bot>>,
    time_limit: Duration,
}

pub fn run_tournament(bots: &Vec<Box<dyn Bot>>) {
    // Implement the tournament logic here
    //
    let start_time = std::time::Instant::now();
    let time_limit = Duration::from_secs(10);

    let mut rand = rand::rng();
    let botcount = bots.len();

    let mut bot_scores = BotScores::new();

    let mut games_played = 0;

    loop {
        games_played += 1;

        // break if time limit is reached
        if start_time.elapsed() >= time_limit {
            break;
        }

        // pick two bots at random
        let bot1 = bots.get(rand.random_range(..botcount));
        let bot2 = bots.get(rand.random_range(..botcount));

        let game_bots: Vec<&Box<dyn Bot>> = vec![bot1.unwrap(), bot2.unwrap()];

        let bot_names = game_bots.iter().map(|bot| bot.name()).collect::<Vec<_>>();
        // run a game and update scores
        let scores = run_game(game_bots);
        for (bot, score) in bot_names.iter().zip(scores) {
            bot_scores.add_score(bot.clone(), score);
        }
    }

    // Print the final scores
    println!("Final Scores after {} games:", games_played);
    for (bot, score) in bot_scores.scores {
        println!("{}: {:?}", bot, score);
    }
}

fn run_game(bots: Vec<&Box<dyn Bot>>) -> Vec<Score> {
    // Implement the game logic here
    let botnames = bots.iter().map(|bot| bot.name()).collect::<Vec<_>>();
    let cloned_bots = bots.iter().map(|bot| clone_bot(bot.as_ref())).collect();
    let gameresult = game::Game::build(11, 11, cloned_bots).run();
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
    use crate::bot::available_bots;

    use super::*;

    #[test]
    fn test_run_game() {
        let bot_constructors = available_bots();

        let bot1 = bot_constructors.get(1).unwrap()("Bot1");
        let bot2 = bot_constructors.get(1).unwrap()("Bot2");

        let bots = vec![bot1, bot2];

        let scores = run_game(bots.iter().collect());
        assert_eq!(scores.len(), 2);
        assert_eq!(scores[0].wins + scores[0].losses, 1);
        assert_eq!(scores[1].wins + scores[1].losses, 1);
    }
}
