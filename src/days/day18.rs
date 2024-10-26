use std::{str::FromStr, string::ParseError};

use crate::geometry::*;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Entry {
    dir: Direction,
    len: i64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Input {
    entries: Vec<Entry>,
    actual_entries: Vec<Entry>,
}

fn get_entry_part1(s: &str) -> Entry {
    let mut split = s.split_whitespace();
    let (Some(dir), len, _) = (split.next(), split.next().unwrap().parse::<i64>().unwrap(), split.next()) else { unreachable!() };
    let dir = match dir {
        "R" => RIGHT,
        "L" => LEFT,
        "D" => DOWN,
        "U" => UP,
        _ => unreachable!(),
    };

    Entry {dir, len}
}

fn get_entry_part2(s: &str) -> Entry {
    const DIRECTIONS: [Direction; 4] = [RIGHT, DOWN, LEFT, UP];
    let instruction = s.split_once('#').unwrap().1;
    let dir = DIRECTIONS[(instruction.as_bytes()[5] as char).to_digit(10).unwrap() as usize];
    let len = i64::from_str_radix(&instruction[0..5], 16).unwrap();
    Entry {dir, len}
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entries = s.lines().map(get_entry_part1).collect();
        let actual_entries = s.lines().map(get_entry_part2).collect();
        Ok(Input {entries, actual_entries})
    }
}

pub fn part1(input: Input) -> i64 {
    solve(&input.entries)
}

pub fn part2(input: Input) -> i64 {
    solve(&input.actual_entries)
}

fn solve(entries: &Vec<Entry>) -> i64 {
    let polygon = get_polygon_vertices(entries);
    calculate_area(&polygon) + calculate_perimeter(entries) / 2 + 1
}

fn get_polygon_vertices(entries: &Vec<Entry>) -> Vec<(i64, i64)> {
    let mut vertices = Vec::new();
    let mut current_vertex = (0, 0);

    for entry in entries {
        vertices.push(current_vertex);
        current_vertex = (
            current_vertex.0 + entry.dir.0 as i64 * entry.len,
            current_vertex.1 + entry.dir.1 as i64 * entry.len
        );
    }

    vertices
}

/// https://en.wikipedia.org/wiki/Shoelace_formula
fn calculate_area(polygon: &Vec<(i64, i64)>) -> i64 {
    polygon.windows(2)
        .map(|w| (w[0].0 - w[1].0) * (w[0].1 + w[1].1))
        .sum::<i64>()
        .abs() / 2
}

fn calculate_perimeter(entries: &Vec<Entry>) -> i64 {
    entries.iter()
        .map(|e| e.len)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn entry_from_str() {
        assert_eq!(Entry {dir: RIGHT, len: 6}, get_entry_part1("R 6 (#70c710)"));
        assert_eq!(Entry {dir: RIGHT, len: 0x70c71}, get_entry_part2("R 6 (#70c710)"));
    }

    #[test]
    fn test_calculate_area() {
        let rect = vec![(0, 0), (0, 5), (3, 5), (3, 0)];
        assert_eq!(15, calculate_area(&rect));
    }

    #[test]
    fn test_calculate_perimeter() {
        assert_eq!(38, calculate_perimeter(&Input::from_str(INPUT).unwrap().entries));
    }

    #[test]
    fn test_part1() {
        assert_eq!(62, part1(Input::from_str(INPUT).unwrap()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(952408144115, part2(Input::from_str(INPUT).unwrap()));
    }
}
