use game::coord::Coord;
use game::map::bomb::Bomb;
use game::map::grid::cell::CellType;
use game::map::enums::command::Command;


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

pub fn get_neighbour_coords(current_pos: Coord) -> Vec<Coord> {
    let mut neighbours = Vec::new();
    if let Some(c) = current_pos.move_left()  { neighbours.push(c); }
    if let Some(c) = current_pos.move_right() { neighbours.push(c); }
    if let Some(c) = current_pos.move_up()    { neighbours.push(c); }
    if let Some(c) = current_pos.move_down()  { neighbours.push(c); }
    neighbours
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

pub fn get_cell_type(cell_value: &char) -> CellType {
    match cell_value {
        'W' => CellType::Wall,
        'P' => CellType::Player,
        ' ' => CellType::Empty,
        '.' => CellType::Destroyable,
        _ => CellType::Wall,
    }
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
