use crate::coord::Coord;
use crate::map::cell::CellType;
use crate::map::map::Map;
pub(super) struct BombProcessor;

impl BombProcessor {
    pub(super) fn process(map: &mut Map){
        map.bomb_timer_decrease();
        let exploding_bombs = map.get_exploding_bombs();
        if exploding_bombs.is_empty() {
            map.explosions = Vec::new();
        }
        Self::remove_bombs(map, &exploding_bombs);
        let mut affected_tiles = Self::get_explosion_locations(map, exploding_bombs);
        let chained_bombs: Vec<Coord> = Self::get_chained_bombs(map, &affected_tiles);
        Self::remove_bombs(map, &chained_bombs);
        affected_tiles.extend(Self::get_explosion_locations(map, chained_bombs));

        //TODO: find faster solution than cloning
        map.explosions = affected_tiles.clone();

        for tile in affected_tiles {
            map.grid.clear_destructable(tile);
            map.kill_at_location(tile);
        }
    }

    fn get_chained_bombs(map: &mut Map, affected_tiles: &Vec<Coord>) -> Vec<Coord> {
        map.bombs.iter()
            .filter(|bomb| affected_tiles.iter().any(|explosion| explosion == &bomb.position))
            .map(|bomb| bomb.position)
            .collect()
    }

    fn remove_bombs(map: &mut Map, bomb_locations: &Vec<Coord>){
        for bomb in bomb_locations {
            map.remove_bomb(*bomb);
        }
    }

    fn get_explosion_locations(map: &mut Map, bomb_locations: Vec<Coord>) -> Vec<Coord>{
         bomb_locations
            .into_iter()
            .flat_map(|bomb| Self::bomb_explosion_locations(bomb, map))
            .collect::<Vec<_>>()
    }

    pub(super) fn bomb_explosion_locations(location: Coord, map: &mut Map) -> Vec<Coord> {
        let mut locations = vec![location];

        let directions = [
            |c: Coord| c.move_up(),
            |c: Coord| c.move_down(),
            |c: Coord| c.move_left(),
            |c: Coord| c.move_right(),
        ];

        // Iterate over each direction and extend the explosion
        for direction in directions.iter() {
            let mut current_loc = Some(location);
            for _ in 1..= map.map_settings.bomb_radius {
                current_loc = current_loc.and_then(direction);

                if let Some(loc) = current_loc {
                    let cell_type = map.grid.cell_type(loc);

                    match cell_type {
                        // A wall stops the explosion completely in this direction.
                        CellType::Wall => {
                            break;
                        }
                        // A destructible block stops the explosion, but is still destroyed.
                        // So we add its location and then stop.
                        CellType::Destroyable => {
                            locations.push(loc);
                            break;
                        }
                        // Empty space, a player, or a bomb will be affected, and the explosion continues.
                        _ => {
                            locations.push(loc);
                        }
                    }
                } else {
                    break;
                }
            }
        }

        locations
    }
}

#[cfg(test)]
mod tests {
    use crate::map::player::Player;
    use crate::map::structs::map_config::MapConfig;
    use super::*;


    #[test]
    fn test_bomb_explosion_center_clear_path() {
        let map_settings = MapConfig {
            size: 7,
            ..Default::default()
        };

        let players = vec![Player::new("Player 1".to_string(), Coord::from(1, 1), 0), Player::new("Player 2".to_string(), Coord::from(1, 6), 1)];

        let  map = &mut Map::new(map_settings, players);

        map.grid.clear_destructable(Coord::from(3, 3));
        map.grid.clear_destructable(Coord::from(2, 3)); // up
        map.grid.clear_destructable(Coord::from(4, 3)); // down
        map.grid.clear_destructable(Coord::from(3, 2)); // left
        map.grid.clear_destructable(Coord::from(3, 4)); // right

        let result = BombProcessor::bomb_explosion_locations(Coord::from(3, 3), map);

        let expected = vec![
            Coord::from(3, 3), // center
            Coord::from(2, 3),
            Coord::from(3, 2), // up
            Coord::from(4, 3),
            Coord::from(3, 4), // down
            Coord::from(1, 3),
            Coord::from(3, 1), // left
            Coord::from(5, 3),
            Coord::from(3, 5), // right
        ];

        assert_eq!(result.len(), expected.len());

        for coord in expected {
            assert_eq!(result.contains(&coord), true);
        }
    }

    #[test]
    fn test_bomb_explosion_corner() {
        let map_settings = MapConfig {
            size: 7,
            ..Default::default()
        };

        let players = vec![Player::new("Player 1".to_string(), Coord::from(1, 1), 0), Player::new("Player 2".to_string(), Coord::from(1, 6), 1)];

        let  map = &mut Map::new(map_settings, players);

        let result = BombProcessor::bomb_explosion_locations(Coord::from(1, 1), map);

        let expected = vec![
            Coord::from(1, 1),
            Coord::from(1, 2),
            Coord::from(1, 3),
            Coord::from(2, 1),
            Coord::from(3, 1),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bomb_explosion_corner_with_destructable() {
        let map_settings = MapConfig {
            size: 7,
            ..Default::default()
        };

        let players = vec![Player::new("Player 1".to_string(), Coord::from(1, 1), 0), Player::new("Player 2".to_string(), Coord::from(1, 6), 1)];

        let  map = &mut Map::new(map_settings, players);

        let result = BombProcessor::bomb_explosion_locations(Coord::from(3, 3), map);

        let expected = vec![
            Coord::from(3, 3),
            Coord::from(3, 2),
            Coord::from(3, 4),
            Coord::from(2, 3),
            Coord::from(4, 3),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bomb_explosion_range_1() {
        let map_settings = MapConfig {
            size: 7,
            ..Default::default()
        };

        let players = vec![Player::new("Player 1".to_string(), Coord::from(1, 1), 0), Player::new("Player 2".to_string(), Coord::from(1, 6), 1)];

        let map = &mut Map::new(map_settings, players);

        let result = BombProcessor::bomb_explosion_locations(Coord::from(2, 2), map);

        let expected = vec![
            Coord::from(2, 2),
            Coord::from(2, 1),
            Coord::from(2, 3),
            Coord::from(1, 2),
            Coord::from(3, 2),
        ];
        assert_eq!(result, expected);
    }
}