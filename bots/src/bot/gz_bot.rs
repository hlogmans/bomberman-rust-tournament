use crate::bot::fuzzy_logic::fuzzy_ai::{decide, handle_intent};
use crate::bot::fuzzy_logic::fuzzy_input::FuzzyInput;
use crate::bot::fuzzy_logic::helper;
use crate::ml_bot::MlBot;
use game::bot::bot::Bot;
use game::coord::{Col, Coord, Row};
use game::map::cell::CellType;
use game::map::enums::command::Command;
use game::map::map::Map;
use game::map::structs::map_config::MapConfig;
use rand::prelude::IndexedRandom;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::fmt::{Display, Pointer, format};
use std::io::empty;
use game::map::bomb::Bomb;

#[derive(Clone)]
pub struct GzBot {
    pub name: String,
    pub id: usize,
    pub debug_info: String,
    current_target: Option<Coord>,
    fleeing: bool,
    map_settings: MapConfig,
}

struct TileMap {
    // [Row][Col]
    map: Vec<Vec<Tile>>,
}

impl TileMap {
    fn get_mut(&mut self, coord: Coord) -> Option<&mut Tile> {
        self.map
            .get_mut(coord.row.get())
            .and_then(|row| row.get_mut(coord.col.get()))
    }

    fn get(&self, coord: Coord) -> Option<&Tile> {
        self.map
            .get(coord.row.get())
            .and_then(|row| row.get(coord.col.get()))
    }

    fn reset(&mut self) {
        for row in self.map.iter_mut() {
            for tile in row {
                tile.reset()
            }
        }
    }

