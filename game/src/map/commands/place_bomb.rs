use crate::map::cell::CellType;
use crate::map::commands::traits::player_command::PlayerCommand;
use crate::map::map::Map;

pub struct PlaceBomb;

impl PlayerCommand for PlaceBomb {
    fn try_execute(&self, map: &mut Map, player_index: usize) {
        let pos = map.players[player_index].position;
        map.add_bomb(pos);
        map.grid.set_cell(pos, CellType::Bomb);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::map::Map;
    use crate::map::commands::traits::player_command::PlayerCommand;
    use crate::map::player::Player;
    use crate::map::structs::map_config::MapConfig;
    use crate::coord::Coord;

    #[test]
    fn test_place_bomb_basic() {
        //Arrange
        let mut map = Map::new(MapConfig { size: 7, ..MapConfig::default() },  vec![]);


        let player_pos = Coord::from(3, 3);
        map.players = vec![Player::new("player1".to_string(), player_pos, 0)];

        // Act
        let place_bomb = PlaceBomb;
        place_bomb.try_execute(&mut map, 0);

        // Assert
        assert!(map.bombs[0].position == player_pos);
    }

    // #[test]
    // fn test_place_bomb_no_player() {
    //     // Arrange
    //     let mut map = Map::new(MapConfig { size: 7, ..MapConfig::default() }, Arc::new(DefaultCommandFactory)).build();
    //     // default has 2 players atm
    //     let player_index = 3;

    //     // Assert
    //     let place_bomb = PlaceBomb;
    //     place_bomb.execute(&mut map, player_index);

    //     // Act
    //     assert!(map.bombs.is_empty());
    // }
}
