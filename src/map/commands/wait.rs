use crate::map::commands::traits::player_command::PlayerCommand;
use crate::map::map::Map;

pub struct Wait;

impl PlayerCommand for Wait {
    fn execute(&self, _map: &mut Map, _player_index: usize) {
        // Do nothing
    }
}
