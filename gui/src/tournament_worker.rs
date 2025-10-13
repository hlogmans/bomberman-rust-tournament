use gloo_worker::oneshot::oneshot;
use runner::tournament_result::{TournamentResult};
use runner::tournament::run_tournament_wasm;
use runner::factories::game_config_factory::{ConfigFactory, GameConfig};
use game::bot::bot::BotConstructor;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use bots::available_bots;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TournamentInput {
    pub duration: f64,
}


#[oneshot]
pub async fn TournamentWorker(input: TournamentInput) -> TournamentResult {
    let bot_constructors = available_bots();

    let config =  GameConfig {num_players:2, size:11};

    run_tournament_wasm(&bot_constructors, Some(Arc::new(AtomicUsize::new(0))), input.duration, ConfigFactory::generate_tournament_configs() ) //[config].to_vec()
}