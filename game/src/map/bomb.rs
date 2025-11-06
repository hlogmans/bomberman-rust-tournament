use crate::coord::Coord;

// a bomb item has a position on the map, and a timer that counts down to explosion.
#[derive(Clone, Debug)]
pub struct Bomb {
    pub position: Coord, // (row, column)
    pub timer: usize,    // counts down to explosion
}
