// this file contains a basic bot that can play the game.
// it should be modified by the user to create their own sophisticated bot.

// a bot consist of returning a desired name, and a callback that provides player actions based
// onthe current map and the players positions.
// the player shoudl self determine how far the game has progressed, including bomb times.

// for every game a bot is constructed. So the lifetime of the bot is the same as the game.

use crate::map::{Command, Map};

pub trait Bot {
    fn name(&self) -> &str;

    fn get_move(&mut self, map: &Map, player_index: usize) -> Command;
}

pub struct RandomBot {}

impl Bot for RandomBot {
    fn name(&self) -> &str {
        "RandomBot"
    }

    fn get_move(&mut self, _map: &Map, _player_index: usize) -> Command {
        // Randomly choose a command for the bot
        use rand::Rng;
        let mut rng = rand::rng();
        let commands = vec![
            Command::Up,
            Command::Down,
            Command::Left,
            Command::Right,
            Command::Wait,
            Command::PlaceBomb,
        ];
        commands[rng.random_range(0..commands.len())].clone()
    }
}
