use crate::map::commands::move_down::MoveDown;
use crate::map::commands::move_left::MoveLeft;
use crate::map::commands::move_right::MoveRight;
use crate::map::commands::move_up::MoveUp;
use crate::map::commands::place_bomb::PlaceBomb;
use crate::map::commands::traits::player_command::PlayerCommand;
use crate::map::commands::wait::Wait;

pub use crate::map::display::*;

use crate::coord::Col;
use crate::coord::Coord;
use crate::coord::Row;
use crate::coord::ValidCoord;
use crate::map::bomb::Bomb;
use crate::map::cell::CellType;
use crate::map::player::Player;
use crate::map::structs::map_config::MapConfig;
use crate::map::validators::map_validator_chain_factory::MapValidatorChainFactory;

// a map is a 2D vector of characters. But also contains a list of players and a turn number.
pub struct Map {
    pub map_settings: MapConfig,
    pub grid: Vec<char>,
    pub height: usize,
    pub width: usize,
    pub players: Vec<Player>,
    // the turn number, starts at 0 and increments every turn. One turn is everybody making a move.
    pub bombs: Vec<Bomb>, // List of bombs on the map
}


impl Map {
    pub fn create(config: MapConfig) -> Self {
        let width = config.width;
        let height = config.height;

        MapValidatorChainFactory::validate(&config).expect("Map validation failed");

        let player_locations = [
            Coord::new(Col::new(1), Row::new(1)),
            Coord::new(Col::new(1), Row::new(width - 2)),
            Coord::new(Col::new(height - 2), Row::new(1)),
            Coord::new(Col::new(height - 2), Row::new(width - 2)),
        ];

        // Create a new map with the given width and height, filled with destructables

        let grid = prepare_grid(width, height);

        let mut map = Map {
            map_settings: config.clone(),
            grid,
            width,
            height,
            players: config.player_names
                .iter()
                .cloned()
                .zip(player_locations.iter().cloned())
                .map(|(name, position)| Player {
                    name,
                    position, // Initial position will be set later
                })
                .collect(),
            bombs: Vec::new(),
        };

        // remove_destructables_around_users in a 3x3 area
        map.remove_destructables_around_users(
            map.players.iter().map(|player| player.position).collect(),
        );

        map
    }

    pub fn clear_destructable(&mut self, location: Coord) {
        // Clear the destructable cell at the given location
        if self.cell_type(location) == CellType::Destroyable {
            // If the cell is destructable, clear it
            self.set_cell(location, CellType::Empty);
        }
    }

    // get the BlockType at a given position
    pub fn cell_type(&self, position: Coord) -> CellType {
        if !position.is_valid(self.width, self.height) {
            return CellType::Wall; // Out of bounds is treated as a wall
        }

        let idx = self.cell_index(&position);
        match self.grid[idx] {
            ' ' => CellType::Empty,
            'B' => CellType::Bomb,
            'W' => CellType::Wall,
            'P' => CellType::Player,
            '.' => CellType::Destroyable,
            _ => CellType::Empty, // Default case for unknown characters
        }
    }

    pub(super) fn set_cell(&mut self, position: Coord, cell_type: CellType) {
        if position.is_valid(self.width, self.height) {
            let char = match cell_type {
                CellType::Empty => ' ',
                CellType::Bomb => 'B',
                CellType::Player => 'P',
                CellType::Wall => 'W',
                _ => panic!(
                    "Cannot set this cell type directly, use appropriate methods for walls or destroyable cells"
                ),
            };
            let idx = self.cell_index(&position);
            self.grid[idx] = char;
        }
    }

    fn cell_index(&self, position: &Coord) -> usize {
        position.row.get() * self.width + position.col.get()
    }

    pub fn get_player(&self, no: usize) -> Option<&Player> {
        self.players.get(no)
    }

    /// Get the index of the player at a specific location. The index is the nth record in the players vector.
    pub fn get_player_index_at_location(&self, location: Coord) -> Option<usize> {
        // Find the player at the given location
        for (index, player) in self.players.iter().enumerate() {
            if player.position == location {
                return Some(index); // Return the index of the player
            }
        }
        None // No player found at this location
    }

    fn get_player_mut(&mut self, no: usize) -> Option<&mut Player> {
        self.players.get_mut(no)
    }

    pub fn get_player_name(&self, no: usize) -> Option<String> {
        self.players.get(no).map(|p| p.name.clone())
    }

    pub(super) fn get_player_position(&self, no: usize) -> Option<Coord> {
        self.get_player(no).map(|p| p.position)
    }

    pub(super) fn set_player_position(&mut self, no: usize, new_position: Coord) {
        // Find the player and update their position
        if let Some(player) = self.get_player_mut(no) {
            player.position = new_position;
            // Successfully updated the player's position
        };
        // If the player is not found, you might want to handle this case
        // For now, we do nothing
    }

    pub(super) fn add_bomb(&mut self, position: Coord) {
        // if there is no bomb yet at this position, add a bomb
        if self.bombs.iter().any(|bomb| bomb.position == position) {
            return; // A bomb already exists at this position, do not add another
        }
        // Add a bomb at the given position with the map_settings bomb timer
        let timer = self.map_settings.bomb_timer;
        self.bombs.push(Bomb { position, timer });
    }

