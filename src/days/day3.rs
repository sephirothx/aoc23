use std::{convert::Infallible, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
struct Number {
    n: i32,
    pos: [(i32, i32); 2],
}

impl Number {
    fn around(&self) -> impl Iterator<Item = (i32,i32)> {
        let (x0, y0) = (self.pos[0].0 - 1, self.pos[0].1 - 1);
        let (x1, y1) = (self.pos[1].0 + 1, self.pos[1].1 + 1);
        (x0..=x1).flat_map(move |i| (y0..=y1).map(move |j| (i, j)))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Symbol {
    c: char,
    pos: (i32, i32),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = Vec::new();
        let mut symbols = Vec::new();
        let mut n: i32 = 0;
        let mut digits: usize = 0;

        for (i, line) in s.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c.is_digit(10) {
                    n = n * 10 + c.to_digit(10).unwrap() as i32;
                    digits += 1;
                } else {
                    if c != '.' {
                        symbols.push(Symbol { c, pos: (i as i32, j as i32) });
                    }
                    if n != 0 {
                        numbers.push(Number { n, pos: [(i as i32, (j - digits) as i32), (i as i32, (j - 1) as i32)]});
                        n = 0;
                        digits = 0;
                    }
                }
            }
            if n != 0 {
                numbers.push(Number { n, pos: [(i as i32, (line.len() - digits) as i32), (i as i32, (line.len() - 1) as i32)]});
                n = 0;
                digits = 0;
            }
        }
        Ok(Input { numbers, symbols })
    }
}

pub fn part1(input: Input) -> i32 {
    input.numbers
        .into_iter()
        .filter(|n| n
            .around()
            .into_iter()
            .any(|p| input.symbols.iter().find(|&sym| sym.pos == p).is_some()))
        .map(|n| n.n)
        .sum()
}

pub fn part2(input: Input) -> i32 {
    input.symbols.into_iter()
        .filter_map(|sym| (sym.c == '*').then_some(sym.pos))
        .filter_map(|pos| {
            let surrounding_nums = input.numbers.iter()
                .filter_map(|number| number.around()
                    .any(|p| p == pos)
                    .then_some(number.n))
                .collect::<Vec<_>>();

            (surrounding_nums.len() == 2)
                .then_some(surrounding_nums.iter().product::<i32>())
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part1() {
        assert_eq!(4361, part1(Input::from_str(INPUT).unwrap()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(467835, part2(Input::from_str(INPUT).unwrap()));
    }
}
