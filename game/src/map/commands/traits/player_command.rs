use crate::map::map::Map;

pub trait PlayerCommand {
    fn try_execute(&self, map: &mut Map, player_index: usize);
}