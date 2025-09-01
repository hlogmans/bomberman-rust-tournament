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

fn main() {
    use num_cpus;

    // Dynamisch aantal threads op basis van CPU cores
    let num_threads = num_cpus::get();
    println!("Aantal nuttige threads: {num_threads}");

    let start_time = std::time::Instant::now();

    // Shared round counters for each thread
    let round_counters = Arc::new(Mutex::new(vec![0; num_threads]));

    // Status thread: print every 250ms
    let status_counters = round_counters.clone();
    let status_handle = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(250));
            let counters = status_counters.lock().unwrap();
            //let line = String::from("Thread rounds: ");
            let mut total = 0;
            for count in counters.iter() {
                // Pad to 6 digits for alignment
                //line.push_str(&format!("T{}:{:6} ", i, count));
                if *count != usize::MAX {
                    total += count;
                }
            }
            let speed = total as f64 / start_time.elapsed().as_secs_f64() / 1000.0;
            // Carriage return + flush to overwrite line
            print!("Total: {total}, Speed: {speed:.1}K rounds/s\r");
            use std::io::{Write, stdout};
            stdout().flush().unwrap();

            // Stop condition: if all threads are done (negative value as marker)
            if counters.iter().all(|&c| c == usize::MAX) {
                break;
            }
        }
        println!(); // Move to next line after finishing
    });

    // Start threads, elke thread maakt zijn eigen bots aan
    let mut handles = Vec::new();

    for thread_idx in 0..num_threads {
        let mut totals = BotScores::new();
        let round_counters = round_counters.clone();
        handles.push(thread::spawn(move || {
            // Geef thread index en Arc door
            let bot_constructors = available_bots();
            let scores = run_tournament(&bot_constructors, Some((thread_idx, round_counters)));
            totals.merge_with(&scores);
            totals
        }));
    }
    let mut grand_totals = BotScores::new();
    for handle in handles {
        grand_totals.merge_with(&handle.join().unwrap());
    }

    // Mark all threads as done for status thread
    {
        let mut counters = round_counters.lock().unwrap();
        for c in counters.iter_mut() {
            *c = usize::MAX;
        }
    }
    status_handle.join().unwrap();

    //Print the final scores
    /*println!("Final Scores after {} games:", grand_totals.total_games);
    for (bot, score) in grand_totals.scores {
        println!("{}: {:?}", bot, score);
    }*/

    // Sort scores by number of wins in descending order
    let mut sorted_scores = grand_totals.scores.clone();
    // sort_by on the percentage of wins, compared as float
    // null exception possible
    sorted_scores.sort_by(|a, b| {
        (b.1.wins as f64 / b.1.total_games as f64)
            .total_cmp(&(a.1.wins as f64 / a.1.total_games as f64))
    });

    //Print the final scores
    println!("Final Scores after {} games:", grand_totals.total_games);
    for (bot, score) in sorted_scores {
        println!(
            "{bot}: WinPercentage: {:.1}% {score:?}",
            (score.wins as f64 / score.total_games as f64) * 100.0
        );
    }
}
