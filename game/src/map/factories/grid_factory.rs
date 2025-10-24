
use crate::{coord::Coord, map::grid::grid::Grid};

pub struct GridFactory {
}

/*

Prepare the 2 dimension Vec by adding enough walls
- the outer line is walled
- the line within the outer wall is destructable
- every even row, and every even column contains a wall

WWWWWWW line 0
W.....W
W.W.W.W line 2
W.....W
W.W.W.W
W.....W
WWWWWWW line 6

 */
impl GridFactory {
    pub fn new(size: usize, player_locations: Vec<Coord>) -> Grid {
        let tiles = Self::generate_grid(size);
        let mut grid = Grid::new(tiles, size);
        Self::remove_destructables_around_users(&mut grid, player_locations);
        grid
    }

    fn generate_grid(size: usize) -> Vec<char> {
        let mut grid = vec!['.'; size * size];

        for row in 0..size {
            for column in 0..size {
                let walled = (row == 0 || row == size - 1 || column == 0 || column == size - 1) || (column.is_multiple_of(2) && row.is_multiple_of(2));

                if walled {
                    grid[row * size + column] = 'W';
                }
            }
        }
        grid
    }

    fn remove_destructables_around_users(grid: &mut Grid, player_positions: Vec<Coord>) {
        for coord in player_positions {
            coord
                .square_3x3()
                .iter()
                .for_each(|c| grid.clear_destructable(*c))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{coord::Coord, map::factories::grid_factory::GridFactory};

    #[test]
    fn test_prepare_grid_example() {
        // Example grid:
        // 0: WWWWWWW
        // 1: W.....W
        // 2: W.W.W.W
        // 3: W.....W
        // 4: W.W.W.W
        // 5: W.....W
        // 6: WWWWWWW
        let size = 7;

        let grid = GridFactory::new(size, [Coord::from(1, 1), Coord::from(5, 5)].to_vec());

        let expected_char_grid = vec![
            'W', 'W', 'W', 'W', 'W', 'W', 'W', 
            'W', ' ', ' ', '.', '.', '.', 'W', 
            'W', ' ', 'W', '.', 'W', '.', 'W', 
            'W', '.', '.', '.', '.', '.', 'W', 
            'W', '.', 'W', '.', 'W', ' ', 'W', 
            'W', '.', '.', '.', ' ', ' ', 'W', 
            'W', 'W', 'W', 'W', 'W', 'W', 'W',
        ];

        assert_eq!(grid.tiles, expected_char_grid);
    }
}