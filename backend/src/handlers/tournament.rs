use std::sync::Arc;
use std::{sync::atomic::AtomicUsize, thread, time::Duration};

use axum::{
    Json, http::StatusCode, response::IntoResponse
};
use serde::Serialize;
use tournament::tournament_result::{TournamentResult};
use tournament::tournament::run_tournament;
use tournament::factories::game_config_factory::{ConfigFactory};
use bots::available_bots;


pub async fn run_tournament_handler() -> impl IntoResponse {
    let tournament_result = execute_new_tournament().await;
    match tournament_result {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(err_msg) => {
            #[derive(Serialize)]
            struct ErrorResponse {
                error: String,
            }
            let body = ErrorResponse { error: err_msg };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
        }
    }
}



pub async fn execute_new_tournament() -> Result<TournamentResult, String> {
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
    .map_err(|e| e.to_string())
}