use crate::{
    bot::Bot,
    coord::Coord,
    game::map_settings::MapSettings,
    map::{Command, Map},
};
use crate::bot::passive_bot::PassiveBot;

#[derive(Clone)]
pub struct ScoutBot {
    pub name: String,
    pub id: usize,
    map_settings : MapSettings,
    passive_bot: PassiveBot,
    command_list: Vec<Command>,
    current_index: usize,
    looping: bool,
    initialized: bool,
}

impl ScoutBot {
    pub fn new() -> Self {
        ScoutBot {
            name: "ScoutBot".to_string(),
            id: 0,
            map_settings: MapSettings::default(),
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
        ]
    }

    fn get_correct_init_list(loc: Coord, height: i32) -> Vec<Command> {
        if loc.row.get() < height as usize / 2 {
            ScoutBot::hardcoded_script()
        } else {
           ScoutBot::hardcoded_script().into_iter()
                .map(|c| match c {
                    Command::Up => Command::Down,
                    Command::Down => Command::Up,
                    other => other,
                })
                .collect()
        }
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

    fn get_move(&mut self, _map: &Map, _player_location: Coord) -> Command {
        if !self.initialized {
            let height = self.map_settings.height as i32;
            self.command_list = ScoutBot::get_correct_init_list(_player_location, height);

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
        return self.passive_bot.get_move(_map, _player_location);
    }
}
