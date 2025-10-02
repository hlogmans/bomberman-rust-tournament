use std::fmt;

use crate::map::enums::command::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Row(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Col(pub usize);

impl Row {
    pub fn new(row: usize) -> Self {
        Row(row)
    }
    pub fn get(&self) -> usize {
        self.0
    }
}
impl Col {
    pub fn new(col: usize) -> Self {
        Col(col)
    }
    pub fn get(&self) -> usize {
        self.0
    }
}

impl From<(usize, usize)> for Coord {
    fn from((col, row): (usize, usize)) -> Self {
        Coord {
            col: Col(col),
            row: Row(row),
        }
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.col.0, self.row.0)
    }
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.col.0, self.row.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Coord {
    pub col: Col,
    pub row: Row,
}

impl Coord {
    pub fn new(col: Col, row: Row) -> Self {
        Coord { col, row }
    }

    pub fn from(col: usize, row: usize) -> Self {
        Coord {
            col: Col(col),
            row: Row(row),
        }
    }

    fn get_relative_cell(&self, col_offset: isize, row_offset: isize) -> Option<Coord> {
        let new_row = self.row.0 as isize + row_offset;
        let new_col = self.col.0 as isize + col_offset;
        if new_row >= 0 && new_col >= 0 {
            Some(Coord {
                row: Row(new_row as usize),
                col: Col(new_col as usize),
            })
        } else {
            None
        }
    }

    pub fn move_up(&self) -> Option<Coord> {
        self.get_relative_cell(0, -1)
    }

    pub fn move_down(&self) -> Option<Coord> {
        self.get_relative_cell(0, 1)
    }

    pub fn move_left(&self) -> Option<Coord> {
        self.get_relative_cell(-1, 0)
    }

    pub fn move_right(&self) -> Option<Coord> {
        self.get_relative_cell(1, 0)
    }

    pub fn move_command(&self, command: Command) -> Option<Coord> {
        match command {
            Command::Up => self.move_up(),
            Command::Down => self.move_down(),
            Command::Left => self.move_left(),
            Command::Right => self.move_right(),
            Command::PlaceBomb => Some(*self),
            Command::Wait => Some(*self),
        }
    }

    /// Returns a vector of coordinates representing a 3x3 square centered at the current coordinate.
    /// Useful to clear a 3x3 area around the current coordinate, where a player starts.
    pub fn square_3x3(&self) -> Vec<Coord> {
        let mut coords = Vec::new();
        for row_change in -1..=1 {
            for col_change in -1..=1 {
                let new_coord = self.get_relative_cell(col_change, row_change);
                coords.extend(new_coord);
            }
        }
        coords
    }

    pub fn is_valid(&self, width: usize, height: usize) -> bool {
        self.col.0 < width && self.row.0 < height
    }

    pub fn valid(&self, width: usize, height: usize) -> Option<Coord> {
        if self.is_valid(width, height) {
            Some(*self)
        } else {
            None
        }
    }
}

pub trait ValidCoord {
    fn valid(&self, width: usize, height: usize) -> Option<Coord>;
}

impl ValidCoord for Option<Coord> {
    fn valid(&self, width: usize, height: usize) -> Option<Coord> {
        self.and_then(|c| c.valid(width, height))
    }
}
