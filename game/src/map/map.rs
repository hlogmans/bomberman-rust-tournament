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


///////////////////////////////////////////////////////////////////////////
/// Handle players
///////////////////////////////////////////////////////////////////////////

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::player::Player;
    use crate::map::grid::cell::CellType;

    #[test]
    fn test_bomb_kills_player() {
        //Arrange
        let map_settings = MapConfig { size: 7, bomb_timer: 0, ..Default::default() };
        let players = vec![
            Player::new("P1".to_string(), Coord::from(1,1), 1),
        ];

        let map = &mut Map::new(map_settings, players);
        map.add_bomb(Coord::from(1, 2), 0);

        //Act
        map.process_bombs();

        //Assert
        let p1 = map.get_player(1).expect("player 1 should exist");
        assert_eq!(p1.is_alive(), false);
        assert_eq!(p1.reason_killed, "bomb");
    }
    #[test]
    fn test_kills_player_when_other_player_was_killed_at_same_location() {
        //Arrange
        let map_settings = MapConfig { size: 7, ..Default::default() };
        let players = vec![
            Player::new("P1".to_string(), Coord::from(1,1), 1),
            Player::new("P2".to_string(), Coord::from(1,1), 2),
            Player::new("P3".to_string(), Coord::from(1,2), 3),
        ];
        let map = &mut Map::new(map_settings, players);
        map.players.get_mut(0)
            .expect("player 1 should exist")
            .kill(&"bomb".to_string(), 3);

        //Act
        map.kill_at_location(Coord::from(1, 1), "bomb".to_string(), 3);

        //Assert
        let p1 = map.get_player(1).expect("player 1 should exist");
        assert_eq!(p1.is_alive(), false);
        let p2 = map.get_player(2).expect("player 2 should exist");
        assert_eq!(p2.is_alive(), false);
        let p3 = map.get_player(3).expect("player 3 should exist");
        assert_eq!(p3.is_alive(), true);
    }
    #[test]
    fn test_own_bomb_kills_player_suicide() {
        //Arrange
        let map_settings = MapConfig { size: 7, bomb_timer: 0, ..Default::default() };
        let players = vec![
            Player::new("P1".to_string(), Coord::from(1,1), 1),
        ];

        let map = &mut Map::new(map_settings, players);
        map.add_bomb(Coord::from(1, 2), 1);

        //Act
        map.process_bombs();

        //Assert
        let p1 = map.get_player(1).expect("player 1 should exist");
        assert_eq!(p1.is_alive(), false);
        assert_eq!(p1.reason_killed, "suicide");
    }

    #[test]
    fn test_handle_shrink_sets_wall_and_kills() {
        //Arrange
        let map_settings = MapConfig { size: 7, endgame: 0, ..Default::default() };
        let players = vec![Player::new("P1".to_string(), Coord::from(1,1), 0)];
        let map = &mut Map::new(map_settings, players);

        //Act
        map.handle_shrink(map.map_settings.endgame);

        //Assert
        assert_eq!(map.grid.cell_type(Coord::from(1, 1)), CellType::Wall);
        let p = map.get_player(0).expect("player exists");
        assert_eq!(p.is_alive(), false);
        assert_eq!(p.reason_killed, "shrink");
    }

    #[test]
    fn test_bomb_chaining_in_range_explodes() {
        //Arrrange
        let map_settings = MapConfig { size: 7, ..Default::default() };
        let players = vec![Player::new("P1".to_string(), Coord::from(1, 1), 0)];
        let map = &mut Map::new(map_settings, players);
        for col in 1..5{
            map.grid.clear_destructable(Coord::from(col, 5));
        }

        map.bombs.push(Bomb::new(Coord::from(1, 5), 0,0));
        map.add_bomb(Coord::from(3, 5), 0);
        map.add_bomb(Coord::from(5, 5), 0);

        //Act
        map.process_bombs();

        //Assert
        assert_eq!(map.bombs.len(), 0)
    }

    #[test]
    fn test_bomb_chaining_ioutside_range_does_not_explodes() {
        //Arrrange
        let map_settings = MapConfig { size: 7, ..Default::default() };
        let players = vec![Player::new("P1".to_string(), Coord::from(1, 1), 0)];
        let map = &mut Map::new(map_settings, players);
        for col in 1..5{
            map.grid.clear_destructable(Coord::from(col, 5));
        }
        map.bombs.push(Bomb::new(Coord::from(1, 5), 0,0));
        map.add_bomb(Coord::from(5, 5), 0);

        //Act
        map.process_bombs();

        //Assert
        assert_eq!(map.bombs.len(), 1)
    }


    #[test]
    fn test_place_bomb_no_duplicate_placement() {
        //Arrange
        let map_settings = MapConfig { size: 7, ..Default::default() };
        let players = vec![Player::new("P1".to_string(), Coord::from(1, 1), 0)];
        let map = &mut Map::new(map_settings, players);
        map.add_bomb(Coord::from(3, 3), 0);

        //Act
        map.add_bomb(Coord::from(3, 3), 0);

        //Assert
        assert_eq!(map.bombs.len(), 1);
    }

    #[test]
    fn test_bomb_timer_decreases_all_bomb() {
        //Arrange
        let map_settings = MapConfig { size: 7, ..Default::default() };
        let players = vec![Player::new("P1".to_string(), Coord::from(1, 1), 0)];
        let map = &mut Map::new(map_settings, players);

        map.add_bomb(Coord::from(1, 2), 0);
        map.add_bomb(Coord::from(2, 1), 0);

        //Act
        map.bomb_timer_decrease();

        //Assert
        assert!(map.bombs.iter().all(|b| b.timer == 2));
    }


    #[test]
    fn test_remove_bomb_at_location() {
        //Arrange
        let map_settings = MapConfig { size: 7, ..Default::default() };
        let players = vec![Player::new("P1".to_string(), Coord::from(1, 1), 0)];
        let map = &mut Map::new(map_settings, players);

        map.add_bomb(Coord::from(1, 2), 0);

        //Act
        map.remove_bombs_at_location(Coord::from(1, 2));
        assert_eq!(map.bombs.len(), 0);
    }

    #[test]
    fn test_two_players_different_tiles_killed_by_one_explosion() {
        //Arrange
        let map_settings = MapConfig { size: 7, bomb_timer: 0, bomb_radius: 2, ..Default::default() };
        let players = vec![
            Player::new("P1".to_string(), Coord::from(1, 2), 0),
            Player::new("P2".to_string(), Coord::from(2, 1), 1),
        ];
        let map = &mut Map::new(map_settings, players);
        map.add_bomb(Coord::from(1, 1), 0);

        //Act
        map.process_bombs();

        //Assert
        assert_eq!(map.get_player(0).unwrap().is_alive(), false);
        assert_eq!(map.get_player(1).unwrap().is_alive(), false);
    }

    #[test]
    fn test_simultaneous_bombs_kill_multiple_players() {
        // Two bombs placed so that their explosions kill players in separate areas simultaneously
        let map_settings = MapConfig { size: 7, bomb_timer: 0, bomb_radius: 1, ..Default::default() };
        let players = vec![
            Player::new("P1".to_string(), Coord::from(2, 2), 0),
            Player::new("P2".to_string(), Coord::from(4, 4), 1),
        ];
        let map = &mut Map::new(map_settings, players);

        map.add_bomb(Coord::from(2, 2), 0);
        map.add_bomb(Coord::from(4, 4), 1);

        map.process_bombs();

        assert_eq!(map.get_player(0).unwrap().is_alive(), false);
        assert_eq!(map.get_player(1).unwrap().is_alive(), false);
    }
}