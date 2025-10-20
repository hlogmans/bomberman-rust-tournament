use leptos::*;
use leptos::prelude::*;
use runner::tournament_result::{TournamentResult};
#[allow(unused_imports)]
use runner::tournament::run_tournament;
#[allow(unused_imports)]
use runner::factories::game_config_factory::{ConfigFactory};
#[allow(unused_imports)]
use std::sync::atomic::AtomicUsize;
#[allow(unused_imports)]
use std::sync::Arc;
#[allow(unused_imports)]
use bots::available_bots;
#[allow(unused_imports)]
use std::time::Duration;
#[allow(unused_imports)]
use std::thread;
#[allow(unused_imports)]
use std::time::Instant;

#[server]
pub async fn execute_new_tournament() -> Result<TournamentResult, ServerFnError> {
    tokio::task::spawn_blocking(move || {
        let num_threads = num_cpus::get();
        let duration = Duration::from_secs(10);
        let round_counters: Vec<_> = (0..num_threads)
            .map(|_| Arc::new(AtomicUsize::new(0)))
            .collect();

        let handles: Vec<_> = round_counters
            .into_iter()
            .map(|counter| {
                let bot_constructors = available_bots();
                thread::spawn(move || {
                    run_tournament(&bot_constructors, Some(counter), duration, ConfigFactory::generate_tournament_configs())
                })
            })
            .collect();

        let mut grand_totals = TournamentResult::new();
        for handle in handles {
            grand_totals.merge_with(&mut handle.join().unwrap());
        }

        grand_totals
    })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}
   