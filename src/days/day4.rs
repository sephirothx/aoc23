use std::{collections::HashSet, convert::Infallible, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
struct Card {
    id: i32,
    score: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    cards: Vec<Card>,
}

impl FromStr for Card {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rest) = s.split_once(':').unwrap();
        let id = id.split_whitespace().last().unwrap().parse::<i32>().unwrap();
        let v = rest
            .split('|')
            .map(|group| group
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<HashSet<_>>())
            .collect::<Vec<_>>();
        Ok(Card { id, score: v[0].intersection(&v[1]).count() as u32})
    }
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s.lines().map(|line| Card::from_str(line).unwrap()).collect();
        Ok(Input { cards })
    }
}

pub fn part1(input: Input) -> i32 {
    input.cards.into_iter()
        .filter_map(|c| if c.score > 0 { Some(2_i32.pow(c.score - 1)) } else { None })
        .sum()
}

pub fn part2(input: Input) -> i32 {
    let mut cards_amount = vec![1; input.cards.len()];
    for (i, c) in input.cards.into_iter().enumerate() {
        for j in (i+1)..=(i+c.score as usize) {
            cards_amount[j] += cards_amount[i];
        }
    }
    cards_amount.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        assert_eq!(13, part1(Input::from_str(INPUT).unwrap()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(30, part2(Input::from_str(INPUT).unwrap()));
    }
}
