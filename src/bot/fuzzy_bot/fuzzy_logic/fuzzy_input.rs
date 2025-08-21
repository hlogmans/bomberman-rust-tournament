use crate::coord::Coord;
use crate::game::map_settings::MapSettings;
use crate::map::*;

pub struct FuzzyInput<'a> {
    pub map: &'a Map,
    pub bot_name: String,
    pub bot_id: usize,
    pub current_position: Coord,
    pub map_settings: &'a MapSettings
}