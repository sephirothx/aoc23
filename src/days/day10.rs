use std::{collections::HashSet, str::FromStr, string::ParseError};

use crate::geometry::manhattan_distance_usize;

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    map: Vec<Vec<u8>>,
    start: (usize, usize)
}

impl Input {
    fn move_next(&self, from: (usize, usize), to: (usize, usize)) -> Option<(usize, usize)> {
        if manhattan_distance_usize(from, to) != 1 {
            None
        } else {
            let dir = (to.0 as i64 - from.0 as i64, to.1 as i64 - from.1 as i64);
            match (dir, self.map[to.0][to.1]) {
                ((0, 1), b'-') => Some((to.0, to.1 + 1)),
                ((0, -1), b'-') => Some((to.0, to.1 - 1)),
                ((1, 0), b'|') => Some((to.0 + 1, to.1)),
                ((-1, 0), b'|') => Some((to.0 - 1, to.1)),
                ((0, 1), b'7') => Some((to.0 + 1, to.1)),
                ((-1, 0), b'7') => Some((to.0, to.1 - 1)),
                ((1, 0), b'L') => Some((to.0, to.1 + 1)),
                ((0, -1), b'L') => Some((to.0 - 1, to.1)),
                ((0, 1), b'J') => Some((to.0 - 1, to.1)),
                ((1, 0), b'J') => Some((to.0, to.1 - 1)),
                ((0, -1), b'F') => Some((to.0 + 1, to.1)),
                ((-1, 0), b'F') => Some((to.0, to.1 + 1)),
                _ => None
            }
        }
    }
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::new();
        let mut start = (0, 0);
        for (index, line) in input.lines().enumerate() {
            if let Some(s) = line.find("S") {
                start = (index, s);
            }
            map.push(line.as_bytes().to_vec());
        }
        Ok(Input { map, start })
    }
}

pub fn part1(input: Input) -> i64 {
    let mut steps: i64 = 2;
    let mut current: (usize, usize) = input.start;
    let mut next: (usize, usize) = (0, 0);

    for dir in [(0_i64, 1_i64), (1, 0), (0, -1), (-1, 0)] {
        let start = input.start;
        current = ((start.0 as i64 + dir.0) as usize, (start.1 as i64 + dir.1) as usize);
        if let Some(n) = input.move_next(start, current) {
            next = n;
            break;
        }
    }

    while let Some(n) = input.move_next(current, next) {
        current = next;
        next = n;
        steps += 1;
    }

    steps / 2
}

pub fn part2(input: Input) -> i64 {
    let mut solution = 0_i64;
    let mut set = HashSet::new();
    set.insert(input.start);
    let mut current: (usize, usize) = input.start;
    let mut next: (usize, usize) = (0, 0);

    for dir in [(0_i64, 1_i64), (1, 0), (0, -1), (-1, 0)] {
        let start = input.start;
        current = ((start.0 as i64 + dir.0) as usize, (start.1 as i64 + dir.1) as usize);
        if let Some(n) = input.move_next(start, current) {
            next = n;
            break;
        }
    }

    while let Some(n) = input.move_next(current, next) {
        set.insert(current);
        current = next;
        next = n;
    }

    for (i, line) in input.map.iter().enumerate() {
        let mut crossings = 0;
        let mut dir_up: bool = false;
        for (j, ch) in line.iter().enumerate() {
            if set.contains(&(i, j)) {
                match ch {
                    b'|' => crossings += 1,
                    b'F' => dir_up = true,
                    b'L' => dir_up = false,
                    b'J' => crossings += if dir_up {1} else {0},
                    b'7' |
                    b'S' => crossings += if dir_up {0} else {1},
                    _ => ()
                }
            } else {
                solution += crossings % 2;
            }
        }
    }

    solution
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    const INPUT2: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_part1() {
        assert_eq!(8, part1(Input::from_str(INPUT).unwrap()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(10, part2(Input::from_str(INPUT2).unwrap()));
    }

    #[test]
    fn test_move_next() {
        let input = Input::from_str(INPUT).unwrap();
        assert_eq!(None, input.move_next((0, 1), (1, 0)));
        assert_eq!(Some((1, 1)), input.move_next(input.start, (2, 1)));
        assert_eq!(Some((1, 2)), input.move_next((2, 1), (1, 1)));
        assert_eq!(Some((0, 2)), input.move_next((1, 1), (1, 2)));
        assert_eq!(Some((0, 3)), input.move_next((1, 2), (0, 2)));
        assert_eq!(Some((1, 3)), input.move_next((0, 2), (0, 3)));
        assert_eq!(Some((2, 3)), input.move_next((0, 3), (1, 3)));
        assert_eq!(Some((2, 4)), input.move_next((1, 3), (2, 3)));
        assert_eq!(Some((3, 4)), input.move_next((2, 3), (2, 4)));
        assert_eq!(Some((3, 3)), input.move_next((2, 4), (3, 4)));
        assert_eq!(Some((3, 2)), input.move_next((3, 4), (3, 3)));
    }
}
