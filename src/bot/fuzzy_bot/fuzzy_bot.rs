use super::fuzzy_logic::{
    fuzzy_ai::{decide, Intent},
    fuzzy_input::FuzzyInput,
    fuzzy_core::FuzzyLogic
};

use crate::{
    bot::Bot,
    coord::Coord,
    game::map_settings::MapSettings,
    map::{Command, Map},
};
use crate::bot::fuzzy_bot::fuzzy_logic::fuzzy_ai::handle_intent;

#[derive(Clone)]
pub struct FuzzyBot {
    pub name: String,
    pub id: usize,
    map_settings: MapSettings,
}

impl FuzzyBot {
    pub fn new(name: String) -> Self {
        Self {
            name,
            id: 0,
            map_settings: MapSettings::default(),
        }
    }

    fn get_intent(&self, map: &Map, current_location: Coord) -> Intent{
        let name = &self.name;
        
        decide(FuzzyInput {
            map: map,
            bot_name: name.clone(),
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
        let intent = self.get_intent(map, player_location);
        handle_intent(intent)
    }

    fn start_game(&mut self, map_settings: &MapSettings, bot_id: usize) -> bool {
        self.id = bot_id;
        self.map_settings = map_settings.clone();
        true
    }
}