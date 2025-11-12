use crate::bot::bot_data::BotData;
use crate::coord::Coord;
use crate::bot::bot::{BotController};
use crate::map::player::Player;
use crate::map::structs::map_config::MapConfig;
use crate::{game::game_result::GameResult, map::enums::command::Command, map::map::Map};

pub struct Game {
    pub map: Map,
    bots: Vec<BotController>,
    pub turn: usize,
    pub max_turn: usize,
    pub player_actions: Vec<Vec<Command>>,
    pub debug_info: Vec<Vec<String>>,
}

impl Game {
    pub fn build(
        mut bots: Vec<BotController>,
        map_settings: MapConfig,
        bot_data: Option<Vec<BotData>>,
    ) -> Self {
        for (i, bot) in bots.iter_mut().enumerate() {
            bot.start_game(&map_settings, i);
        }
        let map_size = map_settings.size.clone();
        let map = Map::new(
            map_settings,
            Self::generate_players_from_bots(&bots, bot_data, map_size),
        );
        Game::from_map(map, bots)
    }

    pub fn generate_players_from_bots(
        bots: &Vec<BotController>,
        bot_data: Option<Vec<BotData>>,
        size: usize,
    ) -> Vec<Player> {
        let positions = [
            Coord::from(1, 1),
            Coord::from(1, size - 2),
            Coord::from(size - 2, 1),
            Coord::from(size - 2, size - 2),
        ];

        match bot_data {
            Some(bot_data) => bot_data
                .into_iter()
                .zip(positions)
                .map(|(bot, position)| Player::new(bot.name.clone(), position, bot.id))
                .collect(),
            None => bots
                .iter()
                .zip(positions)
                .map(|(bot, position)| Player::new(bot.get_name(), position, bot.get_id()))
                .collect(),
        }
    }

    pub fn from_map(map: Map, bots: Vec<BotController>) -> Self {
        let player_count = bots.len();
        let inner_size = map.map_settings.size - 2;
        let max_turn = map.map_settings.endgame + (inner_size * inner_size);
        Game {
            map,
            bots,
            turn: 0,
            max_turn: max_turn,
            player_actions: vec![Vec::new(); player_count],
            debug_info: vec![Vec::new(); player_count],
        }
    }

    pub fn run(&mut self) -> GameResult {
        self.run_game(None)
    }

    pub fn run_game(&mut self, commands: Option<&Vec<Vec<Command>>>) -> GameResult {
        while !self.map.has_winner() {
            self.run_round(commands);
        }
        GameResult::build(self)
    }

    pub fn winner_name(&self) -> String {
        match &self.map.winner {
            Some(player) => player.name.clone(),
            None => "No winner yet".to_string(),
        }
    }
    pub fn run_round(&mut self, replay_commands: Option<&Vec<Vec<Command>>>) {
        if self.turn >= self.max_turn {
            panic!("Something went terribly wrong ")
        }
        for player_id in self.map.get_alive_players_ids() {
            let command = if let Some(replay) = replay_commands {
                replay[player_id][self.turn]
            } else {
                self.get_command(player_id)
            };
            self.map.try_execute_command(player_id, command);
        }
        self.map.process_bombs();
        if self.map.map_settings.endgame <= self.turn {
            self.map.handle_shrink(self.turn);
        }
        self.turn += 1;
    }


    fn get_command(&mut self, player_id: usize) -> Command{
        let bot = self
            .bots
            .get_mut(player_id)
            .expect("Bot not found for player index");
        let loc = self.map.get_player(player_id).unwrap().position;

        let new_command = bot.get_move(&self.map, loc);
        self.player_actions[player_id].push(new_command);
        self.debug_info[player_id].push(bot.get_debug_info());
        new_command
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::bot::{BotController, Bot};
    use crate::map::structs::map_config::MapConfig;
    use crate::map::enums::command::Command;
    use crate::coord::Coord;

    struct DummyBot {
        name: String,
    }

    impl DummyBot {
        fn new() -> Self {
            Self { name: "dummy".to_string() }
        }
    }

    impl Bot for DummyBot {
        fn start_game(&mut self, _map_settings: &MapConfig, bot_name: String, _bot_id: usize) -> bool {
            self.name = bot_name;
            true
        }

        fn get_move(&mut self, _map: &Map, _player_location: Coord) -> Command {
            Command::Wait
        }
    }

    #[test]
    fn test_run_round_records_actions_and_increments_turn() {
        //Arrange
        let map_settings = MapConfig { size: 7, ..Default::default() };
        let bots = vec![
            BotController::new(Box::new(DummyBot::new()), "bot1".to_string()),
            BotController::new(Box::new(DummyBot::new()), "bot2".to_string()),
        ];
        let mut game = Game::build(bots, map_settings, None);
        
        //Act
        game.run_round(None);

        //Assert
        let alive_ids = game.map.get_alive_players_ids();
        for id in alive_ids {
            assert_eq!(game.player_actions[id].len(), 1);
            assert_eq!(game.debug_info[id].len(), 1);
            assert!(matches!(game.player_actions[id][0], Command::Wait));
        }
        assert_eq!(game.turn, 1);
    }

    #[test]
    #[should_panic]
    fn test_run_round_panics_when_turn_too_large() {
        //Arrange
        let map_settings = MapConfig { size: 7, ..Default::default() };
        let bots = vec![BotController::new(Box::new(DummyBot::new()), "bot1".to_string())];
        let mut game = Game::build(bots, map_settings, None);
        game.turn = game.max_turn;

        //Act
        game.run_round(None);
    }
}