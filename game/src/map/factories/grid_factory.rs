pub struct GridFactory {
    width: usize,
    height: usize,
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
    pub fn new(width: usize, height: usize) -> Self {
        GridFactory { width, height }
    }

    pub fn prepare_grid(&self) -> Vec<char> {
        let mut grid = vec!['.'; self.width * self.height];

        for row in 0..self.height {
            for column in 0..self.width {
                let walled = (row == 0 || row == self.height - 1 || column == 0 || column == self.width - 1) || (column.is_multiple_of(2) && row.is_multiple_of(2));

                if walled {
                    grid[row * self.width + column] = 'W';
                }
            }
        }
        grid
    }
}

#[cfg(test)]
mod tests {
    use crate::map::factories::grid_factory::GridFactory;

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
        let width = 7;
        let height = 7;

        let factory = GridFactory::new(width, height);
        let grid = factory.prepare_grid();

        let expected = vec![
            'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', '.', '.', '.', '.', '.', 'W', 'W', '.', 'W',
            '.', 'W', '.', 'W', 'W', '.', '.', '.', '.', '.', 'W', 'W', '.', 'W', '.', 'W', '.',
            'W', 'W', '.', '.', '.', '.', '.', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',
        ];

        assert_eq!(grid, expected);
    }
}