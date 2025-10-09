use std::sync::{
    Arc,
    atomic::{AtomicUsize, AtomicBool, Ordering},
};
use std::thread;
use std::time::{Duration, Instant};

use bots::available_bots;
use runner::tournament::run_tournament;
use runner::tournament_result::TournamentResult;

fn main() {
    let num_threads = num_cpus::get();
    println!("Running on {num_threads} threads");

    let duration = Duration::from_secs(10);
    let start_time = Instant::now();

    // Per-thread counters
    let round_counters: Vec<_> = (0..num_threads)
        .map(|_| Arc::new(AtomicUsize::new(0)))
        .collect();

    // Flag for status thread
    let done = Arc::new(AtomicBool::new(false));

    // Spawn status thread
    {
        let counters = round_counters.clone();
        let done = done.clone();
        thread::spawn(move || {
            use std::io::{stdout, Write};
            loop {
                thread::sleep(Duration::from_millis(250));
                let total: usize = counters.iter().map(|c| c.load(Ordering::Relaxed)).sum();
                let speed = total as f64 / start_time.elapsed().as_secs_f64();
                print!("Total games: {total}, Speed: {speed:.0} games/s\r");
                stdout().flush().unwrap();

                if done.load(Ordering::Relaxed) {
                    break;
                }
            }
            println!();
        })
    };

    // Tournament threads
    let handles: Vec<_> = round_counters
        .into_iter()
        .map(|counter| {
            let bot_constructors = available_bots();
            thread::spawn(move || run_tournament(&bot_constructors, Some(counter), duration))
        })
        .collect();

    // Merge results
    let mut grand_totals = TournamentResult::new();
    for handle in handles {
        grand_totals.merge_with(&mut handle.join().unwrap());
    }

    done.store(true, Ordering::Relaxed);

    // Sort by win percentage
    let mut sorted_scores: Vec<_> = grand_totals.scores.iter().collect();
    sorted_scores.sort_by(|a, b| {
        (b.1.wins as f64 / b.1.total_games as f64)
            .total_cmp(&(a.1.wins as f64 / a.1.total_games as f64))
    });

    println!("Final Scores after {} games:", grand_totals.total_games);
    for (bot, score) in sorted_scores {
        println!(
            "{bot}: WinPercentage: {:.1}% {score:?}",
            (score.wins as f64 / score.total_games as f64) * 100.0
        );
    }

    if let Some(ref result) = grand_totals.most_interesting {
        println!("\nMost interesting replay:");
        //println!("{}", runner::tournament::replay(result));
    } else {
        println!("No interesting game recorded.");
    }
}
