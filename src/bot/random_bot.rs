use crate::{
    bot::Bot,
    coord::Coord,
    game::map_settings::MapSettings,
    map::{Command, Map},
};

#[derive(Clone)]
pub struct RandomBot {
    pub name: String,
    pub id: usize,
}

impl RandomBot {
    pub fn new(name: String) -> Self {
        RandomBot { name, id: 0 }
    }
}

impl Bot for RandomBot {
    fn name(&self) -> String {
        // return the name plus the ID
        format!("{} ({})", self.name, self.id)
    }

    fn get_move(&mut self, _map: &Map, _player_location: Coord) -> Command {
        // Randomly choose a command for the bot
        use rand::Rng;
        let mut rng = rand::rng();
        let commands = vec![
            Command::Up,
            Command::Down,
            Command::Left,
            Command::Right,
            Command::Wait,
            // Command::PlaceBomb,
        ];
        commands[rng.random_range(0..commands.len())].clone()
    }

    fn start_game(&mut self, _: &MapSettings, bot_id: usize) -> bool {
        self.id = bot_id;
        true
    }
}
