// in this module we handle the shrinking of the map after a certain
// amount of turns. The map will shrink by removing outer layers
// of destructible tiles, making the play area smaller.

// the way the shrinking works is that every 5 turns the outermost empty or destructible tiles
// are replaced with a wall, effectively shrinking the playable area. And maybe killing a player
// if they are on one of those tiles.

// shrinking will start on the topleft corner, go clockwise, and then the next line will be processed.
// the calculation is simple, because we also convert a wall to a wall. We should be able to calculate the
// exact tile that is handled by the turn number.

/// Calculate the location of the tile that will be walled. Without regard of amount of turns before a shrink happens.
/// The outermost walls are ignored because they are already a wall.
/// Inner walls are just overriden with a wall.
///
/// Returns the (x, y) coordinates of the tile that will be walled. X and Y are 0-indexed.
///
pub fn calculate_shrink_location(
    shrink_number: usize,
    width: usize,
    height: usize,
) -> Option<(usize, usize)> {
    if width < 2 || height < 2 {
        return None; // No valid shrink location for too small maps
    }
    // Calculate the layer of the shrink
    // Each layer consists of the outermost tiles, which are:
    // top row, right column, bottom row, left column
    // The number of tiles in each layer is:
    // top row: width
    // right column: height - 2 (excluding corners)
    // bottom row: width - 2 (excluding corners)
    // left column: height - 2 (excluding corners)
    // Total tiles in one layer: 2 * width + 2 * height - 4
    let mut layer = 1; // 0 = outermost layer
    let mut shrink_number = shrink_number;

    let total_tiles = (width - 2) * (height - 2);
    if shrink_number >= total_tiles {
        return None; // No valid shrink location for too small maps
    }

    // Calculate the total number of tiles in the current layer
    let mut layer_tiles = calculate_layer_tiles(width, height, layer);

    while shrink_number >= layer_tiles {
        shrink_number -= layer_tiles;
        layer += 1;
        layer_tiles = calculate_layer_tiles(width, height, layer);
    }

    let layer_width = width - 2 * layer;
    let layer_height = height - 2 * layer;

    // calculate the position of the tile in the current layer
    // if the shrink_number is less than the width, it is in the top row
    if shrink_number < layer_width {
        return Some((layer + shrink_number, layer));
    // if the shrink number is less than the width + height, then it is the right column.
    } else if shrink_number < layer_width + layer_height - 1 {
        shrink_number -= layer_width;
        return Some((layer + layer_width - 1, layer + shrink_number + 1));
    } else if shrink_number < 2 * layer_width + layer_height - 2 {
        shrink_number -= layer_width + layer_height - 1;
        // this is the bottom row
        return Some((
            layer + layer_width - shrink_number - 2,
            layer + layer_height - 1,
        ));
    } else {
        shrink_number -= (2 * layer_width) + layer_height - 2;
        return Some((layer, layer + layer_height - shrink_number - 2));
    }
}

/// Calculate the number of tiles in a specific layer of the map.
/// The layer is 0-indexed, where 0 is the outermost layer.
///
fn calculate_layer_tiles(width: usize, height: usize, layer: usize) -> usize {
    if layer < 1 {
        panic!("Layer must be at least 1, 0 is the outermost layer and has no tiles to shrink.");
    }
    if width < 3 || height < 3 {
        panic!("Map must be at least 3x3 to have layers to shrink.");
    }

    // Calculate the number of tiles in the current layer
    let layer_width = width - 2 * layer;
    let layer_height = height - 2 * layer;
    if layer_width <= 0 || layer_height <= 0 {
        return 0; // No valid tiles in this layer
    }
    // Each layer has a top row, right column, bottom row, and left column
    if layer_width == 1 || layer_height == 1 {
        return layer_width * layer_height; // one row or a single cell
    }

    (2 * layer_width) + ((2 * layer_height) - 4)
}

// create a test for the shrink location calculation
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_layer_tiles() {
        assert_eq!(calculate_layer_tiles(5, 5, 1), 8); // Inner layer
        assert_eq!(calculate_layer_tiles(5, 5, 2), 1); // No more layers
        // small map tests
        assert_eq!(calculate_layer_tiles(3, 3, 1), 1); // No more layers
    }
    #[test]
    fn test_shrink_location() {
        assert_eq!(calculate_shrink_location(0, 5, 5), Some((1, 1))); // Top-left corner
        assert_eq!(calculate_shrink_location(1, 5, 5), Some((2, 1))); // Top row
        assert_eq!(calculate_shrink_location(2, 5, 5), Some((3, 1)));
        assert_eq!(calculate_shrink_location(3, 5, 5), Some((3, 2)));
        assert_eq!(calculate_shrink_location(4, 5, 5), Some((3, 3)));
        assert_eq!(calculate_shrink_location(5, 5, 5), Some((2, 3)));
        assert_eq!(calculate_shrink_location(6, 5, 5), Some((1, 3)));
        assert_eq!(calculate_shrink_location(7, 5, 5), Some((1, 2)));
        assert_eq!(calculate_shrink_location(8, 5, 5), Some((2, 2))); // center

        // moet fail of none opleveren
        //assert_eq!(calculate_shrink_location(16, 5, 5), (2, 0));
    }
}
