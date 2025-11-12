use crate::bot::gz_logic::helper;
use crate::bot::gz_logic::tile::Tile;
use crate::bot::gz_logic::tilemap::TileMap;
use game::bot::bot::Bot;
use game::coord::{Col, Coord, Row};
use game::map::bomb::Bomb;
use game::map::grid::cell::CellType;
use game::map::enums::command::Command;
use game::map::map::Map;
use game::map::structs::map_config::MapConfig;
use std::collections::{HashMap, VecDeque};

#[derive(Clone)]
pub struct GzBot {
    pub id: usize,
    pub debug_info: String,
    current_target: Option<Coord>,
    fleeing: bool,
    map_settings: MapConfig,
}

impl GzBot {
    pub fn new() -> Self {
        GzBot {
            id: 0,
            debug_info: "".to_string(),
            current_target: None,
            fleeing: false,
            map_settings: MapConfig::default(),
        }
    }

    fn generate_tile_map(&self, map: &Map, player_position: Coord) -> TileMap {
        let mut tile_map: Vec<Vec<Tile>> = Vec::new();

        for (index, char) in map.grid.tiles.iter().enumerate() {
            let coord: Coord = self.get_coord_from_index(index);
            let mut cell_type = helper::get_cell_type(char);
            if coord == player_position {
                cell_type = CellType::Empty;
            }
            let tile: Tile = Tile {
                coord,
                cell_type,
                visited: false,
                safe: self.is_tile_safe(map, coord),
            };

            while tile_map.len() <= coord.row.get() {
                tile_map.push(Vec::new());
            }

            tile_map[coord.row.get()].push(tile)
        }

        TileMap { map: tile_map }
    }

    fn score_tile(&self, map: &TileMap, tile: &Tile, distance: usize) -> i32 {
        let mut score = 0;

        for neighbour_coord in helper::get_neighbour_coords(tile.coord) {
            if let Some(neighbour_tile) = map.get(neighbour_coord) {
                match neighbour_tile.cell_type {
                    CellType::Destroyable => score += 6,
                    CellType::Player => score += 30,
                    CellType::Empty => score += 2,
                    _ => {}
                }
            }
        }

        if !tile.safe {
            score -= 50;
        }
        score -= distance as i32;

        return score;
    }

    fn reachable_tiles_with_scores(
        &mut self,
        tile_map: &mut TileMap,
        start: Coord,
    ) -> HashMap<Coord, (i32, usize)> {
        let mut queue: VecDeque<(Coord, usize)> = VecDeque::new();
        let mut scores: HashMap<Coord, (i32, usize)> = HashMap::new();

        for row in tile_map.map.iter_mut() {
            for tile in row.iter_mut() {
                tile.visited = false;
            }
        }

        queue.push_back((start, 0));
        tile_map.map[start.row.get()][start.col.get()].visited = true;

        while let Some((current_coord, dist)) = queue.pop_front() {
            let tile = tile_map.get(current_coord).unwrap();

            let score = self.score_tile(&tile_map, tile, dist);
            scores.insert(current_coord, (score, dist));

            for coord in helper::get_neighbour_coords(tile.coord) {
                if let Some(neighbour_tile) = tile_map.get_mut(coord) {
                    if !neighbour_tile.visited && self.is_tile_walkable(neighbour_tile) {
                        neighbour_tile.visit();
                        queue.push_back((neighbour_tile.coord, dist + 1));
                    }
                }
            }
        }

        scores
    }

    fn is_tile_walkable(&self, tile: &Tile) -> bool {
        match tile.cell_type {
            CellType::Empty => true,
            _ => false,
        }
    }

    fn is_tile_safe(&self, map: &Map, coord: Coord) -> bool {
        for bomb in map.bombs.iter() {
            if helper::is_tile_in_bomb_range(coord, bomb.position, self.map_settings.bomb_radius) {
                return false;
            }
        }

        true
    }

