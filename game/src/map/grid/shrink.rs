use crate::coord::Coord;

pub fn calculate_shrink_location(
    shrink_number: usize,
    size: usize
) -> Option<Coord> {
    let inner_length = size - 2;
    if size < 2 || shrink_number >= (inner_length * inner_length) {
        return None;
    }
    spiral_coord(size - 2, shrink_number, 0)
}

fn spiral_coord(size: usize, mut n: usize, layer: usize) -> Option<Coord> {
    if size == 0 {
        return None;
    } else if size == 1 {
        return get_coord_with_offset(layer, layer);
    }
    let perimiter = calculate_perimiter(size);
    if n < perimiter {
        let start = layer;
        let end = layer + size - 1;
        if is_top_row(n, size) {
            get_coord_with_offset(start + n, start)
        } else if is_right_column(n,size) {
            n -= size - 1;
            get_coord_with_offset(end, start + n)
        } else if is_bottom_row(n,size) {
            n -= 2 * (size - 1);
            get_coord_with_offset(end - n, end)
        } else {
            n -= 3 * (size - 1);
            get_coord_with_offset(start, end - n)
        }
    }else {
        spiral_coord(size - 2, n - perimiter, layer + 1)
    }
}

#[inline(always)]
fn calculate_perimiter(size: usize) -> usize {
    4 * (size - 1)
}

#[inline(always)]
fn is_top_row(n: usize, size: usize) -> bool {
    n < size - 1
}

#[inline(always)]
fn is_right_column(n: usize, size: usize) -> bool {
    n < 2 * (size - 1)
}

#[inline(always)]
fn is_bottom_row(n: usize, size: usize) -> bool {
   n < 3 * (size - 1)
}

#[inline(always)]
fn get_coord_with_offset(col: usize, row: usize) -> Option<Coord> {
    Some(Coord::from(col + 1, row + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shrink_location() {
        assert_eq!(calculate_shrink_location(0, 5), Some(Coord::from(1, 1))); // Top-left corner
        assert_eq!(calculate_shrink_location(1, 5), Some(Coord::from(2, 1))); // Top row
        assert_eq!(calculate_shrink_location(2, 5), Some(Coord::from(3, 1)));
        assert_eq!(calculate_shrink_location(3, 5), Some(Coord::from(3, 2)));
        assert_eq!(calculate_shrink_location(4, 5), Some(Coord::from(3, 3)));
        assert_eq!(calculate_shrink_location(5, 5), Some(Coord::from(2, 3)));
        assert_eq!(calculate_shrink_location(6, 5), Some(Coord::from(1, 3)));
        assert_eq!(calculate_shrink_location(7, 5), Some(Coord::from(1, 2)));
        assert_eq!(calculate_shrink_location(8, 5), Some(Coord::from(2, 2))); // center

        // moet fail of none opleveren
        assert_eq!(calculate_shrink_location(25, 7), None);
    }
}
