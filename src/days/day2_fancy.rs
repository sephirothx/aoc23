use std::str::FromStr;

struct Game {
    id: i32,
    rounds: Vec<(i32, i32, i32)>,
}

impl FromStr for Game {
    type Err = Box<dyn std::error::Error>;

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

pub fn part1_v1(input: String) -> i32 {
    input
        .lines()
        .map(|line| Game::from_str(line).unwrap())
        .filter(|game| {
            game.rounds
                .iter()
                .all(|(r, g, b)| *r <= 12 && *g <= 13 && *b <= 14)
        })
        .map(|game| game.id)
        .sum()
}

pub fn part1_v2(input: String) -> i32 {
    input
        .lines()
        .filter_map(|line| {
            let game = Game::from_str(line).unwrap();
            if game
                .rounds
                .iter()
                .all(|(r, g, b)| *r <= 12 && *g <= 13 && *b <= 14)
            {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

pub fn part2(input: String) -> i32 {
    input
        .lines()
        .map(|line| {
            let game = Game::from_str(line).unwrap();
            let max_colors = game
                .rounds
                .iter()
                .fold((0, 0, 0), |(max_r, max_g, max_b), &(r, g, b)| {
                    (max_r.max(r), max_g.max(g), max_b.max(b))
                });
            max_colors.0 * max_colors.1 * max_colors.2
        })
        .sum()
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
    fn test_part1_from_str() {
        assert_eq!(8, part1_v1(INPUT.to_owned()));
    }

    #[test]
    fn test_part1_filter_map() {
        assert_eq!(8, part1_v2(INPUT.to_owned()));
    }

    #[test]
    fn test_part2_from_str() {
        assert_eq!(2286, part2(INPUT.to_owned()));
    }
}