    fn dijkstra<'a>(&'a self, start: &Tile, goal: &'a Tile, bombs: &Vec<Bomb>, bomb_radius: usize) -> Option<(i32, Vec<&'a Tile>)> {
        let mut dist: HashMap<Coord, i32> = HashMap::new();
        let mut came_from: HashMap<Coord, Coord> = HashMap::new();
        let mut heap = BinaryHeap::new();

        dist.insert(start.coord, 0);
        heap.push(Reverse((0, start.coord)));

        while let Some(Reverse((cost_so_far, current_coord))) = heap.pop() {
            if current_coord == goal.coord {
                let mut path = Vec::new();
                let mut current = goal;

                while current.coord != start.coord {
                    path.push(current);
                    if let Some(&prev_coord) = came_from.get(&current.coord) {
                        if let Some(prev_tile) = self.get(prev_coord) {
                            current = prev_tile;
                        } else {
                            break; // safety, shouldn't happen
                        }
                    } else {
                        break; // reached the start or path incomplete
                    }
                }
                path.reverse();
                return Some((cost_so_far, path));
            }

            if let Some(&best) = dist.get(&current_coord) {
                if cost_so_far > best {
                    continue;
                }
            }

            for next_coord in helper::get_neighbour_coords(current_coord) {
                let next_cost = cost_so_far + 1;

                if next_cost < *dist.get(&next_coord).unwrap_or(&i32::MAX) {
                    if let Some(neighbour_tile) = self.get(next_coord) {
                        if neighbour_tile.cell_type == CellType::Empty && helper::is_tile_currently_safe(bombs,neighbour_tile.coord, next_cost as usize, bomb_radius) {
                            dist.insert(next_coord, next_cost);
                            came_from.insert(next_coord, current_coord);
                            heap.push(Reverse((next_cost, next_coord)));
                        }
                    }
                }
            }
        }

        None
    }

    fn nearest_safe_tile(&self, start: &Tile) -> Option<&Tile> {
        let mut dist: HashMap<Coord, i32> = HashMap::new();
        let mut came_from: HashMap<Coord, Coord> = HashMap::new();
        let mut heap = BinaryHeap::new();

        dist.insert(start.coord, 0);
        heap.push(Reverse((0, start.coord)));

        while let Some(Reverse((cost_so_far, current_coord))) = heap.pop() {
            if let Some(current_tile) = self.get(current_coord) {
                if current_tile.safe {
                    return Some(current_tile);
                }
            }

            if let Some(&best) = dist.get(&current_coord) {
                if cost_so_far > best {
                    continue;
                }
            }

            for next_coord in helper::get_neighbour_coords(current_coord) {
                let next_cost = cost_so_far + 1;

                if next_cost < *dist.get(&next_coord).unwrap_or(&i32::MAX) {
                    if let Some(neighbour_tile) = self.get(next_coord) {
                        if neighbour_tile.cell_type == CellType::Empty {
                            dist.insert(next_coord, next_cost);
                            came_from.insert(next_coord, current_coord);
                            heap.push(Reverse((next_cost, next_coord)));
                        }
                    }
                }
            }
        }

        None
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Tile {
    coord: Coord,
    cell_type: CellType,
    visited: bool,
    safe: bool,
}

impl Tile {
    fn visit(&mut self) {
        self.visited = true;
    }

    fn reset(&mut self) {
        self.visited = false;
    }
}

impl GzBot {
    pub fn new() -> Self {
        GzBot {
            name: "GeleZuivelBot".to_string(),
            id: 0,
            debug_info: "".to_string(),
            current_target: None,
            fleeing: false,
            map_settings: MapConfig::default(),
        }
    }

    fn generate_tile_map(&self, map: &Map) -> TileMap {
        let mut tile_map: Vec<Vec<Tile>> = Vec::new();

        for (index, char) in map.grid.tiles.iter().enumerate() {
            let coord: Coord = self.get_coord_from_index(index);
            let tile: Tile = Tile {
                coord: coord,
                cell_type: helper::get_cell_type(char),
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

        // Example heuristics:
        score -= (distance / 5) as i32;
        if tile.safe {
            score += 10
        }
        for neighbour_coord in helper::get_neighbour_coords(tile.coord) {
            if let Some(neighbour_tile) = map.get(neighbour_coord) {
                match neighbour_tile.cell_type {
                    CellType::Destroyable => score += 2,
                    CellType::Player => score += 10,
                    _ => {}
                }
            }
        }

        score
    }

    fn reachable_tiles_with_scores(
        &self,
        tile_map: &mut TileMap,
        start: Coord,
    ) -> HashMap<Coord, i32> {
        let mut queue: VecDeque<(Coord, usize)> = VecDeque::new();
        let mut scores: HashMap<Coord, i32> = HashMap::new();

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
            scores.insert(current_coord, score);

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
            if helper::is_tile_in_bomb_range(
                coord,
                bomb.position,
                self.map_settings.bomb_radius,
            ) {
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

    fn choose_coord_to_move_to(&mut self, tile_map: &mut TileMap, player_location: Coord) -> Coord {
        let mut scores: Vec<(Coord, i32)> = self
            .reachable_tiles_with_scores(tile_map, player_location)
            .into_iter()
            .collect();

        scores.sort_by(|a, b| b.1.cmp(&a.1));

        if scores.is_empty() {
            return player_location;
        }

        let highest_score = scores[0].1;

        let top_tiles_with_score: Vec<(Coord, i32)> = scores
            .iter()
            .filter(|(_, score)| *score == highest_score)
            .map(|(coord, score)| (*coord, *score))
            .collect();

        let mut rng = rand::rng();
        let (chosen_coord, chosen_score) = top_tiles_with_score.choose(&mut rng).cloned().unwrap();

        return chosen_coord;
    }
}

impl Bot for GzBot {
    fn start_game(&mut self, settings: &MapConfig, bot_name: String, bot_id: usize) -> bool {
        self.id = bot_id;
        self.name = bot_name;
        self.map_settings = settings.clone();
        true
    }

    fn get_move(&mut self, map: &Map, player_location: Coord) -> Command {

        let mut tile_map = self.generate_tile_map(map);
        if let Some(current_tile) = tile_map.get(player_location) {
            if !current_tile.safe {
                self.fleeing = true;
                let possible_safe_tile = tile_map.nearest_safe_tile(&current_tile);
                if let Some(safe_tile) = possible_safe_tile {

                    self.current_target = Some(safe_tile.coord);
                    tile_map.reset();
                }
            }
            else {

                if self.fleeing {
                    self.fleeing = false;
                    self.current_target = None;
                }
            }
        }

        if self.current_target.is_none() {
            self.current_target =
                Some(self.choose_coord_to_move_to(&mut tile_map, player_location));

        }




        if let Some(starting_tile) = tile_map.get(player_location) {
            if let Some(goal_tile) = tile_map.get(self.current_target.unwrap()) {
                let possible_path = tile_map.dijkstra(starting_tile, goal_tile, &map.bombs, self.map_settings.bomb_radius);
                if let Some(path) = possible_path {
                    if path.1.len() > 0 {
                        let goal = path.1.first().unwrap();
                        return helper::get_command_to_move_to_coord(player_location, goal.coord);
                    }
                    self.current_target = None;
                    if !self.fleeing {
                        return Command::PlaceBomb;
                    }
                }
            }
        }



        return Command::Wait;
    }

    fn get_debug_info(&self) -> String {
        return self.debug_info.clone();
    }
}
