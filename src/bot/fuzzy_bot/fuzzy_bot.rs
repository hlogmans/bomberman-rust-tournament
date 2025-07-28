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

    fn get_intent() -> Intent{
        decide()
    }
}

impl Bot for FuzzyBot {
    fn name(&self) -> String {
        format!("{} ({})", self.name, self.id)
    }

    fn get_move(&mut self, map: &Map, player_location: Coord) -> Command {
        let inputs = self.get_inputs(map, player_location);
        let (_intent, dir) = fuzzy_ai::decide(&inputs);
        dir
    }

    fn start_game(&mut self, map_settings: &MapSettings, bot_id: usize) -> bool {
        self.id = bot_id;
        self.map_settings = map_settings.clone();
        true
    }
}