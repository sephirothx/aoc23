use std::{collections::HashSet, str::FromStr, string::ParseError};

use crate::geometry::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    map: Vec<Vec<char>>,
    size: (i32, i32),
    start: (i32, i32),
}

impl Input {
    fn is_in(&self, pos: (i32, i32)) -> bool {
        pos.0 >= 0 && pos.0 < self.size.0 &&
        pos.1 >= 0 && pos.1 < self.size.1
    }

    fn get(&self, pos: (i32, i32)) -> char {
        let x = ((pos.0 % self.size.0) + self.size.0) % self.size.0;
        let y = ((pos.1 % self.size.1) + self.size.1) % self.size.1;
        self.map[x as usize][y as usize]
    }
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::new();
        let mut start = (0, 0);
        for (i, line) in s.lines().enumerate() {
            let mut v = Vec::new();
            for (j, c) in line.chars().enumerate() {
                v.push(c);
                if c == 'S' {
                    start = (i as i32, j as i32);
                }
            }
            map.push(v);
        }
        let size = (map.len() as i32, map[0].len() as i32);

        Ok(Input {map, size, start})
    }
}

pub fn part1(input: Input, steps: usize) -> usize {
    let mut sets = [HashSet::new(), HashSet::new()];
    let mut previous_sets = [HashSet::new(), HashSet::new()];
    sets[0].insert(input.start);
    for step in 0..steps {
        previous_sets[(step+1)%2] = sets[(step+1)%2].clone();
        let mut s = sets[(step+1)%2].clone();
        println!("{} - total: {}, diff: {}", step, sets[step%2].len(), sets[step%2].len() - previous_sets[step%2].len());
        for pos in sets[step%2].difference(&previous_sets[step%2]) {
            s.extend(get_neighbors(*pos, &input));
        }
        sets[(step+1)%2] = s;
    }
    sets[steps%2].len()
}

fn get_neighbors(pos: (i32, i32), input: &Input) -> Vec<(i32, i32)> {
    Direction::iter()
        .map(|dir| (pos.0 + dir.0, pos.1 + dir.1))
        .filter(|p| input.get(*p) != '#')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_part1() {
        assert_eq!(16, part1(Input::from_str(INPUT).unwrap(), 6));
    }

    #[test]
    fn test_get_neighbors() {
        let input = Input::from_str(INPUT).unwrap();
        assert_eq!(vec![(5, 4), (4, 5)], get_neighbors((5, 5), &input));
        assert_eq!(vec![(0, 1), (0, -1), (1, 0), (-1, 0)], get_neighbors((0, 0), &input));
    }

    #[test]
    fn test_modulo() {
        assert_eq!(-1, -1 % 10);
        assert_eq!(0, -10 % 10);
        assert_eq!(-1, -11 % 10);
    }
}
