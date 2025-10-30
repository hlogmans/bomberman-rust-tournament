use crate::bot::fuzzy_logic::fuzzy_input::FuzzyInput;
use crate::bot::fuzzy_logic::manhattan::manhattan;
use game::coord::Coord;
use game::map::bomb::Bomb;
use game::map::cell::CellType;
use game::map::enums::command::Command;
use game::map::map::Map;
use crate::bot::fuzzy_logic::helper;

pub fn get_cell(map: &Map, location: Coord) -> char {
    map.grid.tiles
        .get(location.row.get() * map.map_settings.size + location.col.get())
        .copied()
        .unwrap_or('W')
}

pub fn get_cell_type(cell_value: &char) -> CellType {
    match cell_value {
        'W' => CellType::Wall,
        ' ' => CellType::Empty,
        '.' => CellType::Destroyable,
        _ => CellType::Wall,
    }
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

pub fn get_empty_neighbours(map: &Map, position: Coord) -> Vec<Coord> {
    get_neighbour_coords(position)
        .iter()
        .filter_map(|coord| {
            if get_cell_type(&get_cell(map, *coord)) == CellType::Empty {
                Some(*coord)
            } else {
                None
            }
        })
        .collect()
}

pub fn get_neighbour_coords(current_pos: Coord) -> Vec<Coord> {
    let mut neighbours = Vec::new();
    if let Some(c) = current_pos.move_left()  { neighbours.push(c); }
    if let Some(c) = current_pos.move_right() { neighbours.push(c); }
    if let Some(c) = current_pos.move_up()    { neighbours.push(c); }
    if let Some(c) = current_pos.move_down()  { neighbours.push(c); }
    neighbours
}

pub fn closest_enemy_distance(enemy_positions: Vec<Coord>, current_position: Coord) -> i32 {
    let closest_enemy = enemy_positions
        .iter()
        .copied()
        .min_by_key(|&e| manhattan(current_position, e))
        .unwrap();

    manhattan(closest_enemy, current_position)
}

pub fn get_enemy_positions(input: FuzzyInput) -> Vec<Coord> {
    input
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
        .collect()
}

pub fn is_tile_in_bomb_range(position: Coord, bomb_position: Coord, radius: usize) -> bool {
    if position == bomb_position {
        return true;
    }

    if position.row == bomb_position.row {
        if position.col.get().abs_diff(bomb_position.col.get()) <= radius {
            return true;
        }
    }

    if position.col == bomb_position.col {
        if position.row.get().abs_diff(bomb_position.row.get()) <= radius {
            return true;
        }
    }

    false
}

pub(crate) fn is_tile_currently_safe(bombs: &Vec<Bomb>, coord: Coord, steps_to_reach_coord: usize, radius:usize) -> bool {
    if bombs.len() == 0 {
        return true;
    }
    for bomb in bombs.iter(){
        if !tile_current_safety_from_bomb(coord, steps_to_reach_coord, bomb, radius) {
            return false
        }
    }


    return true;
}

pub fn tile_current_safety_from_bomb(position: Coord, steps_to_reach: usize ,bomb: &Bomb, radius: usize) -> bool {
    if !is_tile_in_bomb_range(position, bomb.position, radius) {
        return true;
    }

    if bomb.timer  > steps_to_reach {
        return true;
    }


    return false;
}
