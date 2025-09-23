use crate::map::commands::traits::player_command::PlayerCommand;
use crate::map::map::Map;

pub struct Wait;

impl PlayerCommand for Wait {
    fn execute(&self, _map: &mut Map, _player_index: usize) {
        // Do nothing
    }
}

#[cfg(test)]
mod tests {
    use crate::coord::Coord;
    use crate::map::commands::move_action::tests::test_move_command;
    use crate::map::commands::traits::player_command::PlayerCommand;
    use crate::map::commands::wait::Wait;
    use crate::map::map::Map;
    use crate::map::structs::map_config::MapConfig;

    #[test]
    fn wait_should_not_move() {
        test_move_command(Wait, Coord::from(3, 3), Coord::from(3, 3));
    }

    #[test]
    fn wait_should_not_bomb() {
        // Arrange
        let config = MapConfig { width: 7, height: 7, ..MapConfig::default() };
        let mut map = Map::create(config);

        // Assert
        let wait = Wait;
        wait.execute(&mut map, 0);

        // Act
        assert!(map.bombs.is_empty());
    }
}