pub mod game_progress;
pub mod gameresult;
pub mod map_settings;

use crate::map::CellType;
use rand::seq::SliceRandom;
use rand::{rng};

use crate::{
    bot::Bot,
    coord::Coord,
    game::{game_progress::GameProgress, gameresult::GameResult, map_settings::MapSettings},
    map::{Command, ConsoleDisplay, Map, MapDisplay},
    shrink::calculate_shrink_location,
};

pub struct Game {
    map: Map,
    bots: Vec<Box<dyn Bot>>,
    display: Box<dyn MapDisplay>,
    #[allow(dead_code)]
    player_count: usize,
    turn: usize,
    // history of handled player actions, it is deterministic, so it can be replayed.
    player_actions: Vec<(usize, Command)>,
    // if the winner is determined, it will be set to Some(index of the winner)
    pub winner: Option<usize>,
    // this is the list of active players that can do a move. The game is won if only one is alive.
    // at start the list is randomized.
    alive_players: Vec<usize>,
    is_alive: Vec<bool>,            // track alive status of each player
    alive_count: usize,             // number of alive players
    explosion_buffer: Vec<Coord>,   // reusable buffer to avoid allocations for explosions
}

impl Game {
    /// Constructs a new game instance with the given width, height, and players.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the game map.
    /// * `height` - The height of the game map.
    /// * `players` - The list of players participating in the game.
    ///
    /// # Returns
    ///
    /// A game result object that contains the winner and the history of player actions.
    pub fn build(width: usize, height: usize, players: Vec<Box<dyn Bot>>) -> Game {
        let mut game = Game::new(width, height, players);
        game.init();
        game
    }

    fn new(width: usize, height: usize, players: Vec<Box<dyn Bot>>) -> Self {
        let player_count = players.len();

        let map_settings = MapSettings {
            bombtimer: 4,
            bombradius: 3,
            endgame: 500,
            width,
            height,
            playernames: Vec::new(),
        };

        let map = Map::create(
            width,
            height,
            players.iter().map(|bot| bot.name().to_string()).collect(),
            map_settings.clone(),
        );

        Game {
            map,
            bots: players,
            player_count,
            turn: 0,
            player_actions: Vec::new(),
            winner: None,
            alive_players: Vec::new(),
            display: Box::new(ConsoleDisplay),
            is_alive: vec![true; player_count],
            alive_count: player_count,
            explosion_buffer: Vec::with_capacity(20),
        }
    }

    /// Initialize the game by shuffling players and notifying each bot
    fn init(&mut self) {
        let mut rng = rng();
        self.alive_players = (0..self.bots.len()).collect();
        self.alive_players.shuffle(&mut rng);

        // call start_game for each bot
        for (i, bot) in self.bots.iter_mut().enumerate() {
            bot.start_game(&self.map.map_settings, i);
        }
    }

    /// Run the game loop until a winner is found
    pub fn run(&mut self) -> GameResult {
        while self.winner.is_none() {
            self.run_round(None, None, None);
        }
        GameResult::build(self)
    }

    /// Replay a sequence of commands instead of live bot moves
    pub fn replay(&mut self, commands: &Vec<Command>) -> GameResult {
        while self.winner.is_none() {
            self.run_round(None, Some(commands), None);
        }
        GameResult::build(self)
    }

    /// Returns the name of the winner if there is one
    pub fn winner_name(&self) -> Option<String> {
        // Flatten Option<Option<String>> to Option<String>
        self.winner.and_then(|x| self.map.get_player_name(x))
    }

    /// Run a single round (turn) of the game
    /// Returns true if a winner is found, false otherwise
    pub fn run_round(
        &mut self,
        progress_callback: Option<&mut dyn FnMut(&GameProgress)>,
        replay_commands: Option<&Vec<Command>>,
        logging_callback: Option<&mut dyn FnMut(String)>,
    ) -> bool {
        // Check for winner before processing
        if self.check_winner() {
            return true;
        }

        // Process player actions for the current turn
        for player_index in 0..self.player_count {
            if !self.is_alive[player_index] {
                continue;
            }

            let bot = &mut self.bots[player_index];
            let loc = self.map.get_player(player_index).unwrap().position;

            // if the game is a replay, take the move from the Vec
            let bot_move = if let Some(replay_commands) = replay_commands {
                replay_commands[player_index]
            } else {
                bot.get_move(&self.map, loc) // Call bot for move
            };

            self.player_actions.push((player_index, bot_move));

            // handle the command
            self.map.perform_move(player_index, bot_move);

            // Check for winner after processing moves
            if self.check_winner() {
                return true;
            }
        }

        // process bombs and update the map
        if self.process_bombs(&logging_callback) {
            return true;
        }

        // reduce map size if needed (shrink)
        if self.map.map_settings.endgame <= self.turn {
            if let Some(shrink_location) = calculate_shrink_location(
                self.turn - self.map.map_settings.endgame,
                self.map.map_settings.width,
                self.map.map_settings.height,
            ) {
                // set map location to wall
                self.map.set_wall(shrink_location);

                // check if there is a player at the shrink location
                if let Some(player_index) = self.map.get_player_index_at_location(shrink_location) {
                    self.is_alive[player_index] = false;
                    self.alive_count -= 1;

                    // Optional logging
                    if let Some(player_name) = self.map.get_player_name(player_index) {
                        if let Some(cb) = logging_callback {
                            cb(format!(
                                "Player {player_name} has been removed from the game due to shrinking at location {shrink_location:?}"
                            ));
                        }
                    }
                }

                if self.check_winner() {
                    return true;
                }
            } else {
                panic!("No valid shrink location found for turn {}", self.turn);
            }
        }

        if self.check_winner() {
            return true;
        }

        // Increment turn counter
        self.turn += 1;

        // Optional progress callback
        if let Some(callback) = progress_callback {
            let progress = GameProgress {
                turn: self.turn,
                endgame_started: self.turn >= self.map.map_settings.endgame,
            };
            callback(&progress);
        }

        false
    }

