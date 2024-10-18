use crate::math::lcm_vec;
use regex::Regex;
use std::{collections::HashMap, str::FromStr, string::ParseError};

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    directions: String,
    network: HashMap<String, (String, String)>,
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();
        let mut lines = s.lines();
        let directions = lines.next().unwrap().to_owned();
        _ = lines.next(); // empty line
        let network = lines
            .filter_map(|line| re.captures(line))
            .map(|caps| (caps[1].to_owned(), (caps[2].to_owned(), caps[3].to_owned())))
            .collect();
        Ok(Input {
            directions,
            network,
        })
    }
}

pub fn part1(input: Input) -> i64 {
    let mut step = 0;
    let mut current_location: &String = &"AAA".to_owned();
    while current_location != "ZZZ" {
        current_location = match input.directions.as_bytes()[step % input.directions.len()] {
            b'L' => &input.network[current_location].0,
            b'R' => &input.network[current_location].1,
            _ => unreachable!(),
        };
        step += 1;
    }
    step as i64
}

pub fn part2(input: Input) -> i64 {
    let starts: Vec<_> = input
        .network
        .keys()
        .filter(|key| key.ends_with("A"))
        .collect();
    let mut steps: Vec<i64> = Vec::new();
    for start in starts {
        println!("{:?}", start);
        let mut step = 0;
        let mut current_location = start;
        while !current_location.ends_with("Z") {
            current_location = match input.directions.as_bytes()[step % input.directions.len()] {
                b'L' => &input.network[current_location].0,
                b'R' => &input.network[current_location].1,
                _ => unreachable!(),
            };
            step += 1;
        }
        println!("{step}");
        steps.push(step as i64);
    }
    lcm_vec(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const INPUT2: &str = "LR

AAA = (AAB, XXX)
AAB = (XXX, AAZ)
AAZ = (AAB, XXX)
BBA = (BBB, XXX)
BBB = (BBC, BBC)
BBC = (BBZ, BBZ)
BBZ = (BBB, BBB)
XXX = (XXX, XXX)";

    #[test]
    fn input_from_str() {
        let expected = Input {
            directions: "LLR".to_owned(),
            network: vec![
                ("AAA".to_owned(), ("BBB".to_owned(), "BBB".to_owned())),
                ("BBB".to_owned(), ("AAA".to_owned(), "ZZZ".to_owned())),
                ("ZZZ".to_owned(), ("ZZZ".to_owned(), "ZZZ".to_owned())),
            ]
            .into_iter()
            .collect(),
        };

        assert_eq!(expected, Input::from_str(INPUT1).unwrap());
    }

    #[test]
    fn test_part_1() {
        assert_eq!(6, part1(Input::from_str(INPUT1).unwrap()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(6, part2(Input::from_str(INPUT2).unwrap()));
    }
}
