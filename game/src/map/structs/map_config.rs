use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Clone, Serialize, Deserialize)]
pub struct MapConfig {
    pub width: usize,
    pub height: usize,
    pub player_names: Vec<String>,
    pub bomb_timer: usize,
    pub bomb_radius: usize,
    pub endgame: usize,
}

impl Default for MapConfig {
    fn default() -> Self {
        Self {
            width: 15,
            height: 15,
            player_names: vec!["Player 1".to_string(), "Player 2".to_string()],
            bomb_timer: 3,
            bomb_radius: 2,
            endgame: 100,
        }
    }
}
