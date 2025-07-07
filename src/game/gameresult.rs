use crate::game::Game;
use crate::game::map_settings::MapSettings;
use crate::map::Command;

pub struct GameResult {
    pub winner: String,
    pub replay_data: Vec<Command>,
    pub game_settings: MapSettings,
}

impl GameResult {
    pub fn build(game: &Game) -> Self {
        let winner = game.winner_name().unwrap_or_default();
        let replay_data = game.player_actions.clone();
        let game_settings = game.map_settings.clone();

        GameResult {
            winner,
            replay_data: replay_data
                .iter()
                .map(|command| command.1.clone())
                .collect(),
            game_settings,
        }
    }
}
