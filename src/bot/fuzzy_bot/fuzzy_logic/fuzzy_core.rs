use crate::bot::fuzzy_bot::fuzzy_logic::fuzzy_input::FuzzyInput;
use crate::bot::fuzzy_bot::fuzzy_logic::helper;
use crate::bot::fuzzy_bot::fuzzy_logic::manhattan::manhattan;
use crate::coord::{Col, Coord, Row};
use crate::game::map_settings::MapSettings;
use crate::map::{Bomb, CellType, Command, Map};

pub struct FuzzyLogic;

impl FuzzyLogic {
    pub fn danger_level(timer: usize, max_timer: usize) -> f32 {
        (max_timer as f32 / timer as f32).clamp(0.0, 1.0)
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
                        let time_score =
                            Self::danger_level(bomb.timer, map.map_settings.bombtimer) as f64;
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

    pub fn get_bomb_score(input: FuzzyInput) -> f64{
        let closeness_score = Self::closeness(
            manhattan(input.current_position, Self::get_closest_player_coords(input.map, input.bot_name, input.bot_id, input.current_position)),
            input.map_settings.width - 1 + input.map_settings.height - 1,
        );


    }

  

    pub fn get_safest_move(input: FuzzyInput) -> Command {
        let neighbours = Self::get_empty_neighbours(input.map ,input.current_position);
        let mut lowest_danger_score = 2.0;
        let mut command = Command::Wait;

        for neighbour in neighbours {
            let score =
                Self::danger_level_at(neighbour, input.map, input.map_settings.bombradius);
            if score < lowest_danger_score {
                lowest_danger_score = score;
                command = Self::get_command_for_coord(input.current_position, neighbour);
            }
        }

        command
    }

   

    fn is_blocked_by_wall(map: &Map, bomb: &Bomb, position: Coord, move_row: bool) -> bool {
        Self::is_coord_blocked_by_wall(map, bomb.position, position, move_row)
    }

    fn is_coord_blocked_by_wall(map: &Map, coord: Coord, position: Coord, move_row: bool) -> bool {
        let mut current_position = coord;
        while !Self::is_coord_same(position, current_position) {
            let current_cell_type = helper::get_cell_type(helper::get_cell(map, current_position));

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

    fn is_coord_same(coord_a: Coord, coord_b: Coord) -> bool {
        return if coord_a.col.get() == coord_b.col.get() && coord_a.row.get() == coord_b.row.get() {
            true
        } else {
            false
        };
    }



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

    fn get_closest_player_coords(map: &Map, bot_name: String, bot_id: usize, my_position: Coord) -> Coord {
        let mut closest = 999999;
        let name = (bot_name + " (" + &bot_id.to_string() + ")").to_string();
        let mut coords = my_position;

        for player in &map.players {
            if player.name != name {
                let distance_to_player = manhattan(my_position, player.position);

                if distance_to_player < closest {
                    closest = distance_to_player;
                    coords = player.position
                }
            }
        }

        coords
    }

    fn get_empty_neighbours(map: &Map, position: Coord) -> Vec<Coord> {
        Self::get_neighbour_coords(position)
            .iter()
            .filter_map( | coord| {
                if helper::get_cell_type(helper::get_cell(map, *coord)) == CellType::Empty {
                    Some(*coord)
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_neighbour_coords(current_pos: Coord) -> Vec<Coord> {
        vec![
            current_pos.move_left().unwrap(),
            current_pos.move_right().unwrap(),
            current_pos.move_up().unwrap(),
            current_pos.move_down().unwrap(),
        ]
    }


}
