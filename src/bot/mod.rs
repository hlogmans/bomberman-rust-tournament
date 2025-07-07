// this file contains a basic bot that can play the game.
// it should be modified by the user to create their own sophisticated bot.

// a bot consist of returning a desired name, and a callback that provides player actions based
// onthe current map and the players positions.
// the player shoudl self determine how far the game has progressed, including bomb times.

// for every game a bot is constructed. So the lifetime of the bot is the same as the game.

pub mod easy_bot;
pub mod random_bot;

use crate::map::{Command, Map, MapSettings};

pub trait Bot {
    fn name(&self) -> String;

    fn start_game(&mut self, map_settings: &MapSettings, bot_id: usize) -> bool;

    fn get_move(&mut self, map: &Map, player_index: usize) -> Command;
}

pub type BotConstructor = fn(&str) -> Box<dyn Bot>;

pub fn available_bots() -> Vec<BotConstructor> {
    vec![
        |name| Box::new(random_bot::RandomBot::new(name.to_string())),
        |name| Box::new(easy_bot::EasyBot::new(name.to_string())),
        // Voeg hier nieuwe bots toe!
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_bots_are_registered() {
        // Verwacht bijvoorbeeld 2 bots:
        assert_eq!(available_bots().len(), 2);
    }
}
