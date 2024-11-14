use std::{convert::Infallible, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Mirror {
    size: (usize, usize),
    rows: Vec<u32>,
    cols: Vec<u32>,
}

impl Mirror {
    fn symmetry_value(&self, include: Option<(usize, usize)>) -> Option<usize> {
        if let Some(v) = Self::find_symmetry(&self.rows, include.map(|(r, _)| r)) {
            Some(v * 100)
        } else {
            Self::find_symmetry(&self.cols, include.map(|(_, c)| c))
        }
    }

    fn symmetry_value_with_flip(&self) -> usize {
        let mut flippable = self.clone();
        (0..self.size.0)
            .flat_map(|i| (0..self.size.1).map(move |j| (i, j)))
            .find_map(|pos| {
                flippable.flip_one(pos);
                let result = flippable.symmetry_value(Some(pos));
                flippable.flip_one(pos);
                result
            })
            .unwrap()
    }

    fn flip_one(&mut self, pos: (usize, usize)) {
        self.rows[pos.0] ^= 1 << (self.size.1 - pos.1 - 1);
        self.cols[pos.1] ^= 1 << (self.size.0 - pos.0 - 1);
    }

    fn find_symmetry(v: &[u32], include: Option<usize>) -> Option<usize> {
        let (lo, hi) = include
            .map_or((0, v.len() - 1), |inc| (inc / 2, (inc + v.len()) / 2));
        (lo..hi).find(|&i| {
            let limit = (i + 1).min(v.len() - i - 1);
            (0..limit).all(|j| v[i - j] == v[i + j + 1])
        })
        .map(|i| i + 1)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    mirrors: Vec<Mirror>,
}

impl FromStr for Mirror {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<u32> = s.lines().map(|line| {
            line.chars().fold(0, |acc, c| (acc << 1) | if c == '#' { 1 } else { 0 })
        }).collect();

        let size = (rows.len(), s.lines().next().unwrap().len());
        let cols = (0..size.1).map(|col| {
            (0..size.0).fold(0, |acc, row| (acc << 1) | ((rows[row] >> (size.1 - col - 1)) & 1))
        }).collect();

        Ok(Mirror { size, rows, cols })
    }
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mirrors = s.split("\n\n").map(|m| Mirror::from_str(m).unwrap()).collect();
        Ok(Input { mirrors })
    }
}

pub fn part1(input: Input) -> usize {
    input.mirrors.into_iter().map(|m| m.symmetry_value(None).unwrap()).sum()
}

pub fn part2(input: Input) -> usize {
    input.mirrors.into_iter().map(|m| m.symmetry_value_with_flip()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_part1() {
        assert_eq!(405, part1(Input::from_str(INPUT).unwrap()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(400, part2(Input::from_str(INPUT).unwrap()));
    }
}
