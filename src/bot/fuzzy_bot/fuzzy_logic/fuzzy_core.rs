use crate::bot::fuzzy_bot::fuzzy_logic::manhattan::manhattan;
use crate::coord::{Col, Coord, Row};
use crate::map::{Bomb, CellType, Map};

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

            let in_row = position.row == bomb_position.row;
            let in_col = position.col == bomb_position.col;

            if in_row || in_col {
                let distance_to_bomb = manhattan(position, bomb_position);

                if radius as u32 * 3 > distance_to_bomb as u32 {
                    let blocked_by_wall = if in_row {
                        Self::is_blocked_by_wall(map, bomb, position, true)
                    } else {
                        Self::is_blocked_by_wall(map, bomb, position, false)
                    };

                    if !blocked_by_wall {
                        let relevant_distance = radius * 3;
                        let dist_score =
                            FuzzyLogic::closeness(distance_to_bomb, relevant_distance);
                        let time_score = FuzzyLogic::danger_level(bomb.timer) as f64;
                        total_danger += (dist_score * time_score).clamp(0.0, 1.0);
                    }
                }
            }
        }
        total_danger.clamp(0.0, 1.0)
    }


    pub fn closeness(distance_to_player: i32,  relevant_distance: usize) -> f64 {
        let normalized = 1.0 - (distance_to_player as f64 / relevant_distance as f64);

        normalized.clamp(0.0, 1.0)
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
            'D' => CellType::Destroyable,
            _ => CellType::Wall,
        }
    }

    fn is_blocked_by_wall(map: &Map, bomb: &Bomb, position: Coord, move_row: bool) -> bool {
        let mut current_position = bomb.position;
        while current_position != position {
            let current_cell_type = Self::get_cell_type(Self::get_cell(map, current_position));

            if current_cell_type == CellType::Destroyable || current_cell_type == CellType::Wall {
                return true;
            }

            if move_row {
                let modifier: isize = if current_position.row > position.row { -1 } else { 1 };
                let new_row = current_position.row.get() as isize + modifier;

                let new_row = new_row.max(0) as usize;

                current_position = Coord::new(
                    current_position.col,
                    Row::new(new_row),
                );
            } else {
                let modifier: isize = if current_position.col > position.col { -1 } else { 1 };
                let new_col = current_position.col.get() as isize + modifier;

                let new_col = new_col.max(0) as usize;

                current_position = Coord::new(
                    Col::new(new_col),
                    current_position.row
                );
            }
        }

        false
    }
}
