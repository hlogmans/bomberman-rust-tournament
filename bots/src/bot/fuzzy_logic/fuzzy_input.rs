use game::coord::Coord;
use game::map::map::Map;
use game::map::structs::map_config::MapConfig;


#[derive(Clone)]
pub struct FuzzyInput<'a> {
    pub map: &'a Map,
    pub bot_name: String,
    pub bot_id: usize,
    pub current_position: Coord,
    pub map_settings: &'a MapConfig
}