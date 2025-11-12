use crate::{coord::Coord, map::{grid::cell::CellType, map::Map}};

#[derive(Clone, Debug)]
pub struct Bomb {
    pub position: Coord,
    pub timer: usize,
    pub player_id: usize
}

impl Bomb {
    pub fn new(position: Coord, timer: usize, player_id: usize) -> Bomb {
        Bomb {
            position,
            timer,
            player_id
        }
    }

    pub fn explosion_locations(&self, map: &Map) -> Vec<Coord> {
        let mut locations = vec![self.position];
        let directions = [
            |c: Coord| c.move_up(),
            |c: Coord| c.move_down(),
            |c: Coord| c.move_left(),
            |c: Coord| c.move_right(),
        ];

        for direction in directions.iter() {
            let mut current_loc = Some(self.position);
            for _ in 1..= map.map_settings.bomb_radius {
                current_loc = current_loc.and_then(direction);

                if let Some(loc) = current_loc {
                    let cell_type = map.grid.cell_type(loc);
                    match cell_type {
                        CellType::Wall => {
                            break;
                        }
                        CellType::Destroyable => {
                            locations.push(loc);
                            break;
                        }
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

        let bomb = Bomb::new(Coord::from(3, 3), 0, 0);

        let result = bomb.explosion_locations(map);

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

        let bomb = Bomb::new(Coord::from(1, 1), 0, 0);

        let result = bomb.explosion_locations(map);

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

        let bomb = Bomb::new(Coord::from(3, 3), 0, 0);

        let result = bomb.explosion_locations(map);
        
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

        let bomb = Bomb::new(Coord::from(2, 2), 0, 0);

        let result = bomb.explosion_locations(map);

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