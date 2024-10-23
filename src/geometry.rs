pub fn manhattan_distance_i64(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as i64
}

pub fn manhattan_distance_usize(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Direction(pub i32, pub i32);

pub const RIGHT: Direction = Direction(0, 1);
pub const LEFT: Direction = Direction(0, -1);
pub const DOWN: Direction = Direction(1, 0);
pub const UP: Direction = Direction(-1, 0);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(25, manhattan_distance_i64((-2, 8), (10, -5)));
        assert_eq!(11, manhattan_distance_usize((2, 8), (10, 5)));
    }
}
