use std::sync::Arc;
use rand::seq::SliceRandom;
use crate::game::bomb_processor::BombProcessor;

use crate::{
    game::{game_progress::GameProgress, game_result::GameResult},
    map::map::{ConsoleDisplay, Map, MapDisplay},
    map::enums::command::Command,
    shrink::calculate_shrink_location,
};
use crate::bot::bot::Bot;
use crate::map::structs::map_config::MapConfig;

pub struct Game {
    pub map: Map,
    bots: Vec<Box<dyn Bot>>,
    display: Box<dyn MapDisplay>,

    #[allow(dead_code)]
    pub player_count: usize,
    // map turn. Every player sets a cmmmand for the current turn.
    pub turn: usize,

    // history of handles player actions, it is deterministic, so it can be replayed.
    pub player_actions: Vec<Vec<Command>>,

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

        let map_settings = MapConfig {
            bomb_timer: 4,
            bomb_radius: 3,
            endgame: 500,
            width,
            height,
            player_names: players.iter().map(|bot| bot.name().to_string()).collect(),
        };

        let  map = Map::new(map_settings, Arc::new(crate::map::factories::command_factory::DefaultCommandFactory)).build();

        let mut player_actions = Vec::with_capacity(player_count);
        for _ in 0..player_count {
            player_actions.push(Vec::new());
        }

        Game {
            map,
            bots: players,
            player_count,
            turn: 0,
            player_actions: player_actions,
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

    pub fn replay(&mut self, commands: &Vec<Vec<Command>>) -> GameResult {
        while self.winner.is_none() {
            self.run_round(None, Some(commands), None);
        }
        GameResult::build(self)
    }

    pub fn winner_name(&self) -> Option<String> {
        self.winner.and_then(|x| self.map.get_player_name(x))
    }

    /// run a single turn for the game. Has a callback for player actions.
    /// returns true if the game has a winner, false otherwise.
    /// There is a callback to check game status like turn number.
    pub fn run_round(
        &mut self,
        progress_callback: Option<&mut dyn FnMut(&GameProgress)>,
        replay_commands: Option<&Vec<Vec<Command>>>,
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
            let loc = self.map.get_player(player_index).unwrap().position;

            let command = if let Some(replay) = replay_commands {
                replay[player_index][self.turn]
            } else {
                let new_command = bot.get_move(&self.map, loc);
                self.player_actions[player_index].push(new_command);
                new_command
            };

            self.map.perform_move(player_index, command);

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
            if let Some(shrink_location) = calculate_shrink_location(
                self.turn - self.map.map_settings.endgame,
                self.map.map_settings.width,
                self.map.map_settings.height,
            ) {
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
                            "Player {player_name} has been removed from the game due to shrinking at location {shrink_location:?}"
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

        false
    }

    /// Check if there is a winner after each round
    /// Returns true if there is a winner, false otherwise.
    fn check_winner(&mut self) -> bool {
        // Check if there is only one player left alive
        let alive_count = self.alive_players.len();
        if alive_count == 1 {
            // Set the winner to the index of the last remaining player
            self.winner = self.alive_players.first().copied(); // Assuming the first player in alive_players is the winner
            return true;
        } else if alive_count == 0 {
            // If no players are left, set winner to None or handle as needed
            self.winner = None;
            return true;
        }
        false
    }

    /// process the bombs. If there is a winner, return true. Then not all bombs might have been processed.
    /// It stops immediately if a winner is found.
    fn process_bombs(&mut self, _logging_callback: &Option<&mut dyn FnMut(String)>) -> bool {
        BombProcessor::process(&mut self.map, &mut self.alive_players)
    }

    pub fn display(&self) {
        self.display.display(&self.map);
    }
}
