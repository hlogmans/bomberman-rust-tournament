use std::sync::Arc;
use serde::Serialize;
pub use crate::map::display::*;

use crate::coord::Col;
use crate::coord::Coord;
use crate::coord::Row;
use crate::coord::ValidCoord;
use crate::map::bomb::Bomb;
use crate::map::cell::CellType;
use crate::map::enums::command::Command;
use crate::map::factories::command_factory::CommandFactory;
use crate::map::factories::grid_factory::GridFactory;
use crate::map::player::Player;
use crate::map::structs::map_config::MapConfig;
use crate::map::validators::map_validator::map_validator_chain_factory::MapValidatorChainFactory;

#[derive(Serialize)]
struct MapGridSnapshot {
    grid: Vec<Vec<char>>, // 2D array for easier visualization
}

// a map is a 2D vector of characters. But also contains a list of players and a turn number.
pub struct Map {
    pub map_settings: MapConfig,
    pub grid: Vec<char>,
    pub height: usize,
    pub width: usize,
    pub players: Vec<Player>,
    // the turn number, starts at 0 and increments every turn. One turn is everybody making a move.
    pub bombs: Vec<Bomb>, // List of bombs on the map
    command_factory: Arc<dyn CommandFactory>,
}


impl Map {
    pub fn new(config: MapConfig, factory: Arc<dyn CommandFactory>) -> Self {
        Self {
            map_settings: config,
            grid: vec![],
            width: 0,
            height: 0,
            players: Vec::new(),
            bombs: Vec::new(),
            command_factory: factory,
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        let grid_2d: Vec<Vec<char>> = (0..self.height)
            .map(|row| self.grid[row * self.width..(row + 1) * self.width].to_vec())
            .collect();

        serde_json::json!(MapGridSnapshot {
            grid: grid_2d,
        })
    }

    pub fn build(mut self) -> Self {
        let width = self.map_settings.width;
        let height = self.map_settings.height;

        MapValidatorChainFactory::validate(&self.map_settings).expect("Map validation failed");

        let factory_grid = GridFactory::new(width, height);
        self.grid = factory_grid.prepare_grid();
        self.width = width;
        self.height = height;

        self.players = self.map_settings.player_names.iter().cloned()
            .zip([
                Coord::new(Col::new(1), Row::new(1)),
                Coord::new(Col::new(1), Row::new(width - 2)),
                Coord::new(Col::new(height - 2), Row::new(1)),
                Coord::new(Col::new(height - 2), Row::new(width - 2)),
            ])
            .map(|(name, position)| Player { name, position })
            .collect();

        self.remove_destructables_around_users(
            self.players.iter().map(|p| p.position).collect(),
        );

        self
    }

    // get the BlockType at a given position
    pub fn cell_type(&self, position: Coord) -> CellType {
        if !position.is_valid(self.width, self.height) {
            return CellType::Wall; // Out of bounds is treated as a wall
        }

        let idx = self.cell_index(&position);
        CellType::from_char(self.grid[idx])
    }

    pub(crate) fn clear_destructable(&mut self, location: Coord) {
        if self.cell_type(location) == CellType::Destroyable {
            self.set_cell(location, CellType::Empty);
        }
    }

    pub(crate) fn get_player(&self, no: usize) -> Option<&Player> {
        self.players.get(no)
    }

    /// Get the index of the player at a specific location. The index is the nth record in the players vector.
    pub(crate) fn get_player_index_at_location(&self, location: Coord) -> Option<usize> {
        // Find the player at the given location
        for (index, player) in self.players.iter().enumerate() {
            if player.position == location {
                return Some(index); // Return the index of the player
            }
        }
        None // No player found at this location
    }

    pub(crate) fn get_player_name(&self, no: usize) -> Option<String> {
        self.players.get(no).map(|p| p.name.clone())
    }

    pub(crate) fn get_exploding_bombs(&self) -> Vec<Coord> {
        self.bombs
            .iter()
            .filter(|bomb| bomb.timer == 0)
            .map(|bomb| bomb.position)
            .collect()
    }


    /// remove a bomb from a certain location.
    pub(crate) fn remove_bomb(&mut self, position: Coord) {
        // Remove a bomb at the given position
        self.set_cell(position, CellType::Empty);
        self.bombs.retain(|bomb| bomb.position != position);
    }

    pub(crate) fn bomb_timer_decrease(&mut self) {
        // Decrease the timer of all bombs by 1 if they are not 0
        for bomb in &mut self.bombs {
            if bomb.timer > 0 {
                bomb.timer -= 1; // Decrease the timer
            }
        }
    }

    pub(crate) fn perform_move(&mut self, player: usize, command: Command) -> bool {
        // Validate the command before performing the move
        if !Map::validate_move(self, player, &command) {
            return false; // Invalid move, do not perform it
        }

        // Use the injected factory
        if let Some(cmd) = self.command_factory.create(&command) {
            cmd.execute(self, player);
            true
        } else {
            false
        }
    }

    pub(crate) fn set_wall(&mut self, position: Coord) {
        // Set a wall at the given position
        if position.is_valid(self.width, self.height) {
            self.set_cell(position, CellType::Wall);
        }
    }

    pub(super) fn set_cell(&mut self, position: Coord, cell_type: CellType) {
        if position.is_valid(self.width, self.height) {
            let idx = self.cell_index(&position);
            self.grid[idx] = cell_type.as_char();
        }
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

    fn cell_index(&self, position: &Coord) -> usize {
        position.row.get() * self.width + position.col.get()
    }

    fn get_player_mut(&mut self, no: usize) -> Option<&mut Player> {
        self.players.get_mut(no)
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

    fn remove_destructables_around_users(&mut self, player_positions: Vec<Coord>) {
        for coord in player_positions {
            coord
                .square_3x3()
                .iter()
                .for_each(|c| self.clear_destructable(*c))
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
        Command::Up => current_position.move_up(),
        Command::Down => current_position.move_down(),
        Command::Left => current_position.move_left(),
        Command::Right => current_position.move_right(),
        Command::Wait | Command::PlaceBomb => Some(current_position),
    }
}

