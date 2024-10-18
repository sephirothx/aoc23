use std::{cmp::max, collections::HashMap};

pub fn part1(input: String) -> i32 {
    let mut solution = 0;
    for line in input.lines() {
        let (game, rounds) = line.split_once(": ").unwrap();
        let id = game.split_once(" ").unwrap().1.parse::<i32>().unwrap();
        let is_valid = rounds
            .split("; ")
            .flat_map(|s| s.split(", "))
            .map(|s| {
                let (n, c) = s.split_once(" ").unwrap();
                (n.parse::<i32>().unwrap(), c)
            })
            .all(|(n, color)| matches!((n, color), (0..=12, "red") | (0..=13, "green") | (0..=14, "blue")));
        solution += if is_valid {id} else {0};
    }
    solution
}

pub fn part2(input: String) -> i32 {
    let mut solution = 0;
    for line in input.lines() {
        let mut max_color_values = HashMap::new();
        let (_, rounds) = line.split_once(": ").unwrap();
        let entries: Vec<_> = rounds
            .split("; ")
            .flat_map(|s| s.split(", "))
            .map(|s| {
                let (n, c) = s.split_once(" ").unwrap();
                (n.parse::<i32>().unwrap(), c)
            })
            .collect();
        for (n, c) in entries {
            max_color_values.entry(c).and_modify(|m| *m = max(*m, n)).or_insert(n);
        }
        solution += max_color_values.values().product::<i32>();
    }
    solution
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        assert_eq!(8, part1(INPUT.to_owned()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2286, part2(INPUT.to_owned()));
    }
}
