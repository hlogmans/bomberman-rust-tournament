pub mod game_progress;
pub mod gameresult;
pub mod map_settings;

use rand::seq::SliceRandom;

use crate::{
    bot::Bot,
    coord::Coord,
    game::{game_progress::GameProgress, gameresult::GameResult, map_settings::MapSettings},
    map::{Command, ConsoleDisplay, Map, MapDisplay},
    shrink::calculate_shrink_location,
};

pub struct Game {
    map_settings: MapSettings,

    map: Map,

    bots: Vec<Box<dyn Bot>>,

    display: Box<dyn MapDisplay>,

    #[allow(dead_code)]
    player_count: usize,
    // map turn. Every player sets a cmmmand for the current turn.
    turn: usize,

    // at which turn
    shrink_at_turn: usize,

    // history of handles player actions, it is deterministic, so it can be replayed.
    player_actions: Vec<(usize, Command)>,

    // if the winner is determined, it will be set to Some(index of the winner)
    pub winner: Option<usize>,

    // this is the list of active players that can do a move. The game is won if only one is alive.
    // at start the list is randomized.
    alive_players: Vec<usize>,

    bomb_range: usize,

    width: usize,
    height: usize,
}

impl Game {
    pub fn build(width: usize, height: usize, players: Vec<Box<dyn Bot>>) -> Game {
        // Create a new game instance with the given width, height, and players.
        let mut game = Game::new(width, height, players);
        game.init();
        game
    }

    fn new(width: usize, height: usize, players: Vec<Box<dyn Bot>>) -> Self {
        let player_count = players.len();
        let map = Map::create(
            width,
            height,
            players.iter().map(|bot| bot.name().to_string()).collect(),
        );
        let map_settings = MapSettings {
            bombtimer: 100,
            bombradius: 3,
            endgame: 500,
            width,
            height,
            playernames: Vec::new(),
        };

        let endgame = map_settings.endgame.clone();

        Game {
            map_settings,
            map,
            bots: players,
            player_count: player_count,
            turn: 0,
            shrink_at_turn: endgame,
            player_actions: Vec::new(),
            winner: None,
            // initialize alive player and shuffle them
            alive_players: Vec::new(),
            bomb_range: 3, // Default bomb range, can be adjusted as needed
            width: width,
            height: height,
            display: Box::new(ConsoleDisplay),
        }
    }

    fn init(&mut self) {
        let mut rng = rand::rng();
        self.alive_players = (0..self.bots.len()).collect();
        self.alive_players.shuffle(&mut rng);

        // output the player order for this game
        let player_names_list: Vec<_> = self
            .alive_players
            .iter()
            .map(|&i| self.bots[i].name())
            .collect();
        let pnl = player_names_list.join(", ");
        println!("{}", pnl);

        // call start_game for each bot
        for (i, bot) in self.bots.iter_mut().enumerate() {
            bot.start_game(&self.map_settings, i);
        }
    }

    pub fn run(&mut self) -> GameResult {
        while self.winner.is_none() {
            self.run_round(None, None);
        }
        GameResult::build(self)
    } // loop until a winner is set

    pub fn replay(&mut self, commands: &Vec<Command>) -> GameResult {
        while self.winner.is_none() {
            self.run_round(None, Some(commands));
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
        if self.process_bombs() {
            return true;
        }

        // reduce map size if needed
        if self.shrink_at_turn < self.turn {
            if let Some(shrink_location) =
                calculate_shrink_location(self.turn - self.shrink_at_turn, self.width, self.height)
            {
                // set map location to wall
                self.map.set_wall(shrink_location);

                // check if there is a player at the shrink location
                if let Some(player_index) = self.map.get_player_index_at_location(shrink_location) {
                    // Remove the player from the game
                    let playername = self.map.get_player_name(player_index);
                    if let Some(player_name) = playername {
                        println!(
                            "Player {} has been removed from the game due to shrinking at location {:?}",
                            player_name, shrink_location
                        );
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
                endgame_started: self.turn >= self.shrink_at_turn,
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
        // This function returns the locations that will be affected by a bomb explosion at the given location.
        // It returns the center and all 4 directions (up, down, left, right) within the bomb range.

        // to make it deterministic, the explosion is done in a specific order. From center to outer, then starting on the
        // left side and going to the right side clockwise

        // XXX XXX
        // XXX7XXX
        // XXX3XXX
        //  62148
        // XXX5XXX
        // XXX9XXX
        // XXX XXX
        let mut locations = vec![location];
        let loc = Some(location);
        let mut left = loc;
        let mut right = loc;
        let mut up = loc;
        let mut down = loc;

        for _ in 1..=self.bomb_range {
            left = left.and_then(|l| l.move_left());
            right = right.and_then(|l| l.move_right());
            up = up.and_then(|l| l.move_up());
            down = down.and_then(|l| l.move_down());
            locations.extend(left);
            locations.extend(up);
            locations.extend(right);
            locations.extend(down);
        }

        locations
    }

    /// process the bombs. If there is a winner, return true. Then not all bombs might have been processed.
    /// It stops immediately if a winner is found.
    fn process_bombs(&mut self) -> bool {
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
                    println!(
                        "Player {} has been hit by a bomb at location {:?}",
                        player_index, location
                    );
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

    fn setup_game(width: usize, height: usize, bomb_range: usize) -> Game {
        Game {
            map_settings: MapSettings::default(),
            map: Map::create(width, height, vec!["A".to_string(), "B".to_string()]),
            bots: vec![],
            player_count: 2,
            turn: 0,
            shrink_at_turn: 500,
            player_actions: vec![],
            winner: None,
            alive_players: vec![],
            bomb_range,
            width,
            height,
            display: Box::new(ConsoleDisplay),
        }
    }

    #[test]
    fn test_bomb_explosion_center() {
        let game = setup_game(7, 7, 2);
        let loc = Coord::from(3, 3);
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
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bomb_explosion_corner() {
        let game = setup_game(5, 5, 2);
        let loc = Coord::from(0, 0);
        let result = game.bomb_explosion_locations(loc);
        let expected = vec![
            Coord::from(0, 0),
            Coord::from(1, 0),
            Coord::from(0, 1),
            Coord::from(2, 0),
            Coord::from(0, 2),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bomb_explosion_edge() {
        let game = setup_game(5, 5, 1);
        let loc = Coord::from(0, 2);
        let result = game.bomb_explosion_locations(loc);
        let expected = vec![
            Coord::from(0, 2),
            Coord::from(0, 1),
            Coord::from(1, 2),
            Coord::from(0, 3),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bomb_explosion_range_1() {
        let game = setup_game(5, 5, 1);
        let loc = Coord::from(2, 2);
        let result = game.bomb_explosion_locations(loc);
        let expected = vec![
            Coord::from(2, 2),
            Coord::from(1, 2),
            Coord::from(2, 1),
            Coord::from(3, 2),
            Coord::from(2, 3),
        ];
        assert_eq!(result, expected);
    }
}
