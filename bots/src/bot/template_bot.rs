use game::bot::bot::Bot;
use game::coord::Coord;
use game::map::enums::command::Command;
use game::map::map::Map;
use game::map::structs::map_config::MapConfig;

/// Template struct for implementing a new bot
#[derive(Clone)]
pub struct TemplateBot {
    pub name: String,
    pub id: usize,
}

impl TemplateBot {
    /// Constructs a new bot with default field values
    pub fn new() -> Self {
        Self {
            name: String::new(),
            id: 0,
        }
    }

    // Implement bot-specific helper functions here if needed
}

impl Bot for TemplateBot {
    /// Called once at the start of the game to initialize the bot
    fn start_game(&mut self, _settings: &MapConfig, bot_name: String, bot_id: usize) -> bool {
        self.name = bot_name;
        self.id = bot_id;
        true
    }

    /// Called each turn to determine the bot's move
    fn get_move(&mut self, _map: &Map, _player_location: Coord) -> Command {
        Command::Wait
    }

    /// Optional: provide debug information for this turn
    fn get_debug_info(&self) -> String {
        String::new()
    }
}
