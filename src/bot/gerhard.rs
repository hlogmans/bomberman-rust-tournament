use rand::Rng;

use crate::{
    bot::Bot,
    coord::Coord,
    game::map_settings::MapSettings,
    map::{Map, Command},
};

pub struct GerhardBot {
    name: String,
    id: usize,
    map_settings: MapSettings,
}

impl GerhardBot {
    pub fn new(name: String) -> Self {
        GerhardBot { name, id: 0, map_settings: MapSettings::default() }
    }

    fn safe_moves(&self, map: &Map, me: Coord) -> Vec<Command> {
        let mut opts = Vec::new();

        for &(command, neighbor_field) in &[
            (Command::Up,    me.move_up()),
            (Command::Down,  me.move_down()),
            (Command::Left,  me.move_left()),
            (Command::Right, me.move_right()),
            (Command::Wait,  Some(me)),
        ] {
            let idx = neighbor_field.unwrap().row.get() * map.width + neighbor_field.unwrap().col.get();
            if map.grid[idx] == ' ' && !self.is_danger(map, neighbor_field.unwrap()) {
                opts.push(command);
            }
        }

        return opts
    }

    fn is_a_bot_near(&self, map: &Map, me: Coord) -> bool {
        for &neighbor_field in &[
            (me.move_up()),
            (me.move_down()),
            (me.move_left()),
            (me.move_right()),
            (Some(me)),
        ] {
            let idx = neighbor_field.unwrap().row.get() * map.width + neighbor_field.unwrap().col.get();
            if map.grid[idx] == 'p' {
                return true;
            }
        }

        return false
    }

    fn is_danger(&self, map: &Map, locaction: Coord) -> bool {
        map.bombs.iter()
            .any(|bomb| {                
                let same_row = bomb.position.row.get() == locaction.row.get();
                let same_col = bomb.position.col.get() == locaction.col.get();
                let row_dist = (bomb.position.row.get() as i32 - locaction.row.get() as i32).abs() as usize;
                let col_dist = (bomb.position.col.get() as i32 - locaction.col.get() as i32).abs() as usize;

                (same_row && col_dist <= self.map_settings.bombradius + 5) || (same_col && row_dist <= self.map_settings.bombradius + 5)
            })
    }
}

impl Bot for GerhardBot {
    fn name(&self) -> String {
        format!("{} ({})", self.name, self.id)
    }

    fn start_game(&mut self, settings: &MapSettings, bot_id: usize) -> bool {
        self.id = bot_id;
        self.map_settings = settings.clone();
        false
    }

    fn get_move(&mut self, map: &Map, me: Coord) -> Command {
        // Flee
        let safe = self.safe_moves(map, me);

        if !safe.is_empty() {
            let mut random = rand::rng();
            return *safe.get(random.random_range(0..safe.len())).unwrap();
        }

        // Protect
        if self.is_a_bot_near(map, me) {
            return Command::PlaceBomb
        }

        // Drink beer
        Command::Wait
    }
}