use crate::map::cell::CellType;
use crate::map::commands::traits::player_command::PlayerCommand;
use crate::map::map::Map;

pub struct PlaceBomb;

impl PlayerCommand for PlaceBomb {
    fn execute(&self, map: &mut Map, player_index: usize) {
        if let Some(pos) = map.get_player_position(player_index) {
            map.set_cell(pos, CellType::Bomb);
            map.add_bomb(pos);
        }
    }
}
