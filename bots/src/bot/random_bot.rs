use game::bot::bot::Bot;
use game::coord::Coord;
use game::map::enums::command::Command;
use game::map::map::Map;
use game::map::structs::map_config::MapConfig;

#[derive(Clone)]
pub struct RandomBot {
    pub name: String,
    pub id: usize,
}

impl RandomBot {}

impl Bot for RandomBot {
    fn get_move(&mut self, _map: &Map, _player_location: Coord) -> Command {
        // Randomly choose a command for the bot
        use rand::Rng;
        let mut rng = rand::rng();
        let commands = [
            Command::Up,
            Command::Down,
            Command::Left,
            Command::Right,
            Command::Wait,
            // Command::PlaceBomb,
        ];
        commands[rng.random_range(0..commands.len())]
    }

    fn start_game(&mut self, _: &MapConfig, bot_name: String, bot_id: usize) -> bool {
        self.id = bot_id;
        self.name = bot_name;
        true
    }
}

impl Default for RandomBot {
    fn default() -> Self {
        Self::new()
    }
}

impl RandomBot {
    pub fn new() -> Self {
        RandomBot {
            name: "RandomBot".to_string(),
            id: 0,
        }
    }
}
