use std::{collections::HashMap, convert::Infallible, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand {
    cards: Vec<char>,
    value: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    hands: Vec<Hand>,
}

const NUM_OF_CARDS: usize = 14;

impl FromStr for Hand {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, value) = s.split_once(' ').unwrap();
        let cards = cards.chars().collect();
        let value = value.parse().unwrap();
        Ok(Hand { cards, value })
    }
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hands = s.lines().map(|line| line.parse().unwrap()).collect();
        Ok(Input { hands })
    }
}

impl Hand {
    fn get_score(&self) -> usize {
        let mut score = 0;
        let mut card_counts = HashMap::new();
        for (i, &card) in self.cards.iter().enumerate() {
            *card_counts.entry(card).or_insert(0) += 1;
            score += Self::get_card_value(card) * NUM_OF_CARDS.pow(4 - i as u32);
        }
        let jokers = card_counts.remove(&'*').unwrap_or(0);
        let mut frequencies = card_counts.values().copied().collect::<Vec<_>>();
        frequencies.sort_unstable();
        if let Some(last) = frequencies.last_mut() {
            *last += jokers;
        }
        let multiplier = match frequencies.as_slice() {
            [.., 5] => 6,
            [.., 4] => 5,
            [.., 2, 3] => 4,
            [.., 3] => 3,
            [.., 2, 2] => 2,
            [.., 2] => 1,
            _ => 0,
        };
        score + NUM_OF_CARDS.pow(5) * multiplier
    }

    fn get_card_value(card: char) -> usize {
        match card {
            '2'..='9' => card as usize - '1' as usize,
            'T' => 9,
            'J' => 10,
            'Q' => 11,
            'K' => 12,
            'A' => 13,
            _ => 0,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_score().cmp(&other.get_score())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1(input: Input) -> usize {
    let mut hands = input.hands.clone();
    hands.sort_unstable();
    calculate_total_winnings(hands)
}

pub fn part2(input: Input) -> usize {
    let mut hands = input.hands
        .into_iter()
        .map(|hand| Hand {
            cards: hand.cards.into_iter().map(|c| if c == 'J' {'*'} else {c}).collect(),
            value: hand.value,
        })
        .collect::<Vec<_>>();
    hands.sort_unstable();
    calculate_total_winnings(hands)
}

fn calculate_total_winnings(hands: Vec<Hand>) -> usize {
    hands.into_iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.value)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1() {
        assert_eq!(6440, part1(INPUT.parse().unwrap()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(5905, part2(INPUT.parse().unwrap()));
    }
}
