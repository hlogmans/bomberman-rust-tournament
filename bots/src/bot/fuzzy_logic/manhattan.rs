use game::coord::Coord;

pub fn manhattan(position_a: Coord, position_b: Coord) -> i32 {
    let dx = (position_a.col.get() as i32 - position_b.col.get() as i32).abs();
    let dy = (position_a.row.get() as i32 - position_b.row.get() as i32).abs();

    dx + dy
}