use crate::{
    bot::Bot,
    coord::Coord,
    game::map_settings::MapSettings,
    map::{Map, Command},
};

use crate::bot::passive_bot::PassiveBot;

pub struct ScoutBot {
    name: String,
    id: usize,
    map_settings: MapSettings,
    passive_bot: PassiveBot,
    command_list: Vec<Command>,       // First path (runs once)
    current_index: usize,
    looping: bool,
    initialized: bool,                // Did we already pick which initial list?
}

impl ScoutBot {
    pub fn new(name: String) -> Self {
        ScoutBot {
            name,
            id: 0,
            map_settings: MapSettings::default(),
            passive_bot: PassiveBot::new("helping".to_string()),
            command_list: vec![],
            current_index: 0,
            looping: false,
            initialized: false,
        }
    }

    fn topleft_script() -> Vec<Command> {
        vec![
            Command::Right,

            // bomb place and run
            Command::PlaceBomb, Command::Left, Command::Down, Command::Wait,
            Command::Up, Command::Right, Command::Right,

            // bomb place and run
            Command::PlaceBomb, Command::Left, Command::Left, Command::Down,
            Command::Up, Command::Right,

            // bomb place and run
            Command::PlaceBomb, Command::Left, Command::Down, Command::Wait,
            Command::Up, Command::Right, Command::Right, Command::Right, Command::Right,

            // bomb place and run
            Command::PlaceBomb, Command::Left, Command::Left, Command::Down,
            Command::Up, Command::Right, Command::Right,

            // bomb place and run
            Command::PlaceBomb, Command::Left, Command::Left, Command::Down,
            Command::Up, Command::Right, Command::Right,

            // bomb place and run
            Command::PlaceBomb, Command::Left, Command::Left, Command::Down,
            Command::Up, Command::Right, Command::Right, Command::Down, Command::Down,

            // bomb place and run
            Command::PlaceBomb, Command::Up, Command::Up, Command::Left,
            Command::Right, Command::Down, Command::Down, Command::Down, Command::Down,

            // bomb place and run
            Command::PlaceBomb, Command::Up, Command::Up, Command::Right,
            Command::Left, Command::Down, Command::Down,

            // bomb place and run
            Command::PlaceBomb, Command::Up, Command::Up, Command::Left,

            // bomb place and run
            Command::PlaceBomb, Command::Right, Command::Down, Command::Down,
            Command::Left, Command::Left,

            // bomb place and run
            Command::PlaceBomb, Command::Right, Command::Right, Command::Up,
        ]
    }

    fn bottomleft_script() -> Vec<Command> {
        vec![
            Command::Right,

            // bomb place and run
            Command::PlaceBomb, Command::Left, Command::Up, Command::Wait,
            Command::Down, Command::Right, Command::Right,

            // bomb place and run
            Command::PlaceBomb, Command::Left, Command::Left, Command::Up,
            Command::Down, Command::Right,

            // bomb place and run
            Command::PlaceBomb, Command::Left, Command::Up, Command::Wait,
            Command::Down, Command::Right, Command::Right, Command::Right, Command::Right,

            // bomb place and run
            Command::PlaceBomb, Command::Left, Command::Left, Command::Up,
            Command::Down, Command::Right, Command::Right,

            // bomb place and run
            Command::PlaceBomb, Command::Left, Command::Left, Command::Up,
            Command::Down, Command::Right, Command::Right,

            // bomb place and run
            Command::PlaceBomb, Command::Left, Command::Left, Command::Up,
            Command::Down, Command::Right, Command::Right, Command::Up, Command::Up,

            // bomb place and run
            Command::PlaceBomb, Command::Down, Command::Down, Command::Left,
            Command::Right, Command::Up, Command::Up, Command::Up, Command::Up,

            // bomb place and run
            Command::PlaceBomb, Command::Down, Command::Down, Command::Right,
            Command::Left, Command::Up, Command::Up,

            // bomb place and run
            Command::PlaceBomb, Command::Down, Command::Down, Command::Left,

            // bomb place and run
            Command::PlaceBomb, Command::Right, Command::Up, Command::Up,
            Command::Left, Command::Left,

            // bomb place and run
            Command::PlaceBomb, Command::Right, Command::Right, Command::Down,
        ]
    }
}

impl Bot for ScoutBot {
    fn name(&self) -> String {
        format!("{} ({})", self.name, self.id)
    }

    fn start_game(&mut self, settings: &MapSettings, bot_id: usize) -> bool {
        self.id = bot_id;
        self.map_settings = settings.clone();
        true
    }

    fn get_move(&mut self, _map: &Map, me: Coord) -> Command {
        // First time: decide which script to use
        if !self.initialized {
            let height = self.map_settings.height as i32;
            if me.row.get() < height as usize / 2 {
                self.command_list = ScoutBot::topleft_script();
            } else {
                self.command_list = ScoutBot::bottomleft_script();
            }
            self.initialized = true;
            self.current_index = 0;
            self.looping = false;
        }

        // Run initial script
        if !self.looping {
            if self.current_index < self.command_list.len() {
                let cmd = self.command_list[self.current_index];
                self.current_index += 1;
                return cmd;
            }
            // Switch to rotation mode
            self.looping = true;
            self.current_index = 0;
        }

        // Rotation mode
        return self.passive_bot.get_move(_map, me);
    }
}
