use crate::coord::Coord;

// a player has a name, and a position on the map.
pub struct Player {
    pub name: String,
    pub position: Coord, // (row, column)
}
