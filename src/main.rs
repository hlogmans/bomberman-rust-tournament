#![allow(dead_code)]
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

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::bot::Bot;

fn main() {
    use num_cpus;
    let bot_constructors = available_bots();
    let bot_configs = vec![
        (1, "Easy".to_string()),
        (2, "MartijnBot".to_string()),
    ];

    let bot1 = bot_constructors[bot_configs[0].0](&bot_configs[0].1);
    let bot2 = bot_constructors[bot_configs[1].0](&bot_configs[1].1);

    let game_bots: Vec<Box<dyn Bot>> = vec![bot1, bot2];
    let gameresult = game::Game::build(11, 11, game_bots).run();

    println!("{}", gameresult.winner)



    //
    //
    // // Dynamisch aantal threads op basis van CPU cores
    // let num_threads = num_cpus::get();
    // println!("Aantal nuttige threads: {}", num_threads);
    //
    // let start_time = std::time::Instant::now();
    //
    // // Shared round counters for each thread
    // let round_counters = Arc::new(Mutex::new(vec![0; num_threads]));
    //
    // // Status thread: print every 250ms
    // let status_counters = round_counters.clone();
    // let status_handle = thread::spawn(move || {
    //     loop {
    //         thread::sleep(Duration::from_millis(250));
    //         let counters = status_counters.lock().unwrap();
    //         //let line = String::from("Thread rounds: ");
    //         let mut total = 0;
    //         for (_i, count) in counters.iter().enumerate() {
    //             // Pad to 6 digits for alignment
    //             //line.push_str(&format!("T{}:{:6} ", i, count));
    //             if *count != usize::MAX {
    //                 total += count;
    //             }
    //         }
    //         let speed = total as f64 / start_time.elapsed().as_secs_f64() / 1000.0;
    //         // Carriage return + flush to overwrite line
    //         print!("Total: {}, Speed: {:.1}K rounds/s\r", total, speed);
    //         use std::io::{Write, stdout};
    //         stdout().flush().unwrap();
    //         // Stop condition: if all threads are done (negative value as marker)
    //         if counters.iter().all(|&c| c == usize::MAX) {
    //             break;
    //         }
    //     }
    //     println!(); // Move to next line after finishing
    // });
    //
    // // Start threads, elke thread maakt zijn eigen bots aan
    // let mut handles = Vec::new();
    //
    // for thread_idx in 0..num_threads {
    //     let bot_constructors = bot_constructors.clone();
    //     let bot_configs = bot_configs.clone();
    //     let mut totals = BotScores::new();
    //     let round_counters = round_counters.clone();
    //     handles.push(thread::spawn(move || {
    //         // Geef thread index en Arc door
    //         let scores = run_tournament(
    //             &bot_constructors,
    //             &bot_configs,
    //             Some((thread_idx, round_counters)),
    //         );
    //         totals.merge_with(&scores);
    //         totals
    //     }));
    // }
    // let mut grand_totals = BotScores::new();
    // for handle in handles {
    //     grand_totals.merge_with(&handle.join().unwrap());
    // }
    //
    // // Mark all threads as done for status thread
    // {
    //     let mut counters = round_counters.lock().unwrap();
    //     for c in counters.iter_mut() {
    //         *c = usize::MAX;
    //     }
    // }
    // status_handle.join().unwrap();
    //
    // //Print the final scores
    // println!("Final Scores after {} games:", grand_totals.total_games);
    // for (bot, score) in grand_totals.scores {
    //     println!("{}: {:?}", bot, score);
    // }
}
