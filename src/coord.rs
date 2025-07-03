use std::ops::AddAssign;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Row(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Col(usize);

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

impl Add<usize> for Col {
    type Output = Col;
    fn add(self, rhs: usize) -> Col {
        Col(self.0 + rhs)
    }
}

impl Sub<usize> for Col {
    type Output = Col;
    fn sub(self, rhs: usize) -> Col {
        Col(self.0 - rhs)
    }
}

impl Add<usize> for Row {
    type Output = Row;
    fn add(self, rhs: usize) -> Row {
        Row(self.0 + rhs)
    }
}

impl Add<RowRelative> for Row {
    type Output = Row;
    fn add(self, other: RowRelative) -> Row {
        let mut new_val = self.0 as isize + other.0;
        if new_val < 0 {
            new_val = 0;
        }

        Row(new_val as usize)
    }
}

impl Add<ColRelative> for Col {
    type Output = Col;
    fn add(self, other: ColRelative) -> Col {
        let mut new_val = self.0 as isize + other.0;
        if new_val < 0 {
            new_val = 0;
        }
        Col(new_val as usize)
    }
}

impl Sub<usize> for Row {
    type Output = Row;
    fn sub(self, rhs: usize) -> Row {
        Row(self.0 - rhs)
    }
}

impl Into<usize> for Row {
    fn into(self) -> usize {
        self.0
    }
}

impl Into<usize> for Col {
    fn into(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct RowRelative(isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ColRelative(isize);

impl From<usize> for Row {
    fn from(value: usize) -> Self {
        Row(value)
    }
}

impl From<usize> for Col {
    fn from(value: usize) -> Self {
        Col(value)
    }
}

impl AddAssign<ColRelative> for Col {
    fn add_assign(&mut self, other: ColRelative) {
        let new_val = self.0 as isize + other.0;
        assert!(new_val >= 0, "Col cannot be negative");
        self.0 = new_val as usize;
    }
}

impl AddAssign<RowRelative> for Row {
    fn add_assign(&mut self, other: RowRelative) {
        let new_val = self.0 as isize + other.0;
        assert!(new_val >= 0, "Col cannot be negative");
        self.0 = new_val as usize;
    }
}

impl From<isize> for RowRelative {
    fn from(value: isize) -> Self {
        RowRelative(value)
    }
}

impl From<isize> for ColRelative {
    fn from(value: isize) -> Self {
        ColRelative(value)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    pub fn try_move_relative_to(&self, col: ColRelative, row: RowRelative) -> Option<Coord> {
        let new_col = self.col.add(col);
        let new_row = self.row.add(row);
        if new_col != self.col || new_row != self.row {
            Some(Coord {
                col: new_col,
                row: new_row,
            })
        } else {
            None
        }
    }

    pub fn move_relative_to(&self, col: ColRelative, row: RowRelative) -> Coord {
        let new_coord = Coord {
            row: self.row.add(row),
            col: self.col.add(col),
        };
        new_coord
    }

    pub fn move_up(&self) -> Coord {
        self.move_relative_to(ColRelative(0), RowRelative(-1))
    }

    pub fn try_move_up(&self) -> Option<Coord> {
        self.try_move_relative_to(ColRelative(0), RowRelative(-1))
    }

    pub fn move_down(&self) -> Coord {
        self.move_relative_to(ColRelative(0), RowRelative(1))
    }

    pub fn try_move_down(&self) -> Option<Coord> {
        self.try_move_relative_to(ColRelative(0), RowRelative(1))
    }

    pub fn move_left(&self) -> Coord {
        self.move_relative_to(ColRelative(-1), RowRelative(0))
    }

    pub fn try_move_left(&self) -> Option<Coord> {
        self.try_move_relative_to(ColRelative(-1), RowRelative(0))
    }

    pub fn move_right(&self) -> Coord {
        self.move_relative_to(ColRelative(1), RowRelative(0))
    }

    pub fn try_move_right(&self) -> Option<Coord> {
        self.try_move_relative_to(ColRelative(1), RowRelative(0))
    }

    /// Returns a vector of coordinates representing a 3x3 square centered at the current coordinate.
    /// Useful to clear a 3x3 area around the current coordinate, where a player starts.
    pub fn square_3x3(&self) -> Vec<Coord> {
        let mut coords = Vec::new();
        for row_change in -1..=1 {
            for col_change in -1..=1 {
                let new_coord =
                    self.move_relative_to(ColRelative(col_change), RowRelative(row_change));
                coords.push(new_coord);
            }
        }
        coords
    }

    pub fn is_valid(&self, width: usize, height: usize) -> bool {
        self.col.0 < width && self.row.0 < height && self.col.0 >= 0 && self.row.0 >= 0
    }
}
