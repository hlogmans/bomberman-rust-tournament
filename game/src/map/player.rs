use crate::coord::Coord;

#[derive(Clone, Debug)]
pub struct Player {
    pub name: String,
    pub position: Coord,
    pub id: usize,
    alive: bool,
    killed_by: String
}

impl Player {
    pub fn new(name: String, position: Coord, id: usize) -> Player {
        Player {
            name: name.to_string(),
            position,
            id,
            alive: true,
            killed_by: "".to_string()
        }
    }

    pub(crate) fn move_position(&mut self, new_position: Coord) {
        self.position = new_position;
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub(crate) fn kill(&mut self, killed_by: &String) {
        self.alive = false;
        self.killed_by = killed_by.clone();
    }
}
