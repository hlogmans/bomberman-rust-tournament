#![forbid(unsafe_code)]

mod bot;

use game::bot::bot::BotConstructor;

include!(concat!(env!("OUT_DIR"), "/bot_registry.rs"));
include_bot_modules!();

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
            println!("Bot {}: {}", i, bot.get_name());
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
            let name = bot.get_name();
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
    fn auto_discovery_finds_expected_bots() {
        let names = get_bot_names();

        // Check that we find the expected bots
        let expected_bots = vec![
            "CuddleBot",
            "EasyBot",
            "GerhardBot",
            "PassiveBot",
            "RandomBot",
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
