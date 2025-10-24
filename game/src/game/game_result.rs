use serde::{Deserialize, Serialize};
use crate::bot::bot_data::BotData;
use crate::map::structs::map_config::MapConfig;
use crate::game::game::Game;
use crate::map::enums::command::Command;

/// Represents the result of a game.
#[derive(Clone, Serialize, Deserialize)]
pub struct GameResult {
    pub winner: String,
    pub replay_data: Vec<Vec<Command>>,
    pub debug_data: Vec<Vec<String>>,
    pub game_settings: MapConfig,
    pub rounds: usize,
    pub bots: Vec<BotData>,
}

impl GameResult {
    pub fn build(game: &Game) -> Self {
        let winner = game.winner_name();
        let game_settings = game.map.map_settings.clone();
        let bot_data = game.map.players.iter().map(|player| BotData {id: player.id, name: player.name.clone()}).collect();

        GameResult {
            winner,
            replay_data: game.player_actions.clone(),
            debug_data: game.debug_info.clone(),
            game_settings,
            rounds: game.turn,
            bots: bot_data
        }
    }
}
