use game::coord::Coord;
use game::map::cell::CellType;
use game::map::enums::command::Command;
use game::map::map::Map;
use game::map::structs::map_config::MapConfig;
use crate::bot::fuzzy_logic::fuzzy_input::FuzzyInput;
use crate::bot::fuzzy_logic::fuzzy_movement;
use crate::bot::fuzzy_logic::helper;
use crate::bot::fuzzy_logic::fuzzy_core::FuzzyLogic;
use crate::bot::fuzzy_logic::manhattan::manhattan;

pub fn handle_move_decision(input: FuzzyInput) -> Command {
    let neighbours = helper::get_neighbour_coords(input.current_position);
    let enemy_positions: Vec<Coord> = input
        .map
        .players
        .iter()
        .filter_map(|p| {
            if p.name != input.bot_name.clone() + " (" + &input.bot_id.to_string() + ")" {
                Some(p.position)
            } else {
                None
            }
        })
        .collect();

    let mut command = Command::Wait;
    let mut highest_tile_score = 0.0;
    for neighbour in neighbours {
        let tile_score = fuzzy_movement::get_tile_score(
            input.map,
            neighbour,
            &enemy_positions,
            input.current_position,
            input.map_settings,
        );
        if tile_score > highest_tile_score {
            highest_tile_score = tile_score;

            if helper::get_cell_type(helper::get_cell(input.map, neighbour))
                == CellType::Destroyable
            {
                command = Command::PlaceBomb;
            } else {
                command = fuzzy_movement::get_command_to_move_to_coord(input.current_position, neighbour)
            }
        }
    }

    command
}

fn get_tile_score(
    map: &Map,
    coord: Coord,
    enemy_positions: &Vec<Coord>,
    current_position: Coord,
    map_settings: &MapConfig,
) -> f64 {
    let cell_type = helper::get_cell_type(helper::get_cell(map, coord));

    if !(cell_type == CellType::Empty || cell_type == CellType::Destroyable) {
        return f64::NEG_INFINITY; // not valid
    }

    let danger_level = FuzzyLogic::danger_level_at(coord, map, map_settings.bomb_radius);

    let closest_enemy = enemy_positions
        .iter()
        .copied()
        .min_by_key(|&e| manhattan(current_position, e))
        .unwrap_or(coord); // fallback to self

    let old_distance = manhattan(current_position, closest_enemy);
    let new_distance = manhattan(coord, closest_enemy);

    let mut closer = 0.0;
    if old_distance > new_distance {
        closer = 0.3
    }



    let escape_routes = [
        coord.move_up(),
        coord.move_down(),
        coord.move_left(),
        coord.move_right(),
    ]
        .iter()
        .filter_map(|&opt| opt)
        .filter(|&n| helper::get_cell_type(helper::get_cell(map, n)) == CellType::Empty)
        .count();

    let escape_score = (escape_routes as f64) / 4.0;

    let final_score = closer * 0.3 + escape_score * 0.2 - danger_level * 0.4;

    final_score
}

pub fn get_command_to_move_to_coord(current_position: Coord, target_position: Coord) -> Command {
    let mut command = Command::Wait;
    if current_position.move_down().unwrap() == target_position {
        command = Command::Down;
    }
    if current_position.move_up().unwrap() == target_position {
        command = Command::Up;
    }
    if current_position.move_left().unwrap() == target_position {
        command = Command::Left;
    }
    if current_position.move_right().unwrap() == target_position {
        command = Command::Right
    }

    return command;
}