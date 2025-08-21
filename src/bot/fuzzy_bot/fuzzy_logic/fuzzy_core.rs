use crate::bot::fuzzy_bot::fuzzy_logic::fuzzy_input::FuzzyInput;
use crate::bot::fuzzy_bot::fuzzy_logic::manhattan::manhattan;
use crate::coord::{Col, Coord, Row};
use crate::game::map_settings::MapSettings;
use crate::map::{Bomb, CellType, Command, Map};

pub struct FuzzyLogic;

impl FuzzyLogic {
    pub fn danger_level(seconds: usize) -> f32 {
        if seconds == 1 {
            1.0
        } else if seconds < 3 {
            (3.0 - seconds as f32) / 2.0
        } else {
            0.0
        }
    }

    pub fn danger_level_at(position: Coord, map: &Map, radius: usize) -> f64 {
        let mut total_danger = 0.0;
        let bombs = &map.bombs;

        for bomb in bombs {
            let bomb_position = bomb.position;
            if bomb_position == position {
                return 1.0;
            }

            let in_row = position.row == bomb_position.row;
            let in_col = position.col == bomb_position.col;

            if in_row || in_col {
                let distance_to_bomb = manhattan(position, bomb_position);

                if radius as u32 * 3 > distance_to_bomb as u32 {
                    let blocked_by_wall = if in_row {
                        Self::is_blocked_by_wall(map, bomb, position, false)
                    } else {
                        Self::is_blocked_by_wall(map, bomb, position, true)
                    };

                    if !blocked_by_wall {
                        let relevant_distance = radius * 3;
                        let dist_score = Self::closeness(distance_to_bomb, relevant_distance);
                        let time_score = Self::danger_level(bomb.timer) as f64;
                        total_danger += (dist_score * time_score).clamp(0.0, 1.0);
                    }
                }
            }
        }
        total_danger.clamp(0.0, 1.0)
    }

    pub fn closeness(distance_to_player: i32, relevant_distance: usize) -> f64 {
        let normalized = 1.0 - (distance_to_player as f64 / relevant_distance as f64);

        normalized.clamp(0.0, 1.0)
    }

    pub fn handle_move_decision(input: FuzzyInput) -> Command {
        let neighbours = Self::get_moves_to_neighbour_coords(input.current_position);
        let enemy_positions: Vec<Coord> = input
            .map
            .players
            .iter()
            .filter_map(|p| {
                if p.name !=  input.bot_name.clone() + " (" + &input.bot_id.to_string() + ")" {
                    Some(p.position)
                } else {
                    None
                }
            })
            .collect();

        let mut command = Command::Wait;
        let mut highest_tile_score = 0.0;
        for neighbour in neighbours {
            let tile_score = FuzzyLogic::get_tile_score(
                input.map,
                neighbour.0,
                &enemy_positions,
                input.current_position,
                input.map_settings,
            );
            if tile_score > highest_tile_score {
                highest_tile_score = tile_score;

                if Self::get_cell_type(Self::get_cell(input.map, neighbour.0))
                    == CellType::Destroyable
                {
                    command = Command::PlaceBomb;
                } else {
                    command = neighbour.1;
                }
            }
        }

        command
    }

    pub fn get_safest_move(input: FuzzyInput) -> Command {
        let neighbours = Self::get_moves_to_neighbour_coords(input.current_position);
        let mut lowest_danger_score = 2.0;
        let mut command = Command::Wait;

        for neighbour in neighbours {
            let score = Self::danger_level_at(neighbour.0, input.map, input.map_settings.bombradius);
            if score < lowest_danger_score {
                lowest_danger_score = score;
                command = neighbour.1
            }
        }

        command

    }

    fn get_cell(map: &Map, location: Coord) -> char {
        map.grid
            .get(location.row.get() * map.width + location.col.get())
            .copied()
            .unwrap_or('W')
    }

    fn get_cell_type(cell_value: char) -> CellType {
        match cell_value {
            'W' => CellType::Wall,
            ' ' => CellType::Empty,
            'B' => CellType::Bomb,
            'P' => CellType::Player,
            '.' => CellType::Destroyable,
            _ => CellType::Wall,
        }
    }

    fn is_blocked_by_wall(map: &Map, bomb: &Bomb, position: Coord, move_row: bool) -> bool {
        Self::is_coord_blocked_by_wall(map, bomb.position, position, move_row)
    }

