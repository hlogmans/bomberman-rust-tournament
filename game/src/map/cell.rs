#[derive(Clone, PartialEq, Debug)]
pub enum CellType {
    Empty,       // ' '
    Wall,        // 'W'
    Destroyable, // '.'
}

impl CellType {
    pub fn as_char(&self) -> char {
        match self {
            CellType::Empty => ' ',
            CellType::Wall => 'W',
            CellType::Destroyable => '.',
        }
    }

    pub fn from_char(c: char) -> Self {
        match c {
            ' ' => CellType::Empty,
            'W' => CellType::Wall,
            '.' => CellType::Destroyable,
            _ => CellType::Empty, // fallback
        }
    }
}
