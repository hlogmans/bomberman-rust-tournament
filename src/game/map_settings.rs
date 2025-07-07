#[allow(dead_code)]
#[derive(Clone)]
pub struct MapSettings {
    pub width: usize,
    pub height: usize,
    pub playernames: Vec<String>,
    pub bombtimer: usize,
    pub bombradius: usize,
    pub endgame: usize,
}

impl Default for MapSettings {
    fn default() -> Self {
        Self {
            width: 15,
            height: 15,
            playernames: vec!["Player 1".to_string(), "Player 2".to_string()],
            bombtimer: 3,
            bombradius: 2,
            endgame: 100,
        }
    }
}
