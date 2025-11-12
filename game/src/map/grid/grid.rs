use crate::{coord::Coord, map::grid::cell::CellType};

pub struct Grid {
    pub tiles: Vec<char>,
    size: usize,
}

impl Grid {
    pub fn new(tiles: Vec<char>, size: usize) -> Self {
        Self {
            tiles: tiles,
            size: size
        }
    }

    pub fn cell_type(&self, position: Coord) -> CellType {
        if self.out_of_bounds(&position){
            return CellType::Wall;
        }
        let idx = self.cell_index(&position);
        CellType::from_char(self.tiles[idx])
    }

    pub fn cell_index(&self, position: &Coord) -> usize {
        position.row.get() * self.size + position.col.get()
    }

    pub fn out_of_bounds(&self, position: &Coord) -> bool {
        !position.is_valid(self.size, self.size)
    }

    pub(crate) fn clear_destructable(&mut self, location: Coord) {
        if self.cell_type(location) == CellType::Destroyable {
            self.set_cell(location, CellType::Empty);
        }
    }

    pub(crate) fn set_wall(&mut self, position: Coord) {
        if position.is_valid(self.size, self.size) {
            self.set_cell(position, CellType::Wall);
        }
    }

    pub(crate) fn set_cell(&mut self, position: Coord, cell_type: CellType) {
        if position.is_valid(self.size, self.size) {
            let idx = self.cell_index(&position);
            self.tiles[idx] = cell_type.as_char();
        }
    }


    pub fn can_move_to(&self, coord: Coord) -> bool {
        self.cell_type(coord) == CellType::Empty
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_type_returns_wall_out_of_bounds() {
        //Arrange
        let tiles = vec![' '; 5 * 5];
        let grid = Grid::new(tiles, 5);

        // Act & Assert
        assert_eq!(grid.cell_type(Coord::from(5, 5)), CellType::Wall);
        assert_eq!(grid.cell_type(Coord::from(10, 0)), CellType::Wall);
        assert_eq!(grid.cell_type(Coord::from(0, 10)), CellType::Wall);
    }

    #[test]
    fn test_cell_type_reads_correct_cell() {
        //Arrange
        let mut tiles = vec![' '; 3 * 3];
        tiles[0] = 'W'; // (0, 0)
        tiles[1] = '.'; // (1, 0)
        tiles[3] = 'P'; // (0, 1)
        tiles[4] = 'B'; // (1, 1)
        let grid = Grid::new(tiles, 3);

        // Act & Assert
        assert_eq!(grid.cell_type(Coord::from(0, 0)), CellType::Wall);
        assert_eq!(grid.cell_type(Coord::from(1, 0)), CellType::Destroyable);
        assert_eq!(grid.cell_type(Coord::from(0, 1)), CellType::Player);
        assert_eq!(grid.cell_type(Coord::from(1, 1)), CellType::Bomb);
        assert_eq!(grid.cell_type(Coord::from(2, 2)), CellType::Empty);
    }

    #[test]
    fn test_cell_index_calculation() {
        //Arrange
        let grid = Grid::new(vec![' '; 7 * 7], 7);

        // Act & Assert
        assert_eq!(grid.cell_index(&Coord::from(0, 0)), 0);
        assert_eq!(grid.cell_index(&Coord::from(3, 0)), 3);
        assert_eq!(grid.cell_index(&Coord::from(0, 1)), 7);
        assert_eq!(grid.cell_index(&Coord::from(3, 2)), 2 * 7 + 3);
    }

    #[test]
    fn test_out_of_bounds_detection() {
        //Arrange
        let grid = Grid::new(vec![' '; 5 * 5], 5);

        // Act & Assert
        // Valid bounds
        assert_eq!(grid.out_of_bounds(&Coord::from(0, 0)), false);
        assert_eq!(grid.out_of_bounds(&Coord::from(4, 4)), false);
        assert_eq!(grid.out_of_bounds(&Coord::from(2, 2)), false);

        // Out of bounds
        assert_eq!(grid.out_of_bounds(&Coord::from(5, 0)), true);
        assert_eq!(grid.out_of_bounds(&Coord::from(0, 5)), true);
        assert_eq!(grid.out_of_bounds(&Coord::from(5, 5)), true);
    }

    #[test]
    fn test_set_cell_changes_cell_type() {
        //Arrange
        let tiles = vec![' '; 5 * 5];
        let mut grid = Grid::new(tiles, 5);

        //Act
        grid.set_cell(Coord::from(1, 1), CellType::Wall);
        grid.set_cell(Coord::from(2, 2), CellType::Bomb);
        grid.set_cell(Coord::from(3, 3), CellType::Empty);

        //Assert
        assert_eq!(grid.cell_type(Coord::from(1,1)), CellType::Wall);
        assert_eq!(grid.cell_type(Coord::from(2, 2)), CellType::Bomb);
        assert_eq!(grid.cell_type(Coord::from(3,3)), CellType::Empty);
    }

    #[test]
    fn test_clear_destructable_only_clears_destroyable() {
        //Arrange
        let mut tiles = vec![' '; 5 * 5];
        tiles[0] = '.'; // (0, 0) is destroyable
        tiles[1] = 'W'; // (1, 0) is wall
        tiles[2] = ' '; // (2, 0) is empty
        let mut grid = Grid::new(tiles, 5);

        //Act
        grid.clear_destructable(Coord::from(0, 0));
        grid.clear_destructable(Coord::from(1, 0));
        grid.clear_destructable(Coord::from(2, 0));
        
        //Assert
        assert_eq!(grid.cell_type(Coord::from(0, 0)), CellType::Empty);
        assert_eq!(grid.cell_type(Coord::from(1, 0)), CellType::Wall, "Wall should not be cleared");
        assert_eq!(grid.cell_type(Coord::from(2, 0)), CellType::Empty, "Empty should remain empty");
    }

    #[test]
    fn test_can_move_to_only_empty_cells() {
        //Arrange
        let mut tiles = vec![' '; 5 * 5];
        tiles[0] = ' '; // (0, 0) is empty
        tiles[1] = 'W'; // (1, 0) is wall
        tiles[2] = '.'; // (2, 0) is destroyable
        let grid = Grid::new(tiles, 5);

        //Act & Assert
        assert_eq!(grid.can_move_to(Coord::from(0, 0)), true);
        assert_eq!(grid.can_move_to(Coord::from(1, 0)), false);
        assert_eq!(grid.can_move_to(Coord::from(2, 0)), false);
    }

    #[test]
    fn test_can_move_to_out_of_bounds_is_false() {
        //Arrange
        let grid = Grid::new(vec![' '; 5 * 5], 5);

        //Act & Assert
        assert_eq!(grid.can_move_to(Coord::from(5, 5)), false);
        assert_eq!(grid.can_move_to(Coord::from(10, 0)), false);
    }
}

