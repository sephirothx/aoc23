use std::{str::FromStr, string::ParseError};

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    stories: Vec<Vec<i64>>,
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stories = Vec::new();
        for line in s.lines() {
            let v = line
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect();
            stories.push(v);
        }
        Ok(Input { stories })
    }
}

pub fn part1(input: Input) -> i64 {
    input.stories.iter().map(|story| get_next(story)).sum()
}

pub fn part2(input: Input) -> i64 {
    input.stories.iter().map(|story| get_previous(story)).sum()
}

fn differences(input: &[i64]) -> Vec<i64> {
    input.windows(2).map(|w| w[1] - w[0]).collect()
}

fn get_next(input: &[i64]) -> i64 {
    if input.iter().all(|n| *n == 0) {
        0
    } else {
        input.last().unwrap() + get_next(&differences(input))
    }
}

fn get_previous(input: &[i64]) -> i64 {
    if input.iter().all(|n| *n == 0) {
        0
    } else {
        input.first().unwrap() - get_previous(&differences(input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn input_from_str() {
        let expected = Input {
            stories: vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![1, 3, 6, 10, 15, 21],
                vec![10, 13, 16, 21, 30, 45],
            ],
        };
        assert_eq!(expected, Input::from_str(INPUT).unwrap());
    }

    #[test]
    fn test_get_next() {
        assert_eq!(18, get_next(&[0, 3, 6, 9, 12, 15]));
        assert_eq!(28, get_next(&[1, 3, 6, 10, 15, 21]));
        assert_eq!(68, get_next(&[10, 13, 16, 21, 30, 45]));
    }

    fn test_get_previous() {
        assert_eq!(-3, get_previous(&[0, 3, 6, 9, 12, 15]));
        assert_eq!(0, get_previous(&[1, 3, 6, 10, 15, 21]));
        assert_eq!(5, get_previous(&[10, 13, 16, 21, 30, 45]));
    }

    #[test]
    fn test_part1() {
        assert_eq!(114, part1(Input::from_str(INPUT).unwrap()));
    }

    fn test_part2() {
        assert_eq!(2, part2(Input::from_str(INPUT).unwrap()));
    }
}
