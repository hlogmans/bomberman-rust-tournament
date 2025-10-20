use rand::Rng;

use game::bot::bot::Bot;
use game::coord::Coord;
use game::map::enums::command::Command;
use game::map::map::Map;
use game::map::structs::map_config::MapConfig;

pub struct CuddleBot {
    name: String,
    id: usize,
    map_settings: MapConfig,
}

impl Default for CuddleBot {
    fn default() -> Self {
        Self::new()
    }
}

impl CuddleBot {
    pub fn new() -> Self {
        CuddleBot {
            name: "CuddleBot".to_string(),
            id: 0,
            map_settings: MapConfig::default(),
        }
    }

    fn safe_moves(&self, map: &Map, me: Coord) -> Vec<Command> {
        let mut opts = Vec::new();

        for &(command, neighbor_field) in &[
            (Command::Up, me.move_up()),
            (Command::Down, me.move_down()),
            (Command::Left, me.move_left()),
            (Command::Right, me.move_right()),
            (Command::Wait, Some(me)),
        ] {
            let idx =
                neighbor_field.unwrap().row.get() * map.width + neighbor_field.unwrap().col.get();
            if map.grid[idx] == ' ' && !self.is_danger(map, neighbor_field.unwrap()) {
                opts.push(command);
            }
        }

        opts
    }

    fn is_a_bot_near(&self, map: &Map, me: Coord) -> bool {
        for &neighbor_field in &[
            (me.move_up()),
            (me.move_down()),
            (me.move_left()),
            (me.move_right()),
            (Some(me)),
        ] {
            let idx =
                neighbor_field.unwrap().row.get() * map.width + neighbor_field.unwrap().col.get();
            if map.grid[idx] == 'p' {
                return true;
            }
        }

        false
    }

    fn is_danger(&self, map: &Map, locaction: Coord) -> bool {
        map.bombs.iter().any(|bomb| {
            let same_row = bomb.position.row.get() == locaction.row.get();
            let same_col = bomb.position.col.get() == locaction.col.get();
            let row_dist =
                (bomb.position.row.get() as i32 - locaction.row.get() as i32).unsigned_abs() as usize;
            let col_dist =
                (bomb.position.col.get() as i32 - locaction.col.get() as i32).unsigned_abs() as usize;

            (same_row && col_dist <= self.map_settings.bomb_radius + 5)
                || (same_col && row_dist <= self.map_settings.bomb_radius + 5)
        })
    }

    fn go_to_player(&self, map: &Map, me: Coord) -> Vec<Command> {
        let mut opts = Vec::new();

        for &(command, neighbor_field) in &[
            (Command::Up, me.move_up()),
            (Command::Down, me.move_down()),
            (Command::Left, me.move_left()),
            (Command::Right, me.move_right()),
            (Command::Wait, Some(me)),
        ] {
            let idx =
                neighbor_field.unwrap().row.get() * map.width + neighbor_field.unwrap().col.get();
            if map.grid[idx] == ' ' && self.is_player_this_direction(map, neighbor_field.unwrap()) {
                opts.push(command);
            }
        }

        opts
    }

    fn is_player_this_direction(&self, map: &Map, locaction: Coord) -> bool {
        map.players
            .iter()
            .filter(|player| player.name != "CuddleBot-G (0)")
            .any(|player| {
                let same_row = player.position.row.get() == locaction.row.get();
                let same_col = player.position.col.get() == locaction.col.get();
                let row_dist =
                    (player.position.row.get() as i32 - locaction.row.get() as i32).unsigned_abs() as usize;
                let col_dist =
                    (player.position.col.get() as i32 - locaction.col.get() as i32).unsigned_abs() as usize;
                (same_row && col_dist >= self.map_settings.bomb_radius)
                    || (same_col && row_dist >= self.map_settings.bomb_radius)
            })
    }

    fn is_other_player_gerhard(&self, map: &Map) -> bool {
        map.players
            .iter()
            .filter(|player| player.name == "GBot-G (0)")
            .any(|_player| true)
    }
}

impl Bot for CuddleBot {
    fn name(&self) -> String {
        format!("{} ({})", self.name, self.id)
    }

    fn start_game(&mut self, settings: &MapConfig, bot_id: usize) -> bool {
        self.id = bot_id;
        self.map_settings = settings.clone();
        false
    }

    fn get_move(&mut self, map: &Map, me: Coord) -> Command {
        if self.is_other_player_gerhard(map) {
            return Command::PlaceBomb;
        }

        // Flee
        let safe = self.safe_moves(map, me);

        if !safe.is_empty() {
            let mut random = rand::rng();
            return *safe.get(random.random_range(0..safe.len())).unwrap();
        }

        // Hunt
        let hunt = self.go_to_player(map, me);
        if !hunt.is_empty() {
            let mut random = rand::rng();
            return *hunt.get(random.random_range(0..hunt.len())).unwrap();
        }

        // Protect
        if self.is_a_bot_near(map, me) {
            return Command::PlaceBomb;
        }

        // Drink beer
        Command::Wait
    }
}
