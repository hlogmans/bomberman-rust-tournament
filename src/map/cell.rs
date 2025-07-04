// a enumration of possible blocks
#[derive(Clone, PartialEq)]
pub enum CellType {
    Empty,       // ' '
    Bomb,        // 'B'
    Wall,        // 'W'
    Player,      // 'P'
    Destroyable, // '.'
}
