use crate::map::commands::traits::player_command::PlayerCommand;
use crate::map::map::Map;

pub struct Wait;

impl PlayerCommand for Wait {
    fn try_execute(&self, _map: &mut Map, _player_index: usize) {
        // Do nothing
    }
}

#[cfg(test)]
mod tests {
    use crate::coord::Coord;
    use crate::map::commands::traits::player_command::PlayerCommand;
    use crate::map::commands::wait::Wait;
    use crate::map::map::Map;
    use crate::map::player::Player;
    use crate::map::structs::map_config::MapConfig;

    #[test]
    fn wait_should_not_move() {
        //Arrange
        let start = Coord::from(5, 5);
        let mut map = Map::new(MapConfig { size: 11, ..MapConfig::default() },  vec![]);
        map.players = vec![Player::new("player1".to_string(), start, 0)];

        // Act
        let wait = Wait;
        wait.try_execute(&mut map, 0);

        // Assert
        let new_pos = map.players[0].position;
        assert_eq!(new_pos.row, start.row);
        assert_eq!(new_pos.col, start.col);
    }

    #[test]
    fn wait_should_not_bomb() {
        // Arrange
        let config = MapConfig { size: 7, ..MapConfig::default() };
        let mut map = Map::new(config,  vec![]);

        // Act
        let wait = Wait;
        wait.try_execute(&mut map, 0);

        // Assert
        assert!(map.bombs.is_empty());
    }
}