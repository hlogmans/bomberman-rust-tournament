use game::bot::bot::Bot;
use game::coord::Coord;
use game::map::enums::command::Command;
use game::map::map::Map;
use game::map::structs::map_config::MapConfig;

#[derive(Clone)]
pub struct EasyBot {
    pub name: String,
    pub id: usize,

    nextmoves: Vec<Command>,
    map_settings: MapConfig,
}

impl Bot for EasyBot {
    fn name(&self) -> String {
        format!("{} ({})", self.name, self.id)
    }

    fn get_move(&mut self, map: &Map, player_location: Coord) -> Command {
        if !self.nextmoves.is_empty() {
            return self.nextmoves.pop().unwrap();
        }

        // Run away if in danger
        if let Some(runaway) = self.danger_location(map, &player_location) {
            return runaway;
        }

        // Safe to place a bomb
        if let Some(moves) = self.safe_to_bomb(map, &player_location) {
            self.nextmoves = moves;
            return self.nextmoves.pop().unwrap();
        }

        // Random move
        use rand::Rng;
        let mut rng = rand::rng();
        let commands = [
            Command::Up,
            Command::Down,
            Command::Left,
            Command::Right,
            Command::Wait,
        ];
        commands[rng.random_range(0..commands.len())]
    }

    fn start_game(&mut self, map_settings: &MapConfig, bot_id: usize) -> bool {
        self.id = bot_id;
        self.map_settings = map_settings.clone();
        true
    }
}

impl Default for EasyBot {
    fn default() -> Self {
        Self::new()
    }
}

impl EasyBot {
    pub fn new() -> Self {
        EasyBot {
            name: "EasyBot".to_string(),
            id: 0,
            nextmoves: Vec::new(),
            map_settings: MapConfig::default(),
        }
    }

    fn safe_to_bomb(&self, map: &Map, loc: &Coord) -> Option<Vec<Command>> {
        let vertical = [Command::Up, Command::Down];
        let horizontal = [Command::Left, Command::Right];
        let options = [
            (vertical[0], horizontal[0]),
            (vertical[0], horizontal[1]),
            (vertical[1], horizontal[0]),
            (vertical[1], horizontal[1]),
            (horizontal[0], vertical[0]),
            (horizontal[0], vertical[1]),
            (horizontal[1], vertical[0]),
            (horizontal[1], vertical[1]),
        ];

        for (m1, m2) in options {
            if let Some(loc1) = loc.move_command(m1) && let Some(loc2) = loc1.move_command(m2) {
                if self.get_cell(map, &loc1) == ' ' && self.get_cell(map, &loc2) == ' ' {
                    return Some(vec![
                        Command::Wait,
                        Command::Wait,
                        m2,
                        m1,
                        Command::PlaceBomb,
                    ]);
                }
            }
        }

        None
    }

    fn danger_location(&self, map: &Map, player_location: &Coord) -> Option<Command> {
        for bomb in map.bombs.iter().map(|b| &b.position) {
            if in_bomb_range(player_location, bomb, self.map_settings.bomb_radius as u32) {
                return Some(Command::Left); // temporary naive strategy
            }
        }
        None
    }

    fn get_cell(&self, map: &Map, location: &Coord) -> char {
        *map
            .grid
            .get(location.row.get() * map.width + location.col.get())
            .unwrap_or(&'W')
    }
}

/// Free function to compute distance
pub fn coord_distance(a: &Coord, b: &Coord) -> Option<u32> {
    if a.col.get() == b.col.get() && a.row.get() == b.row.get() {
        return Some(0);
    }
    if a.col.get() != b.col.get() && a.row.get() != b.row.get() {
        return None;
    }
    Some(
        ((a.col.get() as i32 - b.col.get() as i32).abs()
            + (a.row.get() as i32 - b.row.get() as i32).abs()) as u32,
    )
}

/// Free function to check bomb range
pub fn in_bomb_range(loc: &Coord, bomb: &Coord, radius: u32) -> bool {
    if let Some(d) = coord_distance(loc, bomb) {
        d <= radius
    } else {
        false
    }
}
