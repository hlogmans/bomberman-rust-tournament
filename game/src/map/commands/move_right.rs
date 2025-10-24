use crate::map::commands::move_action::{self, Direction};
use crate::map::commands::traits::player_command::PlayerCommand;
use crate::map::map::Map;

pub struct MoveRight;

impl PlayerCommand for MoveRight {
    fn try_execute(&self, map: &mut Map, player_index: usize) {
        move_action::try_move_player(map, player_index, Direction::Right);
    }
}