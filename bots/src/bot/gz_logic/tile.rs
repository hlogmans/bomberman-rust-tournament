use game::coord::Coord;
use game::map::grid::cell::CellType;

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
    
    pub(crate) fn to_string(self) -> String {
        return format!("{}:{}", self.coord.col.get(), self.coord.row.get());
    }
}
