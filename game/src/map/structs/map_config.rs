use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Clone, Serialize, Deserialize)]
pub struct MapConfig {
    pub size: usize,
    pub bomb_timer: usize,
    pub bomb_radius: usize,
    pub endgame: usize,
}

impl Default for MapConfig {
    fn default() -> Self {
        Self {
            size: 15,
            bomb_timer: 3,
            bomb_radius: 2,
            endgame: 100,
        }
    }
}
