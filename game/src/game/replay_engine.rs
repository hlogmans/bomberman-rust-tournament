use crate::coord::Coord;
use crate::game::game::{Game};
use crate::game::game_result::GameResult;
use crate::map::bomb::Bomb;
use crate::map::enums::command::Command;
use crate::map::player::Player;
use crate::map::structs::map_config::MapConfig;

pub struct GameReplaySnapshot {
    pub map_settings: MapConfig,
    pub turns: Vec<MapReplaySnapshot>,

}

pub struct MapReplaySnapshot {
    pub turn: usize,
    pub players: Vec<Player>,
    pub bombs: Vec<Bomb>,
    pub grid: Vec<char>,
    pub explosions: Vec<Coord>,
}

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

    pub fn to_snapshot(&mut self, commands: &Vec<Vec<Command>>) -> GameReplaySnapshot {
        let mut has_winner = false;
        let mut turn_snapshots = Vec::new();

        // 🟢 Record map state each turn
        while !has_winner {
            // Take snapshot BEFORE running round, so it captures start-of-turn state
            turn_snapshots.push(MapReplaySnapshot {
                turn: self.game.turn,
                players: self.game.map.players.clone(),
                bombs: self.game.map.bombs.clone(),
                grid: self.game.map.grid.clone(),
                explosions: self.game.map.explosions.clone(),

            });

            has_winner = self.game.run_round(None, Some(commands), None);
        }

        // 🟣 Wrap all turn snapshots + map settings in a single game replay
        GameReplaySnapshot {
            map_settings: self.game.map.map_settings.clone(),
            turns: turn_snapshots,
        }
    }
}
