use crate::map::map::Map;

pub trait PlayerCommand {
    fn execute(&self, map: &mut Map, player_index: usize);
}