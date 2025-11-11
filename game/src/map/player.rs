use std::usize;

use crate::coord::Coord;

#[derive(Clone, Debug)]
pub struct Player {
    pub name: String,
    pub position: Coord,
    pub id: usize,
    alive: bool,
    pub reason_killed: String,
    pub killed_by: usize
}

impl Player {
    pub fn new(name: String, position: Coord, id: usize) -> Player {
        Player {
            name: name.to_string(),
            position,
            id,
            alive: true,
            reason_killed: "".to_string(),
            killed_by: usize::MAX
        }
    }

    pub(crate) fn move_position(&mut self, new_position: Coord) {
        self.position = new_position;
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub(crate) fn kill(&mut self, reason_killed: &String, killed_by: usize) {
        self.alive = false;
        if killed_by == self.id {
            self.reason_killed = "suicide".to_string()
        }else {
            self.reason_killed = reason_killed.clone();
        }
        self.killed_by = killed_by;
    }
}
