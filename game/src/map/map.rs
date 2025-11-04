use crate::coord::Coord;
use crate::map::bomb::{Bomb};
use crate::map::cell::CellType;
use crate::map::enums::command::Command;
use crate::map::factories::command_factory::CommandFactory;
use crate::map::factories::grid_factory::GridFactory;
use crate::map::player::Player;
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

    pub(crate) fn get_player(&self, id: usize) -> Option<&Player> {
        self.players.iter().find(|player| player.id == id)
    }

    pub(crate) fn kill_at_location(&mut self, location: Coord) {
        if let Some(player) = self.players.iter_mut().find(|player| player.position.col.get() == location.col.get() && player.position.row.get() == location.row.get()) {
            player.kill();
            self.check_winner();
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

    pub(crate) fn get_exploding_bombs(&self) -> Vec<Coord> {
        let mut exploding_bombs: Vec<Coord> = self.bombs
            .iter()
            .filter(|bomb| bomb.timer == 0)
            .map(|bomb| bomb.position)
            .collect();

        let chained: Vec<Coord> = self.bombs
            .iter()
            .filter(|bomb| bomb.timer > 0)
            .filter(|bomb| exploding_bombs.iter().any(|exploding| 
                self.location_in_bomb_range(bomb.position, *exploding)
            ))
            .map(|bomb| bomb.position)
            .collect();

        exploding_bombs.extend(chained);

        exploding_bombs
    }

    fn location_in_bomb_range(&self, position: Coord, bomb_position: Coord) -> bool {
        if position == bomb_position {
            return true;
        }
        if position.row == bomb_position.row {
            if position.col.get().abs_diff(bomb_position.col.get()) <= self.map_settings.bomb_radius {
                return true;
            }
        }
        if position.col == bomb_position.col {
            if position.row.get().abs_diff(bomb_position.row.get()) <= self.map_settings.bomb_radius {
                return true;
            }
        }
        false
    }



    pub(crate) fn remove_bomb(&mut self, position: Coord) {
        if self.grid.cell_type(position) != CellType::Wall{
            self.grid.set_cell(position, CellType::Empty);
        }
        self.remove_bombs_at_location(position);
    }

    pub(crate) fn bomb_timer_decrease(&mut self) {
        for bomb in &mut self.bombs {
            if bomb.timer > 0 {
                bomb.timer -= 1;
            }
        }
    }

    pub(crate) fn try_execute_command(&mut self, player: usize, command: Command) {
        if let Some(cmd) = CommandFactory::create(&command) {
            cmd.try_execute(self, player);
        }
    }

    pub(super) fn add_bomb(&mut self, position: Coord) {
        if self.bombs.iter().any(|bomb| bomb.position == position) {
            return;
        }
        let timer = self.map_settings.bomb_timer;
        self.bombs.push(Bomb { position, timer });
    }

    pub(crate) fn handle_shrink(&mut self, turn: usize){
        let shrink_turn = turn - self.map_settings.endgame;
        let shrink_location = self.grid.shrink(shrink_turn);
        self.remove_bombs_at_location(shrink_location);
        self.kill_at_location(shrink_location);
    }


    pub(crate) fn remove_bombs_at_location(&mut self, location: Coord) {
        self.bombs.retain(|bomb| bomb.position != location);
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

    pub fn can_move_to(&self, coord: Coord) -> bool {
        self.grid.cell_type(coord) == CellType::Empty
    }


}