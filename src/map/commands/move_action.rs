use crate::coord::Coord;
use crate::map::map::Map;
use crate::map::cell::CellType;

pub fn move_player(map: &mut Map, player_index: usize, new_pos: Coord) {
    if let Some(current) = map.get_player_position(player_index) && map.cell_type(new_pos) == CellType::Empty {
        //this quite a unique way of how the code works needs to be reworked some day
        //the reason this works is when the player lays a bomb he will not be on the map anymore so no need to set this tile to empty
        //next to that we don't wanne remove the bomb
        //this is buggy because if a player lays a bomb the player is not on the map anymore so i hope that bots use player data instead of map data
        //because if not the player can hide in a cheaty way :p
        if map.cell_type(current) != CellType::Bomb {
            map.set_cell(current, CellType::Empty);
        }

        map.set_cell(new_pos, CellType::Player);
        map.set_player_position(player_index, new_pos);
    }
}

#[cfg(test)]
pub mod tests {
    use crate::coord::Coord;
    use crate::map::cell::CellType;
    use crate::map::map::Map;
    use crate::map::commands::traits::player_command::PlayerCommand;
    use crate::map::player::Player;
    use crate::map::structs::map_config::MapConfig;

    pub fn test_move_command<C: PlayerCommand>(command: C, start: Coord, expected: Coord) {
        //Arrange
        let mut map = Map::create(MapConfig { width: 7, height: 11, ..MapConfig::default() });

        map.players = vec![Player { name: "player1".to_string(), position: start }];
        map.set_cell(start, CellType::Player);
        map.set_cell(expected, CellType::Empty);

        // Act
        command.execute(&mut map, 0);

        // Assert
        let new_pos = map.get_player_position(0).unwrap();
        assert_eq!(new_pos.row, expected.row);
        assert_eq!(new_pos.col, expected.col);
    }
}