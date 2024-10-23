use std::{collections::{HashMap, HashSet, LinkedList}, str::FromStr, string::ParseError};
use crate::geometry::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    map: HashMap<(i32, i32), u8>,
    size: (i32, i32),
}

#[derive(Debug, PartialEq, Eq)]
enum Action {
    Move(Direction),
    Split(Direction, Direction),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Beam {
    pos: (i32, i32),
    dir: Direction,
}

impl Input {
    fn new(map: HashMap<(i32, i32), u8>, size: (i32, i32)) -> Self {
        Input { map, size }
    }

    fn is_in(&self, pos: (i32, i32)) -> bool {
        pos.0 >= 0 && pos.0 < self.size.0 &&
        pos.1 >= 0 && pos.1 < self.size.0
    }
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        let size = (s.lines().count() as i32, s.lines().last().unwrap().len() as i32);
        for (i, line) in s.lines().enumerate() {
            for (j, c) in line.bytes().enumerate() {
                if c != b'.' {
                    map.insert((i as i32, j as i32), c);
                }
            }
        }
        Ok(Input::new(map, size))
    }
}

impl Beam {
    fn next(&self, dir: Direction) -> Beam {
        Beam { pos: (self.pos.0 + dir.0, self.pos.1 + dir.1), dir }
    }
}

pub fn part1(input: Input) -> usize {
    get_energized_tiles(&input, Beam {pos: (0,0), dir: RIGHT})
}

pub fn part2(input: Input) -> usize {
    get_all_starting_beams(input.size)
        .into_iter()
        .map(|b| get_energized_tiles(&input, b))
        .max()
        .unwrap()
}

fn get_all_starting_beams(size: (i32, i32)) -> Vec<Beam> {
    let mut v = Vec::new();
    for i in 0..size.0 {
        v.push(Beam {pos: (i, 0), dir: RIGHT});
        v.push(Beam {pos: (i, size.1 - 1), dir: LEFT});
    }
    for i in 0..size.1 {
        v.push(Beam {pos: (0, i), dir: DOWN});
        v.push(Beam {pos: (size.0 - 1, i), dir: UP});
    }
    v
}

fn get_energized_tiles(input: &Input, starting_beam: Beam) -> usize {
    let mut stack = LinkedList::new();
    let mut set_beam = HashSet::new();
    let mut set_pos = HashSet::new();
    stack.push_back(starting_beam);

    while let Some(b) = stack.pop_back() {
        if !input.is_in(b.pos) || set_beam.contains(&b){
            continue;
        }
        set_beam.insert(b);
        set_pos.insert(b.pos);
        let c = input.map.get(&b.pos).unwrap_or(&b'.');
        match process_direction(b.dir, *c) {
            Action::Move(d) => stack.push_back(b.next(d)),
            Action::Split(d1, d2) => {
                stack.push_back(b.next(d1));
                stack.push_back(b.next(d2));
            },
        }
    }

    set_pos.len()
}

fn process_direction(dir: Direction, c: u8) -> Action {
    match (dir, c) {
        (_, b'.') |
        (RIGHT | LEFT, b'-') |
        (UP | DOWN, b'|') => Action::Move(dir),

        (RIGHT | LEFT, b'|') => Action::Split(UP, DOWN),
        (UP | DOWN, b'-') => Action::Split(LEFT, RIGHT),

        (RIGHT, b'/') | (LEFT, b'\\') => Action::Move(UP),
        (RIGHT, b'\\') | (LEFT, b'/') => Action::Move(DOWN),
        (UP, b'\\') | (DOWN, b'/') => Action::Move(LEFT),
        (UP, b'/') | (DOWN, b'\\') => Action::Move(RIGHT),

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_input() {
        let input = Input::from_str(INPUT).unwrap();
        assert_eq!((10, 10), input.size);
        assert_eq!(true, input.is_in((0,0)));
        assert_eq!(true, input.is_in((9,9)));
        assert_eq!(false, input.is_in((0,10)));
    }

    #[test]
    fn test_process_direction() {
        assert_eq!(Action::Move(DOWN), process_direction(RIGHT, b'\\'));
        assert_eq!(Action::Move(RIGHT), process_direction(RIGHT, b'-'));
        assert_eq!(Action::Move(UP), process_direction(RIGHT, b'/'));
        assert_eq!(Action::Split(UP, DOWN), process_direction(RIGHT, b'|'));
    }

    #[test]
    fn test_part1() {
        assert_eq!(46, part1(Input::from_str(INPUT).unwrap()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(51, part2(Input::from_str(INPUT).unwrap()));
    }
}
