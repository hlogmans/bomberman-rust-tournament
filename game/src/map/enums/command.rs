#[derive(Debug, Clone, Copy)]
pub enum Command {
    Up,
    Down,
    Left,
    Right,
    Wait,
    PlaceBomb,
}

impl Command {
    // is this a move command? False if the position won't change.
    pub fn is_move(&self) -> bool {
        match self {
            Command::Up | Command::Down | Command::Left | Command::Right => true,
            Command::Wait | Command::PlaceBomb => false,
        }
    }
}