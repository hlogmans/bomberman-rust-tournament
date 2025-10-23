use game::bot::bot::Bot;
use game::coord::Coord;
use game::map::enums::command::Command;
use game::map::map::Map;
use game::map::structs::map_config::MapConfig;
use crate::bot::fuzzy_logic::fuzzy_ai::{decide, handle_intent};
use crate::bot::fuzzy_logic::fuzzy_input::FuzzyInput;

#[derive(Clone)]
pub struct FuzzyBot {
    pub name: String,
    pub id: usize,
    map_settings: MapConfig,
}

impl FuzzyBot {
    pub fn new() -> Self {
        FuzzyBot {
            name: "JustinBot".to_string(),
            id: 0,
            map_settings: MapConfig::default()
        }
    }

    fn get_intent(&self, map: &Map, current_location: Coord) -> Command{
        let name = &self.name;

        let intent = decide(FuzzyInput {
            map,
            bot_name: name.clone(),
            bot_id: self.id,
            current_position: current_location,
            map_settings: &self.map_settings
        });

        handle_intent(intent, FuzzyInput {
            map,
            bot_name: name.clone(),
            bot_id: self.id,
            current_position: current_location,
            map_settings: &self.map_settings
        })
    }

}

impl Bot for FuzzyBot {
    fn name(&self) -> String {
        format!("{} ({})", self.name, self.id)
    }

    fn get_move(&mut self, map: &Map, player_location: Coord) -> Command {
        map.
        self.get_intent(map, player_location)
    }

    fn start_game(&mut self, map_settings: &MapConfig, bot_id: usize) -> bool {
        self.id = bot_id;
        self.map_settings = map_settings.clone();
        true
    }
}