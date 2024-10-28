pub fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        return a ^ b;
    }

    let mut a = a.abs();
    let mut b = b.abs();

    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();

    while b != 0 {
        b >>= b.trailing_zeros();
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        b -= a;
    }

    a << shift
}

pub fn gcd_vec(numbers: Vec<i64>) -> i64 {
    numbers
        .into_iter()
        .reduce(|a, b| gcd(a, b))
        .expect("gcd calculation panicked")
}

pub fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

pub fn lcm_vec(numbers: Vec<i64>) -> i64 {
    numbers
        .into_iter()
        .reduce(|a, b| lcm(a, b))
        .expect("lcm calculation panicked")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(1, gcd(7, 8));
        assert_eq!(7, gcd(28, 49));
        assert_eq!(13, gcd(13, 39));
        assert_eq!(69, gcd(0, 69));
        assert_eq!(420, gcd(420, 0));
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
