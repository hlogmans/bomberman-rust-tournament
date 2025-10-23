use crate::bot::fuzzy_logic::fuzzy_input::FuzzyInput;
use crate::bot::fuzzy_logic::manhattan::manhattan;
use game::coord::Coord;
use game::map::cell::CellType;
use game::map::map::Map;

pub fn get_cell(map: &Map, location: Coord) -> char {
    map.grid
        .get(location.row.get() * map.width + location.col.get())
        .copied()
        .unwrap_or('W')
}

pub fn get_cell_type(cell_value: &char) -> CellType {
    match cell_value {
        'W' => CellType::Wall,
        ' ' => CellType::Empty,
        'B' => CellType::Bomb,
        'P' => CellType::Player,
        '.' => CellType::Destroyable,
        _ => CellType::Wall,
    }
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
    vec![
        current_pos.move_left().unwrap(),
        current_pos.move_right().unwrap(),
        current_pos.move_up().unwrap(),
        current_pos.move_down().unwrap(),
    ]
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

pub fn is_tile_in_bomb_range(position: Coord, bomb_position: Coord, map: &Map, radius: usize) -> bool {
    if position == bomb_position {
        return true;
    }

    if position.row == bomb_position.row {
        if position.col.get().abs_diff(bomb_position.col.get()) > radius {
            return false;
        }
    }

    if position.col == bomb_position.col {
        if position.row.get().abs_diff(bomb_position.row.get()) > radius {
            return false;
        }
    }
    if position.col != bomb_position.col && position.row != bomb_position.row {
        return false;
    }

    true
}
