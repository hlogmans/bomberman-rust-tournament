use super::fuzzy_core::FuzzyLogic;
use super::fuzzy_input::FuzzyInput;
use std::cmp::PartialEq;
use game::map::enums::command::Command;
use crate::bot::fuzzy_logic::{fuzzy_movement, helper};

#[derive(Debug, PartialEq)]
pub enum Intent {
    PlaceBomb,
    Flee,
    Move,
    Wait,
}

pub fn decide(input: FuzzyInput) -> Intent {
    let distance_to_closest_enemy = helper::closest_enemy_distance(helper::get_enemy_positions(input.clone()), input.current_position.clone());

    let close = FuzzyLogic::closeness(distance_to_closest_enemy, input.map_settings.bomb_radius);
    let danger = FuzzyLogic::danger_level_at(
        input.current_position,
        &input.map,
        input.map_settings.bomb_radius,
    );

    let mut intent = Intent::Wait;

    if danger > 0.6 {
        intent = Intent::Flee;
    } else if close > 0.7 {
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
        return fuzzy_movement::handle_move_decision(input)
    }

    if  intent == Intent::Flee {
        return FuzzyLogic::get_safest_move(input)
    }

    Command::Wait
}