    fn is_coord_blocked_by_wall(map: &Map, coord: Coord, position: Coord, move_row: bool) -> bool {
        let mut current_position = coord;
        while !Self::is_coord_same(position, current_position) {
            let current_cell_type = Self::get_cell_type(Self::get_cell(map, current_position));

            if current_cell_type == CellType::Destroyable || current_cell_type == CellType::Wall {
                return true;
            }

            if move_row {
                let modifier: isize = if current_position.row > position.row {
                    -1
                } else {
                    1
                };
                let new_row = current_position.row.get() as isize + modifier;

                current_position = Coord::new(current_position.col, Row::new(new_row as usize));
            } else {
                let modifier: isize = if current_position.col > position.col {
                    -1
                } else {
                    1
                };
                let new_col = current_position.col.get() as isize + modifier;

                current_position = Coord::new(Col::new(new_col as usize), current_position.row);
            }
        }

        false
    }

    fn is_coord_same(coord_a: Coord, coord_b:Coord) -> bool {
        return if coord_a.col.get() == coord_b.col.get() && coord_a.row.get() == coord_b.row.get() {
            true
        } else {
            false
        }
    }

    fn get_tile_score(
        map: &Map,
        coord: Coord,
        enemy_positions: &Vec<Coord>,
        current_position: Coord,
        map_settings: &MapSettings,
    ) -> f64 {
        let cell_type = Self::get_cell_type(Self::get_cell(map, coord));

        if !(cell_type == CellType::Empty || cell_type == CellType::Destroyable) {
            return f64::NEG_INFINITY; // not valid
        }

        let danger_level = Self::danger_level_at(coord, map, map_settings.bombradius);

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


        let closeness_score = Self::closeness(
            new_distance,
            map_settings.width - 1 + map_settings.height - 1,
        );
        //
        // let break_penalty = if cell_type == CellType::Destroyable {
        //     if Self::can_escape_bomb(map, coord, map_settings.bombradius, map_settings.bombtimer) {
        //         0.3
        //     } else {
        //         return 0.0;
        //     }
        // } else {
        //     0.0
        // };

        let escape_routes = [
            coord.move_up(),
            coord.move_down(),
            coord.move_left(),
            coord.move_right(),
        ]
        .iter()
        .filter_map(|&opt| opt)
        .filter(|&n| Self::get_cell_type(Self::get_cell(map, n)) == CellType::Empty)
        .count();

        let escape_score = (escape_routes as f64) / 4.0;

        let final_score =
            closer * 0.3 + escape_score * 0.2 - danger_level * 0.4 ;

        final_score
    }



    // fn can_escape_bomb(map: &Map, start: Coord, bomb_radius: usize, max_depth: usize) -> bool {
    //     use std::collections::VecDeque;
    //     let mut visited = std::collections::HashSet::new();
    //     let mut queue = VecDeque::new();
    //     queue.push_back((start, 0));
    //
    //     while let Some((pos, depth)) = queue.pop_front() {
    //         if depth > max_depth {
    //             continue;
    //         }
    //
    //         if !Self::is_in_bomb_range(pos, start, map, bomb_radius) {
    //             return true;
    //         }
    //
    //         visited.insert(pos);
    //
    //         for neighbour in Self::get_empty_neighbours(map, pos) {
    //             if !visited.contains(&neighbour) {
    //                 queue.push_back((neighbour, depth + 1));
    //             }
    //         }
    //     }
    //
    //     false
    // }

    fn is_in_bomb_range(position: Coord, bomb_position: Coord, map: &Map, radius: usize) -> bool {
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

    fn get_empty_neighbours(map: &Map, position: Coord) -> Vec<Coord> {
        Self::get_moves_to_neighbour_coords(position)
            .iter()
            .filter_map(|(coord, _command)| {
                if Self::get_cell_type(Self::get_cell(map, *coord)) == CellType::Empty {
                    Some(*coord)
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_moves_to_neighbour_coords(position: Coord) -> Vec<(Coord, Command)> {
        let directions = [
            (position.move_up(), Command::Up),
            (position.move_down(), Command::Down),
            (position.move_left(), Command::Left),
            (position.move_right(), Command::Right),
        ];

        let valid_neighbors: Vec<(Coord, Command)> = directions
            .into_iter()
            .filter_map(|(coord_opt, command)| coord_opt.map(|coord| (coord, command)))
            .collect();

        valid_neighbors
    }
}
