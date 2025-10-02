#[derive(Clone, PartialEq, Debug)]
pub enum CellType {
    Empty,       // ' '
    Bomb,        // 'B'
    Wall,        // 'W'
    Player,      // 'P'
    Destroyable, // '.'
}

impl CellType {
    pub fn as_char(&self) -> char {
        match self {
            CellType::Empty => ' ',
            CellType::Bomb => 'B',
            CellType::Wall => 'W',
            CellType::Player => 'P',
            CellType::Destroyable => '.',
        }
    }

    pub fn from_char(c: char) -> Self {
        match c {
            ' ' => CellType::Empty,
            'B' => CellType::Bomb,
            'W' => CellType::Wall,
            'P' => CellType::Player,
            '.' => CellType::Destroyable,
            _ => CellType::Empty, // fallback
        }
    }
}
