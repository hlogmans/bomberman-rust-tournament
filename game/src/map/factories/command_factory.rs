use crate::map::commands::{
    move_down::MoveDown, move_left::MoveLeft, move_right::MoveRight,
    move_up::MoveUp, place_bomb::PlaceBomb, wait::Wait,
};
use crate::map::commands::traits::player_command::PlayerCommand;
use crate::map::enums::command::Command;


pub struct CommandFactory;

impl CommandFactory {
    pub fn create(command: &Command) -> Option<Box<dyn PlayerCommand>> {
        match command {
            Command::Up => Some(Box::new(MoveUp)),
            Command::Down => Some(Box::new(MoveDown)),
            Command::Left => Some(Box::new(MoveLeft)),
            Command::Right => Some(Box::new(MoveRight)),
            Command::PlaceBomb => Some(Box::new(PlaceBomb)),
            Command::Wait => Some(Box::new(Wait)),
        }
    }
}
