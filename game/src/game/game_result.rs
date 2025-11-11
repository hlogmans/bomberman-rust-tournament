use std::collections::HashMap;

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
    pub score: usize,
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
            score: GameResult::calculate_score(game),
            bots: bot_data
        }
    }

    fn calculate_score(game: &Game) -> usize {
        let mut score: usize = game.max_turn - game.turn;
        let mut killers: HashMap<usize, usize> = HashMap::new();

        for player in &game.map.players {
            score += game.map.map_settings.size * match player.reason_killed.as_str() {
                "suicide" => 10,
                "bomb" => {
                    let mut bomb_score = 15;
                    if let Some(kill_count) = killers.get_mut(&player.killed_by) {
                        *kill_count += 1;
                        bomb_score += 5 * *kill_count;
                    } else {
                        killers.insert(player.killed_by, 0);
                    }
                    bomb_score
                }
                "shrink" => 3,
                _ => 0,
            };
        }

        score
    }
}
