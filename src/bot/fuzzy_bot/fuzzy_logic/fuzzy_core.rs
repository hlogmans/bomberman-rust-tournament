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
                            Self::closeness(distance_to_bomb, relevant_distance);
                        let time_score = Self::danger_level(bomb.timer) as f64;
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


    fn get_tile_score(
        map: &Map,
        coord: Coord,
        enemy_positions: &Vec<Coord>,
        current_position: Coord,
        map_settings: &MapSettings
    ) -> f64 {
        let cell_type = Self::get_cell_type(Self::get_cell(map, coord));

        if cell_type != CellType::Empty && cell_type != CellType::Destroyable {
            return f64::NEG_INFINITY; // not valid
        }

        let closest_enemy = enemy_positions
            .iter()
            .copied()
            .min_by_key(|&e| manhattan(current_position, e))
            .unwrap_or(coord); // fallback to self

        let current_distance = manhattan(current_position, closest_enemy);
        let new_distance = manhattan(coord, closest_enemy);

        let is_closer = (current_distance > new_distance) as u8 as f64;


        let closeness_score = Self::closeness(new_distance, map_settings.width -1 + map_settings.height -1);

        let break_penalty = if cell_type == CellType::Destroyable { 0.3 } else { 0.0 };

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
            is_closer * 0.4 +
                closeness_score * 0.3 +
                escape_score * 0.2 -
                break_penalty * 0.3;

        final_score
    }

    fn handle_move_decision(input: FuzzyInput) -> Command {
        let neighbours = Self::get_moves_to_neighbour_coords(input.current_position);
        let enemy_positions: Vec<Coord> = input.map.players
            .iter()
            .filter_map(|p | {
                if p.name != input.bot_name {
                    Some(p.position)
                }
                else {
                    None
                }
            })
            .collect();

        let mut command = Command::Wait;
        let mut highest_tile_score = 0.0;
        for neighbour in neighbours {
            let tile_score = FuzzyLogic::get_tile_score(input.map, neighbour.0, &enemy_positions, input.current_position, input.map_settings);
            if tile_score > highest_tile_score {
                highest_tile_score = tile_score;
                
                if Self::get_cell_type(Self::get_cell(input.map, neighbour.0)) == CellType::Destroyable {
                    command = Command::PlaceBomb;
                }
                else {
                    command = neighbour.1;
                }
            }
        }

        command
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
            .filter_map(|(coord_opt, command)| {
                coord_opt.map(|coord| (coord, command))
            })
            .collect();

        valid_neighbors
    }
}
