use crate::map::commands::move_action;
use crate::map::commands::traits::player_command::PlayerCommand;
use crate::map::map::Map;

pub struct MoveRight;

impl PlayerCommand for MoveRight {
    fn execute(&self, map: &mut Map, player_index: usize) {
        if let Some(current) = map.get_player_position(player_index) {
            if let Some(new_pos) = current.move_right() {
                move_action::move_player(map, player_index, new_pos);
            }
        }
    }
}