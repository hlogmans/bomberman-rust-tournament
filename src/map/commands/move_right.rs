use crate::map::commands::move_action;
use crate::map::commands::traits::player_command::PlayerCommand;
use crate::map::map::Map;

pub struct MoveRight;

impl PlayerCommand for MoveRight {
    fn execute(&self, map: &mut Map, player_index: usize) {
        if let Some(current) = map.get_player_position(player_index) && let Some(new_pos) = current.move_right() {
            move_action::move_player(map, player_index, new_pos);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::coord::Coord;
    use crate::map::commands::move_action::tests::test_move_command;
    use crate::map::commands::move_right::MoveRight;

    #[test]
    fn move_right_works() {
        test_move_command(MoveRight, Coord::from(3, 3), Coord::from(4, 3));
    }
}
