use crate::bot::fuzzy_bot::fuzzy_logic::helper;
use crate::coord::Coord;
use crate::map::{CellType, Command, Map};

pub fn get_cell(map: &Map, location: Coord) -> char {
    map.grid
        .get(location.row.get() * map.width + location.col.get())
        .copied()
        .unwrap_or('W')
}

pub fn get_cell_type(cell_value: char) -> CellType {
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
        .filter_map( | coord| {
            if get_cell_type(get_cell(map, *coord)) == CellType::Empty {
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