    /// Check if a winner exists. Updates `self.winner` accordingly.
    fn check_winner(&mut self) -> bool {
        if self.alive_count == 1 {
            self.winner = Some(
                self.is_alive
                    .iter()
                    .enumerate()
                    .find(|&(_, alive)| *alive)
                    .unwrap()
                    .0,
            );
            true
        } else if self.alive_count == 0 {
            self.winner = None;
            true
        } else {
            false
        }
    }

    /// Calculate all locations affected by a bomb explosion
    fn bomb_explosion_locations(&mut self, location: Coord) -> Vec<Coord> {
        self.explosion_buffer.clear();
        self.explosion_buffer.push(location);

        let directions = [
            |c: Coord| c.move_up(),
            |c: Coord| c.move_down(),
            |c: Coord| c.move_left(),
            |c: Coord| c.move_right(),
        ];

        // Iterate over each direction and extend the explosion
        for dir in directions {
            let mut current = Some(location);
            for _ in 1..=self.map.map_settings.bombradius {
                current = current.and_then(dir);

                if let Some(loc) = current {
                    let cell_type = self.map.cell_type(loc);

                    match cell_type {
                        // A wall stops the explosion completely in this direction.
                        CellType::Wall => break,
                        // A destructible block stops the explosion, but is still destroyed.
                        CellType::Destroyable => {
                            self.explosion_buffer.push(loc);
                            break;
                        }
                        // Empty space, a player, or a bomb will be affected, and the explosion continues.
                        _ => {
                            self.explosion_buffer.push(loc);
                        }
                    }
                } else {
                    break;
                }
            }
        }

        // Return a fresh Vec to avoid borrow conflicts
        self.explosion_buffer.clone()
    }

    /// Process bombs and apply damage/explosions. Returns true if a winner is found.
    fn process_bombs(&mut self, _logging_callback: &Option<&mut dyn FnMut(String)>) -> bool {
        // step one: decrease timers of all bombs
        self.map.bomb_timer_decrease();

        // step two: handle explosions
        while let Some(bomb) = self.map.get_next_exploding_bomb_location() {
            self.map.remove_bomb(bomb);
            let explosion_locations = self.bomb_explosion_locations(bomb);

            for location in explosion_locations {
                // Clear destructible fields
                self.map.clear_destructable(location);

                // Check for player hit
                if let Some(player_index) = self.map.get_player_index_at_location(location) {
                    self.is_alive[player_index] = false;
                    self.alive_count -= 1;

                    // Check if there is a winner
                    if self.check_winner() {
                        return true;
                    }
                }
            }
        }
        false

        // step three: decrease the timer of all bombs by 1, and remove those that have a timer of 0
    }

    /// Display the current map state
    pub fn display(&self) {
        self.display.display(&self.map);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_game(width: usize, height: usize) -> Game {
        Game {
            map: Map::create(
                width,
                height,
                vec!["A".to_string(), "B".to_string()],
                MapSettings::default(),
            ),
            bots: vec![],
            player_count: 2,
            turn: 0,
            player_actions: vec![],
            winner: None,
            alive_players: vec![],
            is_alive: vec![],
            alive_count: 0,
            display: Box::new(ConsoleDisplay),
            explosion_buffer: vec![],
        }
    }

    #[test]
    fn test_bomb_explosion_center_clear_path() {
        let mut game = setup_game(7, 7);
        let loc = Coord::from(3, 3);

        game.map.clear_destructable(Coord::from(3, 3));
        game.map.clear_destructable(Coord::from(2, 3)); // up
        game.map.clear_destructable(Coord::from(4, 3)); // down
        game.map.clear_destructable(Coord::from(3, 2)); // left
        game.map.clear_destructable(Coord::from(3, 4)); // right

        let result = game.bomb_explosion_locations(loc);

        let expected = vec![
            Coord::from(3, 3), // center
            Coord::from(2, 3),
            Coord::from(3, 2), // up
            Coord::from(4, 3),
            Coord::from(3, 4), // down
            Coord::from(1, 3),
            Coord::from(3, 1), // left
            Coord::from(5, 3),
            Coord::from(3, 5), // right
        ];

        assert_eq!(result.len(), expected.len());

        for coord in expected {
            assert_eq!(result.contains(&coord), true);
        }
    }

    #[test]
    fn test_bomb_explosion_corner() {
        let mut game = setup_game(5, 5);

        let loc = Coord::from(1, 1);
        let result = game.bomb_explosion_locations(loc);
        let expected = vec![
            Coord::from(1, 1),
            Coord::from(1, 2),
            Coord::from(1, 3),
            Coord::from(2, 1),
            Coord::from(3, 1),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bomb_explosion_corner_with_destructable() {
        let mut game = setup_game(5, 5);
        let loc = Coord::from(3, 3);

        let result = game.bomb_explosion_locations(loc);
        let expected = vec![
            Coord::from(3, 3),
            Coord::from(3, 2),
            //Coord::from(3, 1), Can't be destroyed there is a destrutable in the way at 3,2
            Coord::from(2, 3),
            Coord::from(1, 3),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bomb_explosion_range_1() {
        let mut game = setup_game(5, 5);
        let loc = Coord::from(2, 2);
        let result = game.bomb_explosion_locations(loc);
        let expected = vec![
            Coord::from(2, 2),
            Coord::from(2, 1),
            Coord::from(2, 3),
            Coord::from(1, 2),
            Coord::from(3, 2),
        ];
        assert_eq!(result, expected);
    }
}