    /// Get the next bomb to explode from the list, if any. Use this method because processing the
    /// bombs could change the list of bombs.
    pub fn get_next_exploding_bomb_location(&self) -> Option<Coord> {
        // Get the next bomb that is about to explode, if any
        for bomb in &self.bombs {
            if bomb.timer == 0 {
                return Some(bomb.position);
            }
        }
        None
    }

    /// remove a bomb from a certain location.
    pub fn remove_bomb(&mut self, position: Coord) {
        // Remove a bomb at the given position
        self.set_cell(position, CellType::Empty);
        self.bombs.retain(|bomb| bomb.position != position);
    }

    pub fn bomb_timer_decrease(&mut self) {
        // Decrease the timer of all bombs by 1 if they are not 0
        for bomb in &mut self.bombs {
            if bomb.timer > 0 {
                bomb.timer -= 1; // Decrease the timer
            }
        }
    }

    // part one: the server must validate the move on a map.
    // the outer edges are always walls, so they cannot be moved to.
    fn validate_move(map: &Map, player_index: usize, command: &Command) -> bool {
        // Calculate the new position based on the command
        let player_position = match map.get_player_position(player_index) {
            Some(p) => p,
            None => return false, // Player does not exist
        };
        let new_position = new_position(player_position, command).valid(map.width, map.height);

        // Ensure the new position is within the bounds of the map
        if let Some(coord) = new_position {
            // Check if the new position is a wall or occupied by another player
            let cell = map.cell_type(coord);
            if command.is_move() && !can_move_to(cell) {
                return false; // Cannot move to a wall or another player
            }
            return true;
        }
        false // Valid move
    }

    // update the map by performing the move, everything is validated before this is called.
    // false is called if some error occurs, like player not found or invalid move.
    pub fn perform_move(&mut self, player: usize, command: Command) -> bool {
        // Validate the command before performing the move
        if !Map::validate_move(self, player, &command) {
            return false; // Invalid move, do not perform it
        }
        
        let command: Box<dyn PlayerCommand> = match command {
            Command::Up => Box::new(MoveUp),
            Command::Down => Box::new(MoveDown),
            Command::Left => Box::new(MoveLeft),
            Command::Right => Box::new(MoveRight),
            Command::PlaceBomb => Box::new(PlaceBomb),
            Command::Wait => Box::new(Wait),
        };

        command.execute(self, player);
        true
    }

    pub fn set_wall(&mut self, position: Coord) {
        // Set a wall at the given position
        if position.is_valid(self.width, self.height) {
            self.set_cell(position, CellType::Wall);
        }
    }

    fn remove_destructables_around_users(&mut self, player_positions: Vec<Coord>) {
        for coord in player_positions {
            coord
                .square_3x3()
                .iter()
                .for_each(|c| self.clear_destructable(*c))
        }
    }
}

/// Prepare the 2 dimension Vec by adding enough walls
/// - the outer line is walled
/// - the line within the outer wall is destructable
/// - every even row, and every even column contains a wall
///
///  WWWWWWW line 0
///  W.....W
///  W.W.W.W line 2
///  W.....W
///  W.W.W.W
///  W.....W
///  WWWWWWW line 6
pub fn prepare_grid(width: usize, height: usize) -> Vec<char> {
    // the grid is now filled with dots (destructable). I need to add walls.
    // first wall the top layer
    let mut grid = vec!['.'; width * height];

    for row in 0..height {
        for column in 0..width {
            let walled = (row == 0 || row == height - 1 || column == 0 || column == width - 1)
                || (column % 2 == 0 && row % 2 == 0);
            if walled {
                grid[row * width + column] = 'W';
            }
        }
    }
    grid
}

// a command is either up, down, left, right, wait or place_bomb.
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
    fn is_move(&self) -> bool {
        match self {
            Command::Up | Command::Down | Command::Left | Command::Right => true,
            Command::Wait | Command::PlaceBomb => false,
        }
    }
}

/// determine if you can move to a certain type of cell
/// At the moment, only empty cells are considered movable.
fn can_move_to(cell: CellType) -> bool {
    cell == CellType::Empty
}

// calculate the new position based on the command, just the location, not if it is valid or not.
fn new_position(current_position: Coord, command: &Command) -> Option<Coord> {
    match command {
        Command::Up => {
            current_position.move_up() // Move up
        }
        Command::Down => {
            current_position.move_down() // Move down, ensuring it doesn't go out of bounds
        }
        Command::Left => {
            current_position.move_left() // Move left
        }
        Command::Right => {
            current_position.move_right() // Move right, ensuring it doesn't go out of bounds
        }
        // wait or bomb is no-move
        Command::Wait | Command::PlaceBomb => Some(current_position),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_prepare_grid_example() {
        // Example grid:
        // 0: WWWWWWW
        // 1: W.....W
        // 2: W.W.W.W
        // 3: W.....W
        // 4: W.W.W.W
        // 5: W.....W
        // 6: WWWWWWW

        let width = 7;
        let height = 7;
        let grid = super::prepare_grid(width, height);

        let expected = vec![
            'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', '.', '.', '.', '.', '.', 'W', 'W', '.', 'W',
            '.', 'W', '.', 'W', 'W', '.', '.', '.', '.', '.', 'W', 'W', '.', 'W', '.', 'W', '.',
            'W', 'W', '.', '.', '.', '.', '.', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',
        ];

        assert_eq!(grid, expected);
    }
}
