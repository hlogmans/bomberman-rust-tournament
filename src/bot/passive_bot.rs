use crate::{
    bot::Bot,
    coord::Coord,
    game::map_settings::MapSettings,
    map::{Map, Command},
};

pub struct PassiveBot {
    name: String,
    id: usize,
    map_settings: MapSettings,
}

impl PassiveBot {
    pub fn new(name: String) -> Self {
        PassiveBot { name, id: 0, map_settings: MapSettings::default() }
    }

    /// Is `loc` in the straight-line blast zone of any bomb (current dont care about timer)
    fn is_danger(&self, map: &Map, loc: Coord) -> bool {
        map.bombs.iter()
            .filter(|b| b.timer <= self.map_settings.bombtimer) //now all can be changed later
            .any(|b| {                
                let same_row = b.position.row.get() == loc.row.get();
                let same_col = b.position.col.get() == loc.col.get();
                let row_dist = (b.position.row.get() as i32 - loc.row.get() as i32).abs() as usize;
                let col_dist = (b.position.col.get() as i32 - loc.col.get() as i32).abs() as usize;

                (same_row && col_dist <= self.map_settings.bombradius + 2) || (same_col && row_dist <= self.map_settings.bombradius + 2) //bombs scary stay even 2 more tiles away than blast
            })
    }

    /// All legal moves (Up/Down/Left/Right/Wait) that land on a space and aren’t dangerous.
    fn safe_moves(&self, map: &Map, me: Coord) -> Vec<(Command, Coord)> {
        let mut opts = Vec::new();
        for &(cmd, neighbor) in &[
            (Command::Up,    me.move_up()),
            (Command::Down,  me.move_down()),
            (Command::Left,  me.move_left()),
            (Command::Right, me.move_right()),
            (Command::Wait,  Some(me)),
        ] {
            if let Some(nc) = neighbor {
                let idx = nc.row.get() * map.width + nc.col.get();
                if map.grid[idx] == ' ' && !self.is_danger(map, nc) {
                    opts.push((cmd, nc));
                }
            }
        }
        opts
    }

    fn get_best_safe_move(&self, map: &Map, safe: &Vec<(Command, Coord)>) -> Command {
        let center_row = map.height / 2;
        let center_col = map.width / 2;

        let best = safe.iter()
            .max_by_key(|(_, coord)| {
                let row_diff = (coord.row.get() as isize - center_row as isize).abs();
                let col_diff = (coord.col.get() as isize - center_col as isize).abs();
                let center_score = -(row_diff + col_diff); // closer to center = higher score

                let escape_routes = self.safe_moves(map, *coord).len();

                return center_score * 2 + escape_routes as isize
            })
            .unwrap();

        best.0
    }
}

impl Bot for PassiveBot {
    fn name(&self) -> String {
        format!("{} ({})", self.name, self.id)
    }

    fn start_game(&mut self, settings: &MapSettings, bot_id: usize) -> bool {
        self.id = bot_id;
        self.map_settings = settings.clone();
        true
    }

    fn get_move(&mut self, map: &Map, me: Coord) -> Command {
       let safe = self.safe_moves(map, me);

        if !safe.is_empty() {
            return self.get_best_safe_move(map, &safe);
        }  
        
        // (3) Else, wait.
        return Command::Wait;
    }
}
