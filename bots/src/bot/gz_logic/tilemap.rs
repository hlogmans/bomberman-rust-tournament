use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use game::coord::Coord;
use game::map::bomb::Bomb;
use game::map::cell::CellType;
use crate::bot::gz_logic::helper;
use crate::bot::gz_logic::tile::Tile;

pub struct TileMap {
    // [Row][Col]
    pub(crate) map: Vec<Vec<Tile>>,
}

impl TileMap {
    pub(crate) fn get_mut(&mut self, coord: Coord) -> Option<&mut Tile> {
        self.map
            .get_mut(coord.row.get())
            .and_then(|row| row.get_mut(coord.col.get()))
    }

    pub(crate) fn get(&self, coord: Coord) -> Option<&Tile> {
        self.map
            .get(coord.row.get())
            .and_then(|row| row.get(coord.col.get()))
    }

    pub(crate) fn reset(&mut self) {
        for row in self.map.iter_mut() {
            for tile in row {
                tile.reset()
            }
        }
    }

    pub(crate) fn dijkstra<'a>(&'a self, start: &Tile, goal: &'a Tile, bombs: &Vec<Bomb>, bomb_radius: usize) -> Option<(i32, Vec<&'a Tile>)> {
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

    pub(crate) fn nearest_safe_tile(&self, start: &Tile) -> Option<&Tile> {
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