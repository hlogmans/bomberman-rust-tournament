use crate::{
    bot::Bot,
    map::{Command, Map, MapSettings},
};

pub struct EasyBot {
    pub name: String,
    pub id: usize,
}

impl Bot for EasyBot {
    fn name(&self) -> String {
        // return the name plus the ID
        format!("{} ({})", self.name, self.id)
    }

    fn get_move(&mut self, _map: &Map, _player_index: usize) -> Command {
        // Randomly choose a command for the bot
        use rand::Rng;
        let mut rng = rand::rng();
        let commands = vec![
            Command::Up,
            Command::Down,
            Command::Left,
            Command::Right,
            Command::Wait,
        ];
        commands[rng.random_range(0..commands.len())].clone()
    }
    fn start_game(&mut self, _map_settings: &MapSettings, bot_id: usize) -> bool {
        self.id = bot_id;
        true
    }
}

impl EasyBot {
    pub fn new(name: String) -> Self {
        EasyBot { name, id: 0 }
    }
}
