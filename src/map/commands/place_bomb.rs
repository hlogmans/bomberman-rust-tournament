use crate::map::cell::CellType;
use crate::map::commands::traits::player_command::PlayerCommand;
use crate::map::map::Map;

pub struct PlaceBomb;

impl PlayerCommand for PlaceBomb {
    fn execute(&self, map: &mut Map, player_index: usize) {
        if let Some(pos) = map.get_player_position(player_index) {
            map.set_cell(pos, CellType::Bomb);
            map.add_bomb(pos);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::map::Map;
    use crate::map::commands::traits::player_command::PlayerCommand;
    use crate::map::player::Player;
    use crate::map::structs::map_config::MapConfig;
    use crate::map::cell::CellType;
    use crate::coord::Coord;

    #[test]
    fn test_place_bomb_basic() {
        //Arrange
        let config = MapConfig { width: 7, height: 7, ..MapConfig::default() };
        let mut map = Map::create(config);

        let player_pos = Coord::from(3, 3);
        map.players = vec![Player { name: "player1".to_string(), position: player_pos }];
        map.set_cell(player_pos, CellType::Player);

        // Act
        let place_bomb = PlaceBomb;
        place_bomb.execute(&mut map, 0);

        // Assert
        assert_eq!(map.cell_type(player_pos), CellType::Bomb);
        assert!(map.bombs[0].position == player_pos);
    }

    #[test]
    fn test_place_bomb_no_player() {
        // Arrange
        let config = MapConfig { width: 7, height: 7, ..MapConfig::default() };
        let mut map = Map::create(config);
        // default has 2 players atm
        let player_index = 3;

        // Assert
        let place_bomb = PlaceBomb;
        place_bomb.execute(&mut map, player_index);

        // Act
        assert!(map.bombs.is_empty());
    }
}
