use crate::map::structs::map_config::MapConfig;
use crate::game::game::Game;
use crate::map::enums::command::Command;

/// Represents the result of a game.
pub struct GameResult {
    pub winner: String,
    pub replay_data: Vec<Command>,
    pub game_settings: MapConfig,
    pub rounds: usize,
}

impl GameResult {
    pub fn build(game: &Game) -> Self {
        let winner = game.winner_name().unwrap_or_default();
        let replay_data = game.player_actions.clone();
        let game_settings = game.map.map_settings.clone();

        GameResult {
            winner,
            replay_data: replay_data.iter().map(|command| command.1).collect(),
            game_settings,
            rounds: game.turn,
        }
    }
}
