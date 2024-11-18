use std::{collections::HashMap, convert::Infallible, str::FromStr};

use crate::geometry::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Input {
    map: Vec<Vec<char>>,
    size: usize,
}

impl Input {
    fn calculate_load(&self) -> usize {
        let mut load = 0;
        for i in 0..self.size {
            for j in 0..self.size {
                if self.map[i][j] == 'O' {
                    load += self.size - i;
                }
            }
        }
        load
    }

    fn tilt(&mut self, dir: Direction) {
        let range: Vec<_> = if dir.0 + dir.1 == -1 {
            (0..self.size).collect()
        } else {
            (0..self.size).rev().collect()
        };
        for i in 0..self.size {
            for j in &range {
                let row = i * (dir.1.abs() as usize) + j * (dir.0.abs() as usize);
                let col = i * (dir.0.abs() as usize) + j * (dir.1.abs() as usize);
                if self.map[row][col] == 'O' {
                    let (mut next_r, mut next_c) = (row as i32 + dir.0, col as i32 + dir.1);
                    while next_r >= 0 && next_r < self.size as i32 &&
                        next_c >= 0 && next_c < self.size as i32 &&
                        self.map[next_r as usize][next_c as usize] == '.' {
                            (next_r, next_c) = (next_r as i32 + dir.0, next_c as i32 + dir.1);
                    }
                    self.map[row][col] = '.';
                    self.map[(next_r - dir.0) as usize][(next_c - dir.1) as usize] = 'O';
                }
            }
        }
    }

    fn tilt_cycle(&mut self) {
        self.tilt(UP);
        self.tilt(LEFT);
        self.tilt(DOWN);
        self.tilt(RIGHT);
    }
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Vec<Vec<char>> = s
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let rows = map.len();
        Ok(Input { map, size: rows })
    }
}

pub fn part1(input: Input) -> usize {
    let mut input = input;
    input.tilt(UP);
    input.calculate_load()
}

pub fn part2(input: Input) -> usize {
    let mut input = input;
    let mut seen: HashMap<Input, i32> = HashMap::new();
    let mut iteration = 0;
    loop {
        iteration += 1;
        input.tilt_cycle();
        if let Some(&first_seen) = seen.get(&input) {
            let period = iteration - first_seen;
            let offset = (1000000000 - first_seen) % period;
            return seen
                .into_iter()
                .find_map(|(state, i)| (i == first_seen + offset).then_some(state.calculate_load()))
                .unwrap();
        }
        seen.insert(input.clone(), iteration);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part1() {
        assert_eq!(136, part1(INPUT.parse().unwrap()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(64, part2(INPUT.parse().unwrap()));
    }
}
