// this file contains a basic bot that can play the game.
// it should be modified by the user to create their own sophisticated bot.

// a bot consist of returning a desired name, and a callback that provides player actions based
// onthe current map and the players positions.
// the player shoudl self determine how far the game has progressed, including bomb times.

// for every game a bot is constructed. So the lifetime of the bot is the same as the game.

pub mod easy_bot;
pub mod random_bot;

use crate::coord::Coord;
use crate::game::map_settings::MapSettings;
use crate::map::{Command, Map};

pub trait Bot {
    fn name(&self) -> String;

    fn start_game(&mut self, map_settings: &MapSettings, bot_id: usize) -> bool;

    fn get_move(&mut self, map: &Map, player_location: Coord) -> Command;

    /// Creates a clone of this bot as a new boxed trait object.
    /// This allows creating multiple instances of the same bot for parallel games
    /// or tournament systems.
    fn clone_bot(&self) -> Box<dyn Bot>;
}

pub type BotConstructor = fn(&str) -> Box<dyn Bot>;

/// Creates a clone of a bot as a new boxed trait object.
///
/// This function allows you to create multiple instances of the same bot,
/// which is useful for:
/// - Running parallel games with separate bot states
/// - Tournament systems where bots compete multiple times
/// - Creating backup copies of bot configurations
///
/// # Arguments
///
/// * `bot` - A reference to the bot to clone
///
/// # Returns
///
/// A new boxed bot that is a clone of the input bot
///
/// # Examples
///
/// ```
/// use rust_bomberman::bot::{available_bots, clone_bot};
///
/// let bot_constructors = available_bots();
/// let original_bot = bot_constructors[0]("MyBot");
/// let cloned_bot = clone_bot(original_bot.as_ref());
///
/// // Both bots have the same configuration but are separate instances
/// assert_eq!(original_bot.name(), cloned_bot.name());
/// ```
pub fn clone_bot(bot: &dyn Bot) -> Box<dyn Bot> {
    bot.clone_bot()
}

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

    #[test]
    fn test_bot_cloning() {
        // Test dat we bots kunnen klonen
        let original_bot = random_bot::RandomBot::new("TestBot".to_string());
        let boxed_bot: Box<dyn Bot> = Box::new(original_bot);

        // Test dat we de bot kunnen klonen
        let cloned_bot = clone_bot(boxed_bot.as_ref());

        // Controleer dat beide bots dezelfde naam hebben
        assert_eq!(boxed_bot.name(), cloned_bot.name());
    }

    #[test]
    fn test_easy_bot_cloning() {
        // Test dat we EasyBot ook kunnen klonen
        let original_bot = easy_bot::EasyBot::new("EasyTestBot".to_string());
        let boxed_bot: Box<dyn Bot> = Box::new(original_bot);

        // Test dat we de bot kunnen klonen
        let cloned_bot = clone_bot(boxed_bot.as_ref());

        // Controleer dat beide bots dezelfde naam hebben (voor start_game)
        assert_eq!(boxed_bot.name(), cloned_bot.name());
    }
}
