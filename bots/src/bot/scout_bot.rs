use game::bot::bot::Bot;
use game::coord::Coord;
use game::map::enums::command::Command;
use game::map::map::Map;
use game::map::structs::map_config::MapConfig;

use crate::passive_bot::PassiveBot;

#[derive(Clone)]
pub struct ScoutBot {
    pub name: String,
    pub id: usize,
    map_settings: MapConfig,
    passive_bot: PassiveBot,
    command_list: Vec<Command>,
    current_index: usize,
    looping: bool,
    initialized: bool,
}

impl Default for ScoutBot {
    fn default() -> Self {
        Self::new()
    }
}

impl ScoutBot {
    pub fn new() -> Self {
        ScoutBot {
            name: "ScoutBot".to_string(),
            id: 0,
            map_settings: MapConfig::default(),
            passive_bot: PassiveBot::new(),
            command_list: vec![],
            current_index: 0,
            looping: false,
            initialized: false,
        }
    }

    fn hardcoded_script() -> Vec<Command> {
        vec![
            Command::Right,
            // bomb place and run
            Command::PlaceBomb,
            Command::Left,
            Command::Down,
            Command::Wait,
            Command::Up,
            Command::Right,
            Command::Right,
            // bomb place and run
            Command::PlaceBomb,
            Command::Left,
            Command::Left,
            Command::Down,
            Command::Up,
            Command::Right,
            // bomb place and run
            Command::PlaceBomb,
            Command::Left,
            Command::Down,
            Command::Wait,
            Command::Up,
            Command::Right,
            Command::Right,
            Command::Right,
            Command::Right,
            // bomb place and run
            Command::PlaceBomb,
            Command::Left,
            Command::Left,
            Command::Down,
            Command::Up,
            Command::Right,
            Command::Right,
            // bomb place and run
            Command::PlaceBomb,
            Command::Left,
            Command::Left,
            Command::Down,
            Command::Up,
            Command::Right,
            Command::Right,
            // bomb place and run
            Command::PlaceBomb,
            Command::Left,
            Command::Left,
            Command::Down,
            Command::Up,
            Command::Right,
            Command::Right,
            Command::Down,
            Command::Down,
            // bomb place and run
            Command::PlaceBomb,
            Command::Up,
            Command::Up,
            Command::Left,
            Command::Right,
            Command::Down,
            Command::Down,
            Command::Down,
            Command::Down,
            // bomb place and run
            Command::PlaceBomb,
            Command::Up,
            Command::Up,
            Command::Right,
            Command::Left,
            Command::Down,
            Command::Down,
            // bomb place and run
            Command::PlaceBomb,
            Command::Up,
            Command::Up,
            Command::Left,
        ]
    }

    fn get_correct_init_list(loc: Coord, height: i32, width: i32) -> Vec<Command> {
        // Start with the base script
        let mut script = ScoutBot::hardcoded_script();

        // Adjust for row
        if loc.row.get() >= height as usize / 2 {
            script = script
                .into_iter()
                .map(|c| match c {
                    Command::Up => Command::Down,
                    Command::Down => Command::Up,
                    other => other,
                })
                .collect();
        }

        // Adjust for column
        if loc.col.get() >= width as usize / 2 {
            script = script
                .into_iter()
                .map(|c| match c {
                    Command::Left => Command::Right,
                    Command::Right => Command::Left,
                    other => other,
                })
                .collect();
        }

        script
    }
}

impl Bot for ScoutBot {
    fn start_game(&mut self, settings: &MapConfig, bot_name: String, bot_id: usize) -> bool {
        self.id = bot_id;
        self.name = bot_name;
        self.map_settings = settings.clone();
        true
    }

    fn get_move(&mut self, _map: &Map, _player_location: Coord) -> Command {
        if !self.initialized {
            let height = self.map_settings.size as i32;
            let width = self.map_settings.size as i32;

            self.command_list = ScoutBot::get_correct_init_list(_player_location, height, width);

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
        self.passive_bot.get_move(_map, _player_location)
    }
}
