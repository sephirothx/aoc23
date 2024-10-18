type Input<'a> = &'a [(i64, i64)];

pub fn part1_2(input: Input) -> i64 {
    input
        .iter()
        .map(|(t, record)| {
            (1..*t)
                .map(|i| i * (t - i))
                .filter(|i| *i > *record)
                .count() as i64
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(288, part1_2(&[(7, 9), (15, 40), (30, 200)]));
    }

    #[test]
    fn test_part2() {
        assert_eq!(71503, part1_2(&[(71530, 940200)]));
    }
}
