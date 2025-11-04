use game::coord::Coord;
use game::map::cell::CellType;

#[derive(Clone, Debug, PartialEq)]
pub struct Tile {
    pub(crate) coord: Coord,
    pub(crate) cell_type: CellType,
    pub(crate) visited: bool,
    pub(crate) safe: bool,
}

impl Tile {
    pub(crate) fn visit(&mut self) {
        self.visited = true;
    }

    pub(crate) fn reset(&mut self) {
        self.visited = false;
    }
}
