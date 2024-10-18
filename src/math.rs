use std::cmp::{max, min};

pub fn gcd(a: i64, b: i64) -> i64 {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

pub fn gcd_vec(numbers: Vec<i64>) -> i64 {
    numbers.into_iter().reduce(|a, b| gcd(a, b)).expect("gcd calculation panicked")
}

pub fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

pub fn lcm_vec(numbers: Vec<i64>) -> i64 {
    numbers.into_iter().reduce(|a, b| lcm(a, b)).expect("lcm calculation panicked")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(1, gcd(7, 8));
        assert_eq!(7, gcd(28, 49));
        assert_eq!(13, gcd(13, 39));
    }

    #[test]
    fn test_gcd_vec() {
        let v = vec![8, 20, 48, 12];
        assert_eq!(4, gcd_vec(v));
    }

    #[test]
    fn test_lcm() {
        assert_eq!(3, lcm(3, 1));
        assert_eq!(8, lcm(2, 8));
        assert_eq!(42, lcm(6, 7));
    }

    #[test]
    fn test_lcm_vec() {
        let v = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(7 * 6 * 5 * 2, lcm_vec(v));
    }
}
