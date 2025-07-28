use std::cmp::PartialEq;
use crate::bot::fuzzy_bot::fuzzy_logic::manhattan::manhattan;
use super::fuzzy_core::FuzzyLogic;
use super::fuzzy_input::FuzzyInput;
use crate::coord::Coord;
use crate::map::{Command, Map};

#[derive(Debug, PartialEq)]
pub enum Intent {
    PlaceBomb,
    Flee,
    MoveToEnemy,
    Wait,
}

pub fn decide(input: FuzzyInput) -> Intent {
    let distance_to_closest_enemy =
        determine_distance_to_closest_enemy(&input.map, input.bot_name, input.current_position);

    let close = FuzzyLogic::closeness(distance_to_closest_enemy, input.map_settings.bombradius);
    let danger = FuzzyLogic::danger_level_at(input.current_position, &input.map, input.map_settings.bombradius);

    let mut score = 0.0;
    let mut intent = Intent::Wait;

    if danger > 0.7 {
        intent = Intent::Flee;
    } else if close > 0.9 {
        intent = Intent::PlaceBomb
    } else {
        intent = Intent::MoveToEnemy
    }

    intent
}


fn handle_intent(intent: Intent){
    if intent == Intent::PlaceBomb {

    }
}

fn determine_distance_to_closest_enemy(map: &Map, bot_name: String, my_position: Coord) -> i32 {
    let mut closest = 999999;

    for player in &map.players {
        if player.name != bot_name {
            let distance_to_player = manhattan(my_position, player.position);

            if distance_to_player < closest {
                closest = distance_to_player;
            }
        }
    }

    closest
}
