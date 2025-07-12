use crate::{
    bot::available_bots,
    tournament::{BotScores, run_tournament},
};

mod bot;
mod coord;
mod game;
mod map;
mod shrink;
mod tournament;

fn main() {
    let bot_constructors = available_bots();
    let bot_configs = vec![
        (1, "Bot1-Easy".to_string()),
        (1, "Bot-Easy2".to_string()),
        (0, "Bot3-Random".to_string()),
        (0, "Bot4-Random".to_string()),
    ];

    // Start 4 threads, elke thread maakt zijn eigen bots aan
    let mut handles = Vec::new();

    for _ in 0..4 {
        let bot_constructors = bot_constructors.clone();
        let bot_configs = bot_configs.clone();
        let mut totals = BotScores::new();
        handles.push(std::thread::spawn(move || {
            // Maak hier pas de bot-instanties aan:
            let scores = run_tournament(&bot_constructors, &bot_configs);
            totals.merge_with(&scores);
            totals
        }));
    }
    let mut grand_totals = BotScores::new();
    for handle in handles {
        grand_totals.merge_with(&handle.join().unwrap());
    }
    //Print the final scores
    println!("Final Scores after {} games:", grand_totals.total_games);
    for (bot, score) in grand_totals.scores {
        println!("{}: {:?}", bot, score);
    }
}
