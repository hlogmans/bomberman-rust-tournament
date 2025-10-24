use crate::map::{cell::CellType, grid::shrink::calculate_shrink_location};

use crate::coord::Coord;

pub struct Grid {
    pub tiles: Vec<char>,
    size: usize,
}

impl Grid {
    pub fn new(tiles: Vec<char>, size: usize) -> Self {
        Self {
            tiles: tiles,
            size: size
        }
    }

    pub fn cell_type(&self, position: Coord) -> CellType {
        if self.out_of_bounds(&position){
            return CellType::Wall;
        }
        let idx = self.cell_index(&position);
        CellType::from_char(self.tiles[idx])
    }

    pub fn cell_index(&self, position: &Coord) -> usize {
        position.row.get() * self.size + position.col.get()
    }

    pub fn out_of_bounds(&self, position: &Coord) -> bool {
        !position.is_valid(self.size, self.size)
    }

    pub(crate) fn clear_destructable(&mut self, location: Coord) {
        if self.cell_type(location) == CellType::Destroyable {
            self.set_cell(location, CellType::Empty);
        }
    }

    pub(crate) fn set_wall(&mut self, position: Coord) {
        if position.is_valid(self.size, self.size) {
            self.set_cell(position, CellType::Wall);
        }
    }

    pub(crate) fn set_cell(&mut self, position: Coord, cell_type: CellType) {
        if position.is_valid(self.size, self.size) {
            let idx = self.cell_index(&position);
            self.tiles[idx] = cell_type.as_char();
        }
    }

    pub(crate) fn shrink(&mut self, shrink_turn: usize) -> Coord {
        if let Some(shrink_location) = calculate_shrink_location(shrink_turn, self.size) {
            self.set_wall(shrink_location);
            shrink_location
        } else {
            panic!("No valid shrink location found for shrink {}", shrink_turn);
        }
    }

}

