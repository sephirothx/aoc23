use std::{convert::Infallible, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: i32,
    rounds: Vec<(i32, i32, i32)>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    games: Vec<Game>,
}

impl FromStr for Game {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rounds) = s.split_once(": ").unwrap();
        let id = id.split_once(" ").unwrap().1.parse().unwrap();
        let rounds = rounds
            .split("; ")
            .map(|round| {
                let mut cubes = (0, 0, 0);
                round.split(", ").for_each(|cube| {
                    let (n, color) = cube.split_once(" ").unwrap();
                    let n = n.parse::<i32>().unwrap();
                    match color {
                        "red" => cubes.0 = n,
                        "green" => cubes.1 = n,
                        "blue" => cubes.2 = n,
                        _ => (),
                    }
                });
                cubes
            })
            .collect();
        Ok(Self { id, rounds })
    }
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let games = s
            .lines()
            .map(|line| Game::from_str(line).unwrap())
            .collect();
        Ok(Self { games })
    }
}

pub fn part1(input: Input) -> i32 {
    input.games
        .into_iter()
        .filter_map(|game| {
            game.rounds
                .iter()
                .all(|(r, g, b)| *r <= 12 && *g <= 13 && *b <= 14)
                .then_some(game.id)
        })
        .sum()
}

pub fn part2(input: Input) -> i32 {
    input.games
        .into_iter()
        .map(|game| game.rounds
                .iter()
                .fold([0, 0, 0], |[max_r, max_g, max_b], &(r, g, b)| [max_r.max(r), max_g.max(g), max_b.max(b)])
                .into_iter()
                .product::<i32>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn game_from_str() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = Game {
            id: 1,
            rounds: vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)],
        };
        assert_eq!(expected, Game::from_str(input).unwrap());
    }

    #[test]
    fn test_part1() {
        assert_eq!(8, part1(Input::from_str(INPUT).unwrap()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2286, part2(Input::from_str(INPUT).unwrap()));
    }
}
