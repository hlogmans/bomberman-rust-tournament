use std::usize;

use crate::coord::Coord;
use crate::map::bomb::{Bomb};
use crate::map::grid::cell::CellType;
use crate::map::enums::command::Command;
use crate::map::factories::command_factory::CommandFactory;
use crate::map::factories::grid_factory::GridFactory;
use crate::map::player::Player;
use crate::map::shrink::calculate_shrink_location;
use crate::map::structs::map_config::MapConfig;
use crate::map::validators::map_validator::map_validator_chain_factory::MapValidatorChainFactory;
use super::grid::grid::Grid;

pub struct Map {
    pub map_settings: MapConfig,
    pub grid: Grid,
    pub players: Vec<Player>,
    pub bombs: Vec<Bomb>,
    pub(crate) explosions: Vec<Coord>,
    pub(crate) winner: Option<Player>,
}


impl Map {
    pub fn new(config: MapConfig, players: Vec<Player>) -> Self {
        MapValidatorChainFactory::validate(&config).expect("Map validation failed");
        let size = config.size.clone();
        Self {
            map_settings: config,
            grid: GridFactory::new(size, players.iter().map(|p| p.position).collect()),
            players: players,
            bombs: Vec::new(),
            explosions: Vec::new(),
            winner: None
        }
    }

    fn check_winner(&mut self) {
        let alive_players = self.get_alive_players();
        let alive_count = alive_players.len();
        if alive_count == 1 {
            self.winner = alive_players.first().cloned().cloned();
        }
    }

    pub fn has_winner(&self) -> bool{
        self.winner.is_some()
    }

    pub fn can_move_to(&self, coord: Coord) -> bool {
        self.grid.cell_type(coord) == CellType::Empty
    }


///////////////////////////////////////////////////////////////////////////
/// Handle players
///////////////////////////////////////////////////////////////////////////

    pub(crate) fn get_player(&self, id: usize) -> Option<&Player> {
        self.players.iter().find(|player| player.id == id)
    }

    pub(crate) fn kill_at_location(&mut self, location: Coord, reason_killed: String, killed_by: usize) {
        if let Some(player) = self.players.iter_mut().find(|player| player.position.col.get() == location.col.get() && player.position.row.get() == location.row.get() && player.is_alive()) {
            player.kill(&reason_killed, killed_by);
            if reason_killed == "bomb" {
                self.grid.set_cell(player.position, CellType::Empty);
            }
            self.check_winner();
        }
    }

    pub fn get_alive_players(&self) -> Vec<&Player>{
        self.players.iter().filter(|player| player.is_alive()).collect()
    }

    pub fn get_alive_players_ids(&self) -> Vec<usize>{
        self
            .get_alive_players()
            .iter()
            .map(|player| player.id)
            .collect()
    }

///////////////////////////////////////////////////////////////////////////
/// Handle player input
///////////////////////////////////////////////////////////////////////////
    pub(crate) fn try_execute_command(&mut self, player: usize, command: Command) {
        if let Some(cmd) = CommandFactory::create(&command) {
            cmd.try_execute(self, player);
        }
    }

///////////////////////////////////////////////////////////////////////////
/// Handle shrink
///////////////////////////////////////////////////////////////////////////

    pub(crate) fn handle_shrink(&mut self, turn: usize){
        let shrink_turn = turn - self.map_settings.endgame;
        if let Some(shrink_location) = calculate_shrink_location(shrink_turn, self.map_settings.size) {
            self.grid.set_wall(shrink_location);
            self.remove_bombs_at_location(shrink_location);
            self.kill_at_location(shrink_location, "shrink".to_string(), usize::MAX);
        } else {
            panic!("No valid shrink location found for shrink {}", shrink_turn);
        };
    }

///////////////////////////////////////////////////////////////////////////
/// Handle bombs
///////////////////////////////////////////////////////////////////////////

    pub(super) fn add_bomb(&mut self, position: Coord, player: usize) {
        if self.bombs.iter().any(|bomb| bomb.position == position) {
            return;
        }
        let timer = self.map_settings.bomb_timer;
        self.bombs.push(Bomb::new(position, timer, player));
    }
    
    pub(crate) fn bomb_timer_decrease(&mut self) {
        for bomb in &mut self.bombs {
            if bomb.timer > 0 {
                bomb.timer -= 1;
            }
        }
    }

    pub(crate) fn remove_bombs_at_location(&mut self, location: Coord) {
        self.bombs.retain(|bomb| bomb.position != location);
    }


    pub(crate) fn get_exploding_bombs(&mut self) -> Vec<Bomb> {
        self.bombs.extract_if(.. , |bomb| bomb.timer == 0)
            .collect()
    }


    fn get_chained_bombs(&mut self, explosion_locations: &Vec<Coord>) -> Vec<Bomb> {
        self.bombs.extract_if(.. , |bomb| explosion_locations.iter().any(|explosion| explosion == &bomb.position))
            .collect()
    }



    pub fn process_bombs(&mut self) {
        self.explosions = Vec::new();
        self.bomb_timer_decrease();
        let exploding_bombs: Vec<Bomb> = self.get_exploding_bombs();
        for bomb in exploding_bombs {
            let player_id = bomb.player_id;
            self.handle_exploding_bomb(bomb, player_id)
        }
    }


    fn handle_exploding_bomb(&mut self, bomb: Bomb, killed_by: usize){
        if self.grid.cell_type(bomb.position) != CellType::Wall{
            self.grid.set_cell(bomb.position, CellType::Empty);
        }
        let mut explosion_locations: Vec<Coord> = bomb.explosion_locations(self);
        for tile in &explosion_locations {
            self.grid.clear_destructable(*tile);
            self.kill_at_location(*tile, "bomb".to_string(), killed_by);
        }
        let chain = self.get_chained_bombs(&explosion_locations);
        for bomb in chain {
            self.handle_exploding_bomb(bomb, killed_by)
        }
        self.explosions.append(&mut explosion_locations);
    }



}