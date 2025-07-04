// this file contains a basic bot that can play the game.
// it should be modified by the user to create their own sophisticated bot.

// a bot consist of returning a desired name, and a callback that provides player actions based
// onthe current map and the players positions.
// the player shoudl self determine how far the game has progressed, including bomb times.

// for every game a bot is constructed. So the lifetime of the bot is the same as the game.

pub mod random_bot;
use crate::map::{Command, Map};

pub trait Bot {
    fn name(&self) -> &str;

    fn get_move(&mut self, map: &Map, player_index: usize) -> Command;
}
