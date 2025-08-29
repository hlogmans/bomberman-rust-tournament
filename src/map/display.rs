use crate::coord::Coord;

pub trait MapDisplay {
    fn display(&self, map: &crate::map::Map);
}

pub struct ConsoleDisplay;

impl MapDisplay for ConsoleDisplay {
    fn display(&self, map: &crate::map::Map) {
        for y in 0..map.height {
            for x in 0..map.width {
                let cell = map.cell_type(Coord::from(x, y));
                let symbol = match cell {
                    crate::map::cell::CellType::Empty => "  ",
                    crate::map::cell::CellType::Wall => "🟥",
                    crate::map::cell::CellType::Destroyable => "D ",
                    crate::map::cell::CellType::Bomb => "💣",
                    crate::map::cell::CellType::Player => "😀",
                };
                print!("{symbol}");
            }
            println!();
        }
    }
}
