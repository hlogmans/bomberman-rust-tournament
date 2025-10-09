use crate::game::game::{Game, GameReplaySnapshot};
use crate::game::game_result::GameResult;
use crate::map::enums::command::Command;

pub struct ReplayEngine<'a> {
    game: &'a mut Game,
}

impl<'a> ReplayEngine<'a> {
    pub fn new(game: &'a mut Game) -> Self {
        Self { game }
    }

    pub fn run(&mut self, commands: &Vec<Vec<Command>>) -> GameResult {
        let mut has_winner: bool = false;
        while !has_winner {
            has_winner = self.game.run_round(None, Some(commands), None);
        }
        GameResult::build(self.game)
    }

    pub fn to_json(&mut self, commands: &Vec<Vec<Command>>) -> serde_json::Value {
        let mut has_winner: bool = false;
        let mut snapshots = Vec::new();

        while !has_winner {
            has_winner = self.game.run_round(None, Some(commands), None);
            snapshots.push(GameReplaySnapshot {
                turn: self.game.turn,
                map: self.game.map.to_json(),
            });
        }

        serde_json::json!(snapshots)
    }
}