    #[inline(always)]
    fn get_coord_from_index(&self, index: usize) -> Coord {
        let row = index / self.map_settings.size;
        let col = index % self.map_settings.size;

        return Coord {
            col: Col::new(col),
            row: Row::new(row),
        };
    }

    fn try_find_path<'a>(
        &mut self,
        tile_map: &'a TileMap,
        bombs: &Vec<Bomb>,
        player_location: Coord,
        target_location: Coord,
    ) -> Option<(i32, Vec<&'a Tile>)> {
        if let Some(starting_tile) = tile_map.get(player_location)
            && let Some(goal_tile) = tile_map.get(target_location)
        {
            let possible_path = tile_map.dijkstra(
                starting_tile,
                goal_tile,
                bombs,
                self.map_settings.bomb_radius,
            );
            return possible_path;
        }

        return None;
    }

    fn get_flee_location(&mut self, tile_map: &TileMap, current_tile: &Tile) -> Option<Coord> {
        let possible_safe_tile = tile_map.nearest_safe_tile(&current_tile);
        if let Some(safe_tile) = possible_safe_tile {
            return Some(safe_tile.coord);
        }
        None
    }

}

impl Bot for GzBot {
    fn start_game(&mut self, settings: &MapConfig, _bot_name: String, bot_id: usize) -> bool {
        self.id = bot_id;
        self.map_settings = settings.clone();
        true
    }

    fn get_move(&mut self, map: &Map, player_location: Coord) -> Command {
        let mut tile_map = self.generate_tile_map(map, player_location);
        if let Some(current_tile) = tile_map.get(player_location) {
            if !current_tile.safe {
                self.fleeing = true;
                self.current_target = self.get_flee_location(&tile_map, current_tile)
            } else {
                if self.fleeing {
                    self.current_target = None;
                }
                self.fleeing = false;
            }
        }

        if !self.fleeing {
            if let Some(current_tile) = tile_map.get(player_location) {
                let mut high_value_adjacent = false;
                for n in helper::get_neighbour_coords(current_tile.coord) {
                    if let Some(t) = tile_map.get(n) {
                        if t.cell_type == CellType::Destroyable || t.cell_type == CellType::Player {
                            high_value_adjacent = true;
                            break;
                        }
                    }
                }

                if high_value_adjacent {
                    if let Some(escape_tile) = tile_map.nearest_safe_tile(current_tile) {
                        if let Some((_cost, escape_path)) = tile_map.dijkstra(
                            current_tile,
                            escape_tile,
                            &map.bombs,
                            self.map_settings.bomb_radius,
                        ) {
                            if escape_path.len() <= 4 {
                               
                                return Command::PlaceBomb;
                            }
                        }
                    }
                }
            }
        }

        // Score and sort all possible moves
        let mut scored_moves: Vec<(Coord, i32, usize)> = self
            .reachable_tiles_with_scores(&mut tile_map, player_location)
            .into_iter()
            .map(|(coord, (score, dist))| (coord, score, dist))
            .collect();

        scored_moves.sort_by(|a, b| {
            if a.1 != b.1 {
                b.1.cmp(&a.1)
            } else {
                a.2.cmp(&b.2)
            }
        });

        for (target, score, dist) in scored_moves {
            if target == player_location {
                continue;
            }

            if let Some(path) = self.try_find_path(&tile_map, &map.bombs, player_location, target) {
                if path.1.len() > 0 {

                    return helper::get_command_to_move_to_coord(player_location, path.1[0].coord);
                }
            }
        }

        if let Some(current_tile) = tile_map.get(player_location) {
            for n in helper::get_neighbour_coords(current_tile.coord) {
                if let Some(t) = tile_map.get(n) {
                    if t.cell_type == CellType::Empty && t.safe {

                        return helper::get_command_to_move_to_coord(player_location, t.coord);
                    }
                }
            }
        }


        Command::Wait
    }

    fn get_debug_info(&self) -> String {
        return self.debug_info.clone();
    }
}
