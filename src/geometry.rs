pub fn manhattan_distance_i64(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as i64
}

pub fn manhattan_distance_usize(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

pub fn get_neighbors(pos: (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
    Direction::iter().map(move |dir| (pos.0 + dir.0, pos.1 + dir.1))
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Direction(pub i32, pub i32);

pub const RIGHT: Direction = Direction(0, 1);
pub const LEFT: Direction = Direction(0, -1);
pub const DOWN: Direction = Direction(1, 0);
pub const UP: Direction = Direction(-1, 0);

impl Direction {
    pub fn reverse(&self) -> Self {
        Self(self.0 * -1, self.1 * -1)
    }

    pub fn turn_right(&self) -> Self {
        Self(self.1, self.0 * -1)
    }

    pub fn turn_left(&self) -> Self {
        Self(self.1 * -1, self.0)
    }

    pub fn iter() -> impl Iterator<Item = Direction> {
        [RIGHT, LEFT, DOWN, UP].into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(25, manhattan_distance_i64((-2, 8), (10, -5)));
        assert_eq!(11, manhattan_distance_usize((2, 8), (10, 5)));
    }

    #[test]
    fn test_direction_reverse() {
        assert_eq!(RIGHT, LEFT.reverse());
        assert_eq!(LEFT, RIGHT.reverse());
        assert_eq!(DOWN, UP.reverse());
        assert_eq!(UP, DOWN.reverse());
    }

    #[test]
    fn test_direction_turn_right() {
        assert_eq!(DOWN, RIGHT.turn_right());
        assert_eq!(LEFT, DOWN.turn_right());
        assert_eq!(UP, LEFT.turn_right());
        assert_eq!(RIGHT, UP.turn_right());
    }

    #[test]
    fn test_direction_turn_left() {
        assert_eq!(RIGHT, DOWN.turn_left());
        assert_eq!(DOWN, LEFT.turn_left());
        assert_eq!(LEFT, UP.turn_left());
        assert_eq!(UP, RIGHT.turn_left());
    }

    #[test]
    fn test_get_neighbors() {
        let expected = vec![(5, 6), (5, 4), (6, 5), (4, 5)];
        assert_eq!(expected, get_neighbors((5, 5)).collect::<Vec<_>>());
    }
}
