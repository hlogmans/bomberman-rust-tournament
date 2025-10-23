use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Pointer};
use game::bot::bot::Bot;
use game::coord::{Col, Coord, Row};
use game::map::cell::CellType;
use game::map::enums::command::Command;
use game::map::map::Map;
use game::map::structs::map_config::MapConfig;
use crate::bot::fuzzy_logic::fuzzy_ai::{decide, handle_intent};
use crate::bot::fuzzy_logic::fuzzy_input::FuzzyInput;
use crate::bot::fuzzy_logic::helper;

#[derive(Clone)]
pub struct GeleZuivelBot {
    pub name: String,
    pub id: usize,
    map_settings: MapConfig,
}

struct TileMap {
    // [Row][Col]
    map: Vec<Vec<Tile>>
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
}

#[derive(Clone, Debug, PartialEq,)]
struct Tile {
    coord: Coord,
    cell_type: CellType,
    visited: bool,
    safe: bool
}

impl Tile {
    fn visit(&mut self){
        self.visited = true;
    }

    fn reset(&mut self){
        self.visited = false;
    }
}

impl GeleZuivelBot {
    pub fn new() -> Self {
        GeleZuivelBot {
            name: "GeleZuivelBot".to_string(),
            id: 0,
            map_settings: MapConfig::default()
        }
    }

    fn generate_tile_map(&self, map: &Map) -> TileMap {
        let mut tile_map: Vec<Vec<Tile>> = Vec::new();

        for (index, char) in map.grid.iter().enumerate() {
            let coord: Coord = self.get_coord_from_index(index);
            let tile: Tile = Tile {coord: coord, cell_type: helper::get_cell_type(char), visited: false, safe:self.is_tile_safe(map, coord)};
            while tile_map.len() <= coord.row.get() {
                tile_map.push(Vec::new());
            }

            tile_map[coord.row.get()].push(tile)
        }

        TileMap {map: tile_map}

    }


    fn score_tile(&self, tile: &Tile, distance: usize) -> i32 {
        let mut score = 0;

        // Example heuristics:
        score -= distance as i32;          // prefer closer tiles
        if tile.safe { score += 10 }       // safe tiles are better
        match tile.cell_type {
            CellType::Wall => score += 5,
            CellType::Player => score += 2,
            _ => {}
        }

        score
    }


    fn reachable_tiles_with_scores(&self,tile_map: &mut TileMap, start: Coord) -> HashMap<Coord, i32> {
        let mut queue: VecDeque<(Coord, usize)> = VecDeque::new(); // coord + distance
        let mut scores: HashMap<Coord, i32> = HashMap::new();

        // mark all tiles unvisited
        for row in tile_map.map.iter_mut() {
            for tile in row.iter_mut() { tile.visited = false; }
        }

        queue.push_back((start, 0));
        tile_map.map[start.row.get()][start.col.get()].visited = true;

        while let Some((current_coord, dist)) = queue.pop_front() {
            let tile = tile_map.get(current_coord).unwrap();

            // compute score
            let score = self.score_tile(tile, dist);
            scores.insert(current_coord, score);

            // enqueue neighbors
            for coord in helper::get_neighbour_coords(tile.coord){
                let neighbour_tile = tile_map.get_mut(coord).unwrap();
                if neighbour_tile.visited != true {
                    neighbour_tile.visit();
                    queue.push_back((neighbour_tile.coord, dist + 1))
                }
            }
        }

        scores
    }

    fn is_tile_safe(&self, map: &Map, coord: Coord) -> bool {
        for bomb in map.bombs.iter() {
            if helper::is_tile_in_bomb_range(coord, bomb.position, map, self.map_settings.bomb_radius){
                return false
            }
        }

        true
    }


    #[inline(always)]
    fn get_coord_from_index(&self, index: usize) -> Coord {
        let row = index / self.map_settings.width;
        let col = index % self.map_settings.width;

        return Coord {col: Col::new(col), row: Row::new(row)};
    }
}

impl Bot for GeleZuivelBot {
    fn name(&self) -> String {
        format!("{} ({})", self.name, self.id)
    }

    fn start_game(&mut self, map_settings: &MapConfig, bot_id: usize) -> bool {
        self.id = bot_id;
        self.map_settings = map_settings.clone();
        true
    }

    fn get_debug_info(&self) -> String {
        todo!()
    }

    fn get_move(&mut self, map: &Map, player_location: Coord) -> Command {
        self.generate_tile_map(map, player_location);
        return Command::Wait;
    }

}