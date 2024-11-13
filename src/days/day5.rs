use std::{convert::Infallible, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
struct Transform {
    rules: Vec<(i64, i64, i64)>,
}

impl Transform {
    fn process(&self, i: i64) -> i64 {
        for &(to, from, range) in &self.rules {
            if i >= from && i < from + range {
                return to - from + i;
            }
        }
        i
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    seeds: Vec<i64>,
    transforms: Vec<Transform>,
}

impl FromStr for Transform {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rules = s
            .lines()
            .skip(1)
            .map(|line| {
                let v = line.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>();
                (v[0], v[1], v[2])
            })
            .collect();
        Ok(Transform { rules })
    }
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (seeds, transfoms) = s.split_once("\n\n").unwrap();
        let seeds = seeds.split_whitespace().skip(1).map(|n| n.parse::<i64>().unwrap()).collect();
        let transforms = transfoms.split("\n\n").map(|t| Transform::from_str(t).unwrap()).collect();
        Ok(Input { seeds, transforms })
    }
}

pub fn part1(input: Input) -> i64 {
    input.seeds.into_iter()
        .map(|seed| map_to_location(seed, &input.transforms))
        .min()
        .unwrap()
}

pub fn part2(input: Input) -> i64 {
    input.seeds
        .chunks(2)
        .flat_map(|w| map_range_to_locations(w[0], w[1], &input.transforms))
        .fold(i64::MAX, |min, range| min.min(range.0))
}

pub fn part2_naive(input: Input) -> i64 {
    input.seeds
        .chunks(2)
        .flat_map(|w| (w[0]..w[0]+w[1]).map(|seed| map_to_location(seed, &input.transforms)))
        .min()
        .unwrap()
}

fn map_to_location(seed: i64, transforms: &Vec<Transform>) -> i64 {
    transforms.iter().fold(seed, |i, transform| transform.process(i))
}

fn map_range_to_locations(start: i64, length: i64, transforms: &Vec<Transform>) -> Vec<(i64, i64)> {
    let mut queue = Vec::new();
    queue.push((start, length));
    for transform in transforms {
        let mut temp = Vec::new();
        while let Some((s, l)) = queue.pop() {
            temp.append(&mut range_transform(s, l, transform));
        }
        queue = temp;
    }
    queue
}

fn range_transform(start: i64, length: i64, transform: &Transform) -> Vec<(i64, i64)> {
    let mut queue = Vec::new();
    let mut transformed = Vec::new();
    queue.push((start, length));
    for &rule in &transform.rules {
        let mut temp = Vec::new();
        while let Some((s, l)) = queue.pop() {
            let (a, b, c) = intersect_range(s, l, &rule);
            if let Some(a) = a {
                transformed.push(a);
            }
            if let Some(b) = b {
                temp.push(b);
            }
            if let Some(c) = c {
                temp.push(c);
            }
        }
        queue = temp;
    }
    transformed.append(&mut queue);
    transformed
}

fn intersect_range(start: i64, length: i64, rule: &(i64, i64, i64)) -> (Option<(i64, i64)>, Option<(i64, i64)>, Option<(i64, i64)>) {
    let lo = start;
    let hi = start + length - 1;
    let rule_lo = rule.1;
    let rule_hi = rule.1 + rule.2 - 1;
    let delta = rule.0 - rule.1;
    if lo >= rule_lo {
        if hi <= rule_hi {
            (Some((start + delta, length)), None, None)
        } else if lo <= rule_hi{
            (Some((start + delta, length - hi + rule_hi)), Some((rule_hi + 1, hi - rule_hi)), None)
        } else {
            (None, Some((start, length)), None)
        }
    } else {
        if hi > rule_hi {
            (Some((rule.0, rule.2)), Some((lo, rule_lo - lo)), Some((rule_hi + 1, hi - rule_hi)))
        } else if hi >= rule_lo{
            (Some((rule.0, hi - rule_lo + 1)), Some((lo, rule_lo - lo)), None)
        } else {
            (None, Some((start, length)), None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_map_to_location() {
        let input = Input::from_str(INPUT).unwrap();
        assert_eq!(82, map_to_location(79, &input.transforms));
        assert_eq!(43, map_to_location(14, &input.transforms));
        assert_eq!(86, map_to_location(55, &input.transforms));
        assert_eq!(35, map_to_location(13, &input.transforms));
    }

    #[test]
    fn test_part1() {
        assert_eq!(35, part1(Input::from_str(INPUT).unwrap()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(46, part2(Input::from_str(INPUT).unwrap()));
    }
}
