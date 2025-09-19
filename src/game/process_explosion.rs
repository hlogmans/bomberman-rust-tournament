use crate::coord::Coord;
use crate::map::cell::CellType;
use crate::map::map::Map;

#[derive(Debug, Clone)]
pub(super) struct ExplosionEvent {
    origin: Coord,
    affected_tiles: Vec<Coord>,
}

pub(super) struct ExplosionSystem;

impl ExplosionSystem {
    pub(super) fn process(map: &mut Map, alive_players: &mut Vec<usize>) -> bool {
        map.bomb_timer_decrease();

        let exploding_bombs = map.get_exploding_bombs();

        if exploding_bombs.is_empty() {
            return false;
        }

        // 🔧 Remove bombs immediately so subsequent bomb explosion calculations see them as gone
        for bomb in &exploding_bombs {
            map.remove_bomb(*bomb);
        }

        // Compute explosions *after* bombs are removed
        let explosions = exploding_bombs
            .into_iter()
            .map(|bomb| ExplosionEvent {
                origin: bomb,
                affected_tiles: Self::bomb_explosion_locations(bomb, map),
            })
            .collect::<Vec<_>>();

        // Flatten & dedup tiles
        let affected_tiles = explosions.into_iter().flat_map(|e| e.affected_tiles).collect::<Vec<_>>();

        // At the moment not really needed can be needed when adding chain explosions
        //affected_tiles.sort();
        //affected_tiles.dedup();

        for tile in affected_tiles {
            map.clear_destructable(tile);

            if let Some(player_index) = map.get_player_index_at_location(tile) {
                alive_players.retain(|&p| p != player_index);
                if alive_players.len() <= 1 {
                    return true;
                }
            }
        }

        false
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
                    let cell_type = map.cell_type(loc);

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
    use crate::map::structs::map_config::MapConfig;
    use super::*;


    #[test]
    fn test_bomb_explosion_center_clear_path() {
        let map_settings = MapConfig {
            width: 7,
            height: 7,
            ..Default::default()
        };
        let map = &mut Map::create(map_settings);

        map.clear_destructable(Coord::from(3, 3));
        map.clear_destructable(Coord::from(2, 3)); // up
        map.clear_destructable(Coord::from(4, 3)); // down
        map.clear_destructable(Coord::from(3, 2)); // left
        map.clear_destructable(Coord::from(3, 4)); // right

        let result = ExplosionSystem::bomb_explosion_locations(Coord::from(3, 3), map);

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
            width: 5,
            height: 5,
            ..Default::default()
        };
        let map = &mut Map::create(map_settings);

        let result = ExplosionSystem::bomb_explosion_locations( Coord::from(1, 1), map);

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
            width: 5,
            height: 5,
            ..Default::default()
        };
        let map = &mut Map::create(map_settings);

        let result = ExplosionSystem::bomb_explosion_locations( Coord::from(3, 3), map);

        let expected = vec![
            Coord::from(3, 3),
            Coord::from(3, 2),
            //Coord::from(3, 1), Can't be destroyed there is a destrutable in the way at 3,2
            Coord::from(2, 3),
            Coord::from(1, 3),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bomb_explosion_range_1() {
        let map_settings = MapConfig {
            width: 5,
            height: 5,
            ..Default::default()
        };
        let map = &mut Map::create(map_settings);

        let result = ExplosionSystem::bomb_explosion_locations( Coord::from(2, 2), map);

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