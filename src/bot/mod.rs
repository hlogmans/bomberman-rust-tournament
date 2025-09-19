// this file contains a basic bot that can play the game.
// it should be modified by the user to create their own sophisticated bot.

// a bot consist of returning a desired name, and a callback that provides player actions based
// onthe current map and the players positions.
// the player shoudl self determine how far the game has progressed, includigerhard_bot times.

// for every game a bot is constructed. So the lifetime of the bot is the same as the game.

// Include the auto-generated code that handles both module declarations and registration
include!(concat!(env!("OUT_DIR"), "/bot_registry.rs"));

// Use the macro to include all bot modules automatically
include_bot_modules!();

use crate::coord::Coord;
use crate::game::map_settings::MapSettings;
use crate::map::map::{Command, Map};

/// Represents a bot that can play the game.
/// The tournament is a competition where bots compete against each other.
/// The tournament code only interacts with the bot through the Bot trait.
///
/// NEW: Bots are now FULLY automatically registered!
/// Just add a new .rs file to src/bot/ with a struct implementing Bot trait and a new() function.
/// No manual registration needed anywhere - not even module declarations!
pub trait Bot {
    fn name(&self) -> String;

    fn start_game(&mut self, map_settings: &MapSettings, bot_id: usize) -> bool;

    fn get_move(&mut self, map: &Map, player_location: Coord) -> Command;
}

pub type BotConstructor = Box<dyn Fn() -> Box<dyn Bot>>;

// The available_bots(), bot_count(), and get_bot_names() functions are automatically generated!

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn all_bots_are_registered() {
        let bots = available_bots();
        let bot_count = bots.len();

        println!("Auto-registered {} bots", bot_count);

        // Test that all bots can be instantiated correctly
        for (i, constructor) in bots.iter().enumerate() {
            let bot = constructor();
            println!("Bot {}: {}", i, bot.name());
        }

        // Expect at least 1 bot to be registered
        assert!(bot_count > 0, "At least 1 bot should be auto-registered");
    }

    #[test]
    fn bot_names_are_unique() {
        let bots = available_bots();
        let mut names = HashSet::new();

        for constructor in bots.iter() {
            let bot = constructor();
            let name = bot.name();
            assert!(
                names.insert(name.clone()),
                "Bot name '{}' appears multiple times",
                name
            );
        }
    }

    #[test]
    fn get_bot_names_works() {
        let names = get_bot_names();
        println!("Bot names: {:?}", names);
        assert!(!names.is_empty());
        assert_eq!(names.len(), available_bots().len());
    }

    #[test]
    fn bot_count_matches_available_bots() {
        assert_eq!(bot_count(), available_bots().len());
    }

    #[test]
    fn all_bots_can_be_instantiated() {
        let bots = available_bots();

        for (i, constructor) in bots.iter().enumerate() {
            let mut bot = constructor();

            // Test basic functionality
            let name = bot.name();
            assert!(!name.is_empty(), "Bot {} should have a non-empty name", i);

            // Test that start_game can be called
            let map_settings = MapSettings::default();
            let _result = bot.start_game(&map_settings, 0);
        }
    }

    #[test]
    fn auto_discovery_finds_expected_bots() {
        let names = get_bot_names();

        // Check that we find the expected bots
        let expected_bots = vec![
            "CuddleBot",
            "EasyBot",
            "GerhardBot",
            "PassiveBot",
            "RandomBot",
            "ScoutBot",
        ];

        for expected in expected_bots {
            assert!(
                names.contains(&expected.to_string()),
                "Expected bot '{}' was not auto-discovered. Found: {:?}",
                expected,
                names
            );
        }
    }

    #[test]
    fn automatic_registration_is_truly_automatic() {
        // This test verifies that the auto-registration actually works
        let bot_count = available_bots().len();
        let name_count = get_bot_names().len();

        println!("Found {} bots with {} names", bot_count, name_count);

        // Should have same number of bots as names
        assert_eq!(bot_count, name_count);

        // Should have found the expected number of existing bots
        assert!(
            bot_count >= 6,
            "Should find at least 6 existing bots, found {}",
            bot_count
        );
    }
}
