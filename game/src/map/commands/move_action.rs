use crate::{coord::{Coord, ValidCoord},map::map::Map};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn try_move_player(map: &mut Map, player_index: usize, direction: Direction) {
    let player = &map.players[player_index];
    if let Some(new_pos) = get_new_position(direction, player.position).valid(map.map_settings.size, map.map_settings.size) {
        if map.can_move_to(new_pos) {
            map.players[player_index].move_position(new_pos);
        }
    }
}

fn get_new_position(direction: Direction, coord: Coord) -> Option<Coord>{ 
    match direction {
        Direction::Up => coord.move_up(),
        Direction::Down => coord.move_down(),
        Direction::Left => coord.move_left(),
        Direction::Right => coord.move_right()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::coord::Coord;
    use crate::map::commands::move_action::{try_move_player, Direction};
    use crate::map::map::Map;
    use crate::map::player::Player;
    use crate::map::structs::map_config::MapConfig;


    #[test]
    fn test_move_up() {
        // Arrange
        let expected = Coord::from(5, 4);

        // Act & Assert
        test_move_command(Direction::Up, expected)
    }

    #[test]
    fn test_move_up_blocked_by_wall() {
        // Arrange
        let start = Coord::from(5, 5);

        // Act & Assert
        test_move_command_cannot_move(Direction::Up, start)
    }

    #[test]
    fn test_move_down() {
        // Arrange
        let expected = Coord::from(5, 6);

        // Act & Assert
        test_move_command(Direction::Down, expected )
    }
    #[test]
    fn test_move_down_blocked() {
        // Arrange
        let start = Coord::from(5, 5);
        // Act & Assert
        test_move_command_cannot_move(Direction::Down, start)
    }

    #[test]
    fn test_move_left() {
        // Arrange
        let expected = Coord::from(4, 5);
        // Act & Assert
        test_move_command(Direction::Left, expected )
    }

    #[test]
    fn test_move_left_blocked() {
        // Arrange
        let start = Coord::from(5, 5);
        // Act & Assert
        test_move_command_cannot_move(Direction::Left, start)
    }

    #[test]
    fn test_move_right() {
        // Arrange
        let expected = Coord::from(6, 5);
        // Act & Assert
        test_move_command(Direction::Right, expected )
    }
    #[test]
    fn test_move_right_blocked() {
        // Arrange
        let start = Coord::from(5, 5);
        // Act & Assert
        test_move_command_cannot_move(Direction::Right, start)
    }

    pub fn test_move_command(direction: Direction, expected: Coord) {
        //Arrange
        let start = Coord::from(5, 5);
        let mut map = Map::new(MapConfig { size: 11, ..MapConfig::default() },  vec![Player::new("player1".to_string(), start, 0)]);

        // Act
        try_move_player(&mut map, 0, direction);

        // Assert
        let new_pos = map.players[0].position;
        assert_eq!(new_pos.row, expected.row);
        assert_eq!(new_pos.col, expected.col);
    }


    pub fn test_move_command_cannot_move(direction: Direction, start: Coord) {
        //Arrange
        let mut map = Map::new(MapConfig { size: 11, ..MapConfig::default() },  vec![Player::new("player1".to_string(), start, 0)]);
        map.grid.set_wall(start.move_up().expect("Controlled test"));
        map.grid.set_wall(start.move_down().expect("Controlled test"));
        map.grid.set_wall(start.move_left().expect("Controlled test"));
        map.grid.set_wall(start.move_right().expect("Controlled test"));

        // Act
        try_move_player(&mut map, 0, direction);

        // Assert
        let new_pos = map.players[0].position;
        assert_eq!(new_pos.row, start.row);
        assert_eq!(new_pos.col, start.col);
    }
}