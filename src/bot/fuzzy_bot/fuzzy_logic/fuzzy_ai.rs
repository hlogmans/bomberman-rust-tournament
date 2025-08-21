use super::fuzzy_core::FuzzyLogic;
use super::fuzzy_input::FuzzyInput;
use crate::bot::fuzzy_bot::fuzzy_logic::manhattan::manhattan;
use crate::coord::Coord;
use crate::map::{Command, Map};
use std::cmp::PartialEq;

#[derive(Debug, PartialEq)]
pub enum Intent {
    PlaceBomb,
    Flee,
    Move,
    Wait,
}

pub fn decide(input: FuzzyInput) -> Intent {
    let distance_to_closest_enemy =
        determine_distance_to_closest_enemy(&input.map, input.bot_name, input.bot_id, input.current_position);

    let close = FuzzyLogic::closeness(distance_to_closest_enemy, input.map_settings.bombradius);
    let danger = FuzzyLogic::danger_level_at(
        input.current_position,
        &input.map,
        input.map_settings.bombradius,
    );

    let mut intent = Intent::Wait;

    if danger > 0.7 {
        intent = Intent::Flee;
    } else if close > 0.9 {
        intent = Intent::PlaceBomb
    } else {
        intent = Intent::Move
    }

    intent
}

pub fn handle_intent(intent: Intent, input: FuzzyInput) -> Command {
    if intent == Intent::PlaceBomb {
        return Command::PlaceBomb;
    }
    if intent == Intent::Move {
        return FuzzyLogic::handle_move_decision(input)
    }

    if  intent == Intent::Flee {
        return FuzzyLogic::get_safest_move(input)
    }

    Command::Wait
}

fn determine_distance_to_closest_enemy(map: &Map, bot_name: String, bot_id: usize, my_position: Coord) -> i32 {
    let mut closest = 999999;
    let name = (bot_name + " (" + &bot_id.to_string() + ")").to_string();

    for player in &map.players {
        if player.name != name {
            let distance_to_player = manhattan(my_position, player.position);

            if distance_to_player < closest {
                closest = distance_to_player;
            }
        }
    }

    closest
}
