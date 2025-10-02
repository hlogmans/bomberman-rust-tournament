use std::sync::{Arc, atomic::{AtomicUsize, Ordering}, atomic::AtomicBool};
use std::thread;
use std::time::{Duration, Instant};

use bots::available_bots;
use runner::tournament::{BotScores, run_tournament};


fn main() {
    let num_threads = num_cpus::get();
    println!("Running on {num_threads} threads");

    let duration = Duration::from_secs(10); // tournament duration
    let start_time = Instant::now();

    // Per-thread counters
    let round_counters: Vec<Arc<AtomicUsize>> = (0..num_threads)
        .map(|_| Arc::new(AtomicUsize::new(0)))
        .collect();
    let counters_for_status = round_counters.clone();

    // Flag to tell status thread when threads are done
    let done = Arc::new(AtomicBool::new(false));
    let done_for_status = done.clone();

    // Status thread
    let status_handle = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(250));
            let total: usize = counters_for_status
                .iter()
                .map(|c| c.load(Ordering::Relaxed))
                .sum();

            let speed = total as f64 / start_time.elapsed().as_secs_f64();
            print!("Total games: {total}, Speed: {speed:.0} games/s\r");
            use std::io::{Write, stdout};
            stdout().flush().unwrap();

            if done_for_status.load(Ordering::Relaxed) {
                break;
            }
        }
        println!();
    });

    // Tournament threads
    let mut handles = Vec::new();
    for counter in round_counters.iter().take(num_threads){
        let bot_constructors = available_bots();
        let round_counter = counter.clone();

        handles.push(thread::spawn(move || {
            run_tournament(&bot_constructors, Some(round_counter), duration)
        }));
    }

    // Merge results
    let mut grand_totals = BotScores::new();
    for handle in handles {
        grand_totals.merge_with(&handle.join().unwrap());
    }

    done.store(true, Ordering::Relaxed);
    status_handle.join().unwrap();

    // Sort by win percentage
    let mut sorted_scores = grand_totals.scores.clone();
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
}
