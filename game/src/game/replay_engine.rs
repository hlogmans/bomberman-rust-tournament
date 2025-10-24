use crate::coord::Coord;
use crate::game::game::{Game};
use crate::map::bomb::Bomb;
use crate::map::enums::command::Command;
use crate::map::player::Player;
use crate::map::structs::map_config::MapConfig;

#[derive(Clone)]
pub struct GameReplaySnapshot {
    pub map_settings: MapConfig,
    pub turns: Vec<MapReplaySnapshot>,

}
#[derive(Clone)]
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

    pub fn to_snapshot(&mut self, commands: &Vec<Vec<Command>>) -> GameReplaySnapshot {
        let mut turn_snapshots = Vec::new();
        turn_snapshots.push(self.get_snapshot());
        while !self.game.map.has_winner() {
            self.game.run_round( Some(commands));
            turn_snapshots.push(self.get_snapshot());

        }
        GameReplaySnapshot {
            map_settings: self.game.map.map_settings.clone(),
            turns: turn_snapshots,
        }
    }

    fn get_snapshot(&self) -> MapReplaySnapshot {
        MapReplaySnapshot {
                turn: self.game.turn,
                players: self.game.map.players.clone(),
                bombs: self.game.map.bombs.clone(),
                grid: self.game.map.grid.tiles.clone(),
                explosions: self.game.map.explosions.clone(),

            }
    }

}
