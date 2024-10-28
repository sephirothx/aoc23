use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}, str::FromStr, string::ParseError, u32};

use crate::geometry::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    map: Vec<Vec<u32>>,
    size: (i32, i32),
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    heat_loss: u32,
    pos: (i32, i32),
    dir: Option<Direction>,
}

impl Input {
    fn is_in(&self, pos: (i32, i32)) -> bool {
        pos.0 >= 0 && pos.0 < self.size.0 &&
        pos.1 >= 0 && pos.1 < self.size.1
    }

    fn get_cost(&self, pos: (i32, i32)) -> u32 {
        self.map[pos.0 as usize][pos.1 as usize]
    }
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let size = (map.len() as i32, map.first().unwrap().len() as i32);

        Ok(Input {map, size})
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heat_loss.cmp(&other.heat_loss).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1(input: Input) -> u32 {
    let start = (0, 0);
    let end = (input.size.0 - 1, input.size.1 - 1);

    find_min_heat_loss(input, start, end, 1, 3)
}

pub fn part2(input: Input) -> u32 {
    let start = (0, 0);
    let end = (input.size.0 - 1, input.size.1 - 1);

    find_min_heat_loss(input, start, end, 4, 10)
}

fn find_min_heat_loss(input: Input, start: (i32, i32), end: (i32, i32), min_steps: i32, max_steps: i32) -> u32 {
    let mut heap = BinaryHeap::new();
    let mut dist = HashMap::new();
    let mut best = HashMap::new();

    heap.push(State {
        heat_loss: 0,
        pos: start,
        dir: None,
    });

    while let Some(state) = heap.pop() {
        for (pos, dir, cost) in get_positions(&input, state.pos, min_steps, max_steps, state.dir) {
            let next_state = State {
                heat_loss: state.heat_loss + cost,
                pos,
                dir: Some(dir),
            };
            let z = dist.entry((pos, dir)).or_insert(u32::MAX);
            let x = best.entry(pos).or_insert(u32::MAX);
            if next_state.heat_loss < *z {
                *z = next_state.heat_loss;
                *x = (*x).min(next_state.heat_loss);
                heap.push(next_state);
            }
        }
    }

    best[&end]
}

fn get_possible_next_directions(direction: Option<Direction>) -> Vec<Direction> {
    match direction {
        Some(dir) => vec![dir.turn_left(), dir.turn_right()],
        None => vec![RIGHT, DOWN, LEFT, UP]
    }
}

fn get_positions(input: &Input, position: (i32, i32), min: i32, max: i32, direction: Option<Direction>) -> Vec<((i32, i32), Direction, u32)> {
    let mut results = Vec::new();

    for next_direction in get_possible_next_directions(direction) {
        let mut sum = 0;
        let mut current_position = position;

        for dist in 1..=max {
            current_position.0 += next_direction.0;
            current_position.1 += next_direction.1;

            if !input.is_in(current_position) { break; }

            sum += input.get_cost(current_position);
            if dist >= min {
                results.push((current_position, next_direction, sum));
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    const INPUT2: &str = "\
111111111111
999999999991
999999999991
999999999991
999999999991";

    #[test]
    fn test_input() {
        let input = Input::from_str(INPUT).unwrap();
        assert_eq!((13, 13), input.size);
        assert_eq!(2, input.get_cost((0, 0)));
        assert_eq!(8, input.get_cost((4, 7)));
        assert_eq!(3, input.get_cost((12, 12)));
    }

    #[test]
    fn test_get_positions() {
        let input = Input::from_str(INPUT).unwrap();
        let expected = vec![
            ((7, 11), Direction(0, 1), 27),
            ((7, 12), Direction(0, 1), 30),
            ((7, 3), Direction(0, -1), 29),
            ((7, 2), Direction(0, -1), 32),
            ((7, 1), Direction(0, -1), 38),
            ((7, 0), Direction(0, -1), 41)];
        assert_eq!(expected, get_positions(&input, (7,7), 4, 10, Some(DOWN)));
    }

    #[test]
    fn test_part1() {
        assert_eq!(102, part1(Input::from_str(INPUT).unwrap()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(94, part2(Input::from_str(INPUT).unwrap()));
        assert_eq!(71, part2(Input::from_str(INPUT2).unwrap()));
    }
}
