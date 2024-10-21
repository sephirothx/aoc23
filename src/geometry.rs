pub fn manhattan_distance_i64(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as i64
}

pub fn manhattan_distance_usize(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(25, manhattan_distance_i64((-2, 8), (10, -5)));
        assert_eq!(11, manhattan_distance_usize((2, 8), (10, 5)));
    }
}
