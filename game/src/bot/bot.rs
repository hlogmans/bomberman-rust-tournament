// this file contains a basic bot that can play the game.
// it should be modified by the user to create their own sophisticated bot.

// a bot consist of returning a desired name, and a callback that provides player actions based
// onthe current map and the players positions.
// the player shoudl self determine how far the game has progressed, includigerhard_bot times.

// for every game a bot is constructed. So the lifetime of the bot is the same as the game.

// Include the auto-generated code that handles both module declarations and registration

// Use the macro to include all bot modules automatically


use crate::coord::Coord;
use crate::map::{ map::Map, enums::command::Command };
use crate::map::structs::map_config::MapConfig;

/// Represents a bot that can play the game.
/// The tournament is a competition where bots compete against each other.
/// The tournament code only interacts with the bot through the Bot trait.
///
/// NEW: Bots are now FULLY automatically registered!
/// Just add a new .rs file to src/bot/ with a struct implementing Bot trait and a new() function.
/// No manual registration needed anywhere - not even module declarations!
#[forbid(unsafe_code)]
pub trait Bot {
    fn start_game(&mut self, map_settings: &MapConfig,bot_name: String, bot_id: usize) -> bool;

    fn get_move(&mut self, map: &Map, player_location: Coord) -> Command;

    fn get_debug_info(&self) -> String {
        "".to_string()
    }
}

pub type BotConstructor = Box<dyn Fn() -> BotController + Send + Sync>;

pub struct BotController {
    inner: Box<dyn Bot>,
    id: usize,
    name: String
}

impl BotController {
    pub fn new(inner: Box<dyn Bot>, name: String) -> Self {
        Self {
            inner,
            id: 0,
            name: name,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }


    pub fn get_name(&self) -> String {
        format!("{} ({})", self.name, self.id)
    }

    pub fn start_game(&mut self, map_settings: &MapConfig, bot_id: usize) -> bool {
        self.id = bot_id;
        self.inner.start_game(map_settings, self.name.clone(), self.id)
    }

    pub fn get_move(&mut self, map: &Map, player_location: Coord) -> Command {
        self.inner.get_move(map, player_location)
    }

    pub fn get_debug_info(&self) -> String {
        self.inner.get_debug_info()
    }
}