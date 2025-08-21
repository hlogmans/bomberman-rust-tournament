pub mod game_progress;
pub mod gameresult;
pub mod map_settings;

use rand::seq::SliceRandom;
use crate::map::CellType;

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
    // map turn. Every player sets a cmmmand for the current turn.
    turn: usize,

    // history of handles player actions, it is deterministic, so it can be replayed.
    player_actions: Vec<(usize, Command)>,

    // if the winner is determined, it will be set to Some(index of the winner)
    pub winner: Option<usize>,

    // this is the list of active players that can do a move. The game is won if only one is alive.
    // at start the list is randomized.
    alive_players: Vec<usize>,
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
        // Create a new game instance with the given width, height, and players.
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
            map_settings.clone()
        );

        let endgame = map_settings.endgame.clone();

        Game {
            map,
            bots: players,
            player_count: player_count,
            turn: 0,
            player_actions: Vec::new(),
            winner: None,
            // initialize alive player and shuffle them
            alive_players: Vec::new(),
            display: Box::new(ConsoleDisplay),
        }
    }

    fn init(&mut self) {
        let mut rng = rand::rng();
        self.alive_players = (0..self.bots.len()).collect();
        self.alive_players.shuffle(&mut rng);

        // call start_game for each bot
        for (i, bot) in self.bots.iter_mut().enumerate() {
            bot.start_game(&self.map.map_settings, i);
        }
    }

    pub fn run(&mut self) -> GameResult {
        while self.winner.is_none() {
            self.run_round(None, None, None);
            
        }
        GameResult::build(self)
    } // loop until a winner is set

    pub fn replay(&mut self, commands: &Vec<Command>) -> GameResult {
        while self.winner.is_none() {
            self.run_round(None, Some(commands), None);
        }
        GameResult::build(self)
    }

    pub fn winner_name(&self) -> Option<String> {
        match self.winner {
            None => None,
            Some(x) => self.map.get_player_name(x),
        }
    }

    /// run a single turn for the game. Has a callback for player actions.
    /// returns true if the game has a winner, false otherwise.
    /// There is a callback to check game status like turn number.
    pub fn run_round(
        &mut self,
        progress_callback: Option<&mut dyn FnMut(&GameProgress)>,
        replay_commands: Option<&Vec<Command>>,
        logging_callback: Option<&mut dyn FnMut(String)>,
    ) -> bool {
        // This method will run a round of the game.
        // It will handle player actions, update the map, and check for a winner.
        if self.check_winner() {
            return true;
        }

        // Process player actions for the current turn
        for player_index in 0..self.alive_players.len() {
            let bot = self
                .bots
                .get_mut(player_index)
                .expect("Bot not found for player index");
            let loc = self.map.get_player(player_index).unwrap().position.clone();

            // if the game is a replay, take the move from the Vec
            let bot_move;
            if let Some(replay_commands) = replay_commands {
                bot_move = replay_commands[player_index];
            } else {
                bot_move = bot.get_move(&self.map, loc); // Call the provided callback to get the player's command
            }

            self.player_actions.push((player_index, bot_move));

            // handle the command
            self.map.perform_move(player_index, bot_move);

            // Check for winner after processing all actions
            if self.check_winner() {
                return true;
            }
        }

        // process bombs and update the map
        if self.process_bombs(&logging_callback) {
            return true;
        }

        // reduce map size if needed
        if self.map.map_settings.endgame <= self.turn {
            if let Some(shrink_location) =
                calculate_shrink_location(self.turn - self.map.map_settings.endgame, self.map.map_settings.width, self.map.map_settings.height)
            {
                // set map location to wall
                self.map.set_wall(shrink_location);

                // check if there is a player at the shrink location
                if let Some(player_index) = self.map.get_player_index_at_location(shrink_location) {
                    // Remove the player from the game
                    let playername = self.map.get_player_name(player_index);
                    if let Some(player_name) = playername
                        && let Some(cb) = logging_callback
                    {
                        cb(format!(
                            "Player {} has been removed from the game due to shrinking at location {:?}",
                            player_name, shrink_location
                        ));
                    }

                    self.alive_players.retain(|&x| x != player_index);
                }
                if self.check_winner() {
                    return true;
                }
            } else {
                // If no valid shrink location is found, we can handle it as needed.
                // For now, we will just log an error or panic.
                panic!("No valid shrink location found for turn {}", self.turn);
            }

            // set map location to wall
        }

        if self.check_winner() {
            return true;
        }

        // Increment turn
        self.turn += 1;

        if let Some(callback) = progress_callback {
            let progress = GameProgress {
                turn: self.turn,
                endgame_started: self.turn >= self.map.map_settings.endgame,
            };
            callback(&progress);
        }

        return false;
    }

    /// Check if there is a winner after each round
    /// Returns true if there is a winner, false otherwise.
    fn check_winner(&mut self) -> bool {
        // Check if there is only one player left alive
        let alive_count = self.alive_players.len();
        if alive_count == 1 {
            // Set the winner to the index of the last remaining player
            self.winner = self.alive_players.first().map(|x| x.clone()); // Assuming the first player in alive_players is the winner
            return true;
        } else if alive_count == 0 {
            // If no players are left, set winner to None or handle as needed
            self.winner = None;
            return true;
        }
        false
    }

    fn bomb_explosion_locations(&self, location: Coord) -> Vec<Coord> {
        let mut locations = vec![location];

        let directions = [
            |c: Coord| c.move_up(),
            |c: Coord| c.move_down(),
            |c: Coord| c.move_left(),
            |c: Coord| c.move_right(),
        ];
        
        // Iterate over each direction and extend the explosion
        for direction in directions.iter() {
            let mut current_loc = Some(location);
            for _ in 1..=self.map.map_settings.bombradius {
                current_loc = current_loc.and_then(|l| direction(l));
                
                if let Some(loc) = current_loc {
                    let cell_type = self.map.cell_type(loc);
                    

                    match cell_type {
                        // A wall stops the explosion completely in this direction.
                        CellType::Wall => {
                            break;
                        }
                        // A destructible block stops the explosion, but is still destroyed.
                        // So we add its location and then stop.
                        CellType::Destroyable => {
                            locations.push(loc);
                            break;
                        }
                        // Empty space, a player, or a bomb will be affected, and the explosion continues.
                        _ => {
                            locations.push(loc);
                        }
                    }
                } else {
                    break;
                }
            }
        }

        locations
    }

    /// process the bombs. If there is a winner, return true. Then not all bombs might have been processed.
    /// It stops immediately if a winner is found.
    fn process_bombs(&mut self, _logging_callback: &Option<&mut dyn FnMut(String)>) -> bool {
        // step one: check for all bombs that have 1 round left, those will explode this turn
        // changed to a while loop

        // step two: for each bomb that explodes, check the surrounding cells and destroy them if they are destructible
        // for each bomb that explodes, check for each exploding field if there is a player on it, if so, remove the player from the game
        // and check if only one player is left, if so, set the winner to that player.
        // if an exploding field is a bomb, remove it from the map (it won't explode, even if it should explode now).

        // remove 1 from all the bomb timers
        self.map.bomb_timer_decrease();

        // step 2a: check the bomb location to destroy destructible fields
        while let Some(bomb) = self.map.get_next_exploding_bomb_location() {
            self.map.remove_bomb(bomb);
            let explosion_locations = self.bomb_explosion_locations(bomb);
            for location in explosion_locations {
                // Check if the location is destructible
                // first just remove any destructible field at this location
                self.map.clear_destructable(location);

                // clear any bomb at this location
                // if so, it might also need to be removed from the current boms_to_explode list.

                // Check if there is a player at this location, there can only be one
                if let Some(player_index) = self.map.get_player_index_at_location(location) {
                    // if let Some(cb) = logging_callback {
                    //     cb(format!(
                    //         "Player {} has been hit by a bomb at location {:?}",
                    //         player_index, location
                    //     ));
                    // }

                    // Remove the player from the game
                    self.alive_players.retain(|&x| x != player_index);

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

    pub fn display(&self) {
        self.display.display(&self.map);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_game(width: usize, height: usize) -> Game {
        Game {
            map: Map::create(width, height, vec!["A".to_string(), "B".to_string()], MapSettings::default()),
            bots: vec![],
            player_count: 2,
            turn: 0,
            player_actions: vec![],
            winner: None,
            alive_players: vec![],
            display: Box::new(ConsoleDisplay),
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
        let game = setup_game(5, 5);

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
        let game = setup_game(5, 5);
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
        let game = setup_game(5, 5);
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
