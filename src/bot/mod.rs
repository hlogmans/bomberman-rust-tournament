// this file contains a basic bot that can play the game.
// it should be modified by the user to create their own sophisticated bot.

// a bot consist of returning a desired name, and a callback that provides player actions based
// onthe current map and the players positions.
// the player shoudl self determine how far the game has progressed, including bomb times.

// for every game a bot is constructed. So the lifetime of the bot is the same as the game.

pub mod cuddle_bot;
pub mod easy_bot;
pub mod gerhard;
pub mod passive_bot;
pub mod random_bot;
pub mod scout_bot;

use crate::coord::Coord;
use crate::game::map_settings::MapSettings;
use crate::map::{Command, Map};
use std::collections::HashMap;

pub trait Bot {
    fn name(&self) -> String;

    fn start_game(&mut self, map_settings: &MapSettings, bot_id: usize) -> bool;

    fn get_move(&mut self, map: &Map, player_location: Coord) -> Command;
}

pub type BotConstructor = Box<dyn Fn() -> Box<dyn Bot>>;

pub fn bot_registry() -> HashMap<&'static str, BotConstructor> {
    let mut map: HashMap<&'static str, BotConstructor> = HashMap::new();
    map.insert(
        "RandomBot",
        Box::new(|| Box::new(crate::bot::random_bot::RandomBot::new()) as Box<dyn Bot>)
            as BotConstructor,
    );
    map.insert(
        "EasyBot",
        Box::new(|| Box::new(crate::bot::easy_bot::EasyBot::new()) as Box<dyn Bot>)
            as BotConstructor,
    );
    // Voeg hier nieuwe bots toe!
    map
}

pub fn instantiate_bots(names: &[&str]) -> Vec<Box<dyn Bot>> {
    let registry = bot_registry();
    names
        .iter()
        .filter_map(|name| registry.get(*name).map(|ctor| ctor()))
        .collect()
}

pub fn available_bots() -> Vec<BotConstructor> {
    vec![
        Box::new(|| Box::new(random_bot::RandomBot::new())),
        Box::new(|| Box::new(easy_bot::EasyBot::new())),
        Box::new(|| Box::new(gerhard::GerhardBot::new())),
        Box::new(|| Box::new(cuddle_bot::CuddleBot::new())),
        Box::new(|| Box::new(scout_bot::ScoutBot::new())),
        // Voeg hier nieuwe bots toe!
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_bots_are_registered() {
        // Verwacht bijvoorbeeld 2 bots:
        assert_eq!(available_bots().len(), 3);
    }
}
