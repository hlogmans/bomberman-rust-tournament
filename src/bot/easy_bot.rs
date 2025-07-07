use crate::{
    bot::Bot,
    coord::Coord,
    game::map_settings::MapSettings,
    map::{Command, Map},
};

pub struct EasyBot {
    pub name: String,
    pub id: usize,

    nextmoves: Vec<Command>,
}

impl Bot for EasyBot {
    fn name(&self) -> String {
        // return the name plus the ID
        format!("{} ({})", self.name, self.id)
    }

    fn get_move(&mut self, map: &Map, player_location: Coord) -> Command {
        // if there are moves left, do them first
        if !self.nextmoves.is_empty() {
            return self.nextmoves.pop().unwrap();
        }

        // // do I need to run away?
        // if let Some(runaway) = self.danger_location(map, player_location) {
        //     return runaway;
        // }
        // is this location safe to put a bomb?
        if let Some(moves) = self.safe_to_bomb(map, player_location) {
            self.nextmoves = moves;
            return self.nextmoves.pop().unwrap();
        }

        // Randomly choose a command for the bot, there is no specific strategy
        use rand::Rng;
        let mut rng = rand::rng();
        let commands = vec![
            Command::Up,
            Command::Down,
            Command::Left,
            Command::Right,
            Command::Wait,
        ];
        commands[rng.random_range(0..commands.len())].clone()
    }

    fn start_game(&mut self, _map_settings: &MapSettings, bot_id: usize) -> bool {
        self.id = bot_id;
        true
    }
}

impl EasyBot {
    pub fn new(name: String) -> Self {
        EasyBot {
            name,
            id: 0,
            nextmoves: Vec::new(),
        }
    }

    fn safe_to_bomb(&self, map: &Map, loc: Coord) -> Option<Vec<Command>> {
        // it is safe to bomb if there is an open space next to this position, and there is an open space lateral to that
        // position. So there are 8 options: up-left, up-right, down-left, down-right, left-up, left-down, right-up, right-down
        let vertical = vec![Command::Up, Command::Down];
        let horizontal = vec![Command::Left, Command::Right];
        let options = vec![
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
            if let Some(loc1) = loc.move_command(m1) {
                if let Some(loc2) = loc1.move_command(m2) {
                    // if both locations are empty, then we can bomb there
                    if self.get_cell(map, loc1) == ' ' && self.get_cell(map, loc2) == ' ' {
                        return Some(vec![m2, m1, Command::PlaceBomb]);
                    }
                }
            }
        }
        None
    }

    /// Check if the current location is dangerous because of bombs
    fn danger_location(&self, map: &Map, player_location: Coord) -> Option<Command> {
        None
    }

    /// get a cell, returns a wall if invalid
    fn get_cell(&self, map: &Map, location: Coord) -> char {
        return map
            .grid
            .get(location.row.get() * map.width + location.col.get())
            .unwrap_or(&'W')
            .clone();
    }
}
