use crate::coord::Coord;

#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub position: Coord,
    pub id: usize,
    alive: bool
}

impl Player {
    pub fn new(name: String, position: Coord, id: usize) -> Player {
        Player {
            name: name.to_string(),
            position,
            id,
            alive: true,
        }
    }

    pub(crate) fn move_position(&mut self, new_position: Coord) {
        self.position = new_position;
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub(crate) fn kill(&mut self) {
        self.alive = false;
    }
}
