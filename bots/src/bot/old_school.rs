use rand::Rng;
use game::bot::bot::Bot;
use game::coord::Coord;
use game::map::enums::command::Command;
use game::map::map::Map;
use game::map::structs::map_config::MapConfig;

pub struct OldSchool {
    name: String,
    id: usize,
    debug_info: String,
    map_settings: MapConfig,
    nextmoves: Vec<Command>,
    target: usize,
}

impl Default for OldSchool {
    fn default() -> Self {
        Self::new()
    }
}

impl OldSchool {
    pub fn new() -> Self {
        OldSchool {
            name: "OldSchool".to_string(),
            id: 0,
            debug_info: "".to_string(),
            map_settings: MapConfig::default(),
            nextmoves: Vec::new(),
            target: 0,
        }
    }
}

impl Bot for OldSchool {

    fn start_game(&mut self, settings: &MapConfig, bot_name: String, bot_id: usize) -> bool {
        self.id = bot_id;
        self.name = bot_name;
        self.map_settings = settings.clone();
        false
    }

    fn get_move(&mut self, map: &Map, me: Coord) -> Command {
        let row = me.row.get() as isize;
        let column = me.col.get() as isize;

        let max_row = (self.map_settings.size - 2) as isize;
        let max_column = (self.map_settings.size - 2) as isize;

        if !self.nextmoves.is_empty() {
            return self.nextmoves.pop().unwrap();
        }

        if column == 1 && row == 1 {
            self.target = 1;
            self.nextmoves.clear();
        } else if column == max_column && row == 1 {
            self.target = 2;
            self.nextmoves.clear();
        } else if column == max_column && row == max_row {
            self.target = 3;
            self.nextmoves.clear();
        } else if column == 1 && row == max_row {
            self.target = 4;
            self.nextmoves.clear();
        }

        self.debug_info = format!(
            "T{} from r{}, c{}",
            self.target, row, column
        );

        if self.target == 1 {
            if column % 2 == 0 {
                self.nextmoves.push(Command::Right);
            }
            self.nextmoves.push(Command::Right);
            self.nextmoves.push(Command::Up);

            self.nextmoves.push(Command::Wait);
            self.nextmoves.push(Command::Down);
            if column % 2 == 0 {
                self.nextmoves.push(Command::Left);
            }
            self.nextmoves.push(Command::Left);

            self.nextmoves.push(Command::PlaceBomb);
            self.nextmoves.push(Command::Right);
        } else if self.target == 2 {
            if row % 2 == 0 {
                self.nextmoves.push(Command::Down);
            }
            self.nextmoves.push(Command::Down);
            self.nextmoves.push(Command::Right);
            self.nextmoves.push(Command::Wait);
            self.nextmoves.push(Command::Wait);
            self.nextmoves.push(Command::Wait);
            self.nextmoves.push(Command::Left);
            if row % 2 == 0 {
                self.nextmoves.push(Command::Up);
            }
            self.nextmoves.push(Command::Up);
            self.nextmoves.push(Command::PlaceBomb);
            self.nextmoves.push(Command::Down);
        } else if self.target == 3 {
            if column % 2 == 0 {
                self.nextmoves.push(Command::Left);
            }
            self.nextmoves.push(Command::Left);
            self.nextmoves.push(Command::Down);

            self.nextmoves.push(Command::Wait);
            self.nextmoves.push(Command::Up);
            if column % 2 == 0 {
                self.nextmoves.push(Command::Right);
            }
            self.nextmoves.push(Command::Right);

            self.nextmoves.push(Command::PlaceBomb);
            self.nextmoves.push(Command::Left);
        } else if self.target == 4 {
            if row % 2 == 0 {
                self.nextmoves.push(Command::Up);
            }
            self.nextmoves.push(Command::Up);
            self.nextmoves.push(Command::Left);
            self.nextmoves.push(Command::Wait);
            self.nextmoves.push(Command::Wait);
            self.nextmoves.push(Command::Wait);
            self.nextmoves.push(Command::Right);
            if row % 2 == 0 {
                self.nextmoves.push(Command::Down);
            }
            self.nextmoves.push(Command::Down);
            self.nextmoves.push(Command::PlaceBomb);
            self.nextmoves.push(Command::Up);
        } else {
            self.nextmoves.push(Command::Wait);
        }

        self.nextmoves.pop().unwrap()
    }

    fn get_debug_info(&self) -> String {
        return self.debug_info.clone();
    }
}
