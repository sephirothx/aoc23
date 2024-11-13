use std::{convert::Infallible, str::FromStr};

use crate::geometry::manhattan_distance_usize;

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    galaxies: Vec<(usize, usize)>,
    empty: [Vec<usize>; 2],
}

impl Input {
    fn get_distance(&self, g1: usize, g2: usize, expansion: usize) -> usize {
        let g1 = self.galaxies[g1];
        let g2 = self.galaxies[g2];
        let mut dist = manhattan_distance_usize(g1, g2);
        let get_expansion_cost = |c1: usize, c2: usize, dimension: usize| {
            self.empty[dimension]
                .iter()
                .filter(|&&empty_row| empty_row > c1.min(c2) && empty_row < c1.max(c2))
                .count() * (expansion - 1)
        };
        dist += get_expansion_cost(g1.0, g2.0, 0);
        dist += get_expansion_cost(g1.1, g2.1, 1);
        dist
    }
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut galaxies = Vec::new();
        let size = s.lines().count();
        let mut empty = [Vec::new(), Vec::new()];
        for (i, line) in s.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    galaxies.push((i, j));
                }
            }
        }
        for i in 0..size {
            if !galaxies.iter().any(|&g| g.0 == i) {
                empty[0].push(i);
            }
            if !galaxies.iter().any(|&g| g.1 == i) {
                empty[1].push(i);
            }
        }
        Ok(Input { galaxies, empty })
    }
}

pub fn part1(input: Input) -> usize {
    sum_distances(&input, 2)
}

pub fn part2(input: Input) -> usize {
    sum_distances(&input, 1000000)
}

fn sum_distances(input: &Input, expansion: usize) -> usize {
    let mut sol = 0;
    for g1 in 0..input.galaxies.len()-1 {
        for g2 in g1+1..input.galaxies.len() {
            sol += input.get_distance(g1, g2, expansion);
        }
    }
    sol
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_sum_distances() {
        let input = Input::from_str(INPUT).unwrap();
        assert_eq!(374, sum_distances(&input, 2));
        assert_eq!(1030, sum_distances(&input, 10));
        assert_eq!(8410, sum_distances(&input, 100));
    }
}
