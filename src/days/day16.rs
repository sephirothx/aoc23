use std::{collections::{HashMap, HashSet, LinkedList}, str::FromStr, string::ParseError};

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    map: HashMap<(i32, i32), u8>,
    size: (i32, i32),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Down,
    Up,
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

pub fn part1(input: Input) -> usize {
    get_energized_tiles(&input, Beam {pos: (0,0), dir: Direction::Right})
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
        v.push(Beam {pos: (i, 0), dir: Direction::Right});
        v.push(Beam {pos: (i, size.1 - 1), dir: Direction::Left});
    }
    for i in 0..size.1 {
        v.push(Beam {pos: (0, i), dir: Direction::Down});
        v.push(Beam {pos: (size.0 - 1, i), dir: Direction::Up});
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
        process(&b, *c).into_iter().for_each(|b| stack.push_back(b));
    }

    set_pos.len()
}

fn process(b: &Beam, c: u8) -> Vec<Beam> {
    let mut v = Vec::new();
    match (&b.dir, c) {
        (Direction::Right, b'-' | b'.') => v.push(Beam {pos: (b.pos.0, b.pos.1 + 1), dir: Direction::Right}),
        (Direction::Right, b'/') => v.push(Beam {pos: (b.pos.0 - 1, b.pos.1), dir: Direction::Up}),
        (Direction::Right, b'\\') => v.push(Beam {pos: (b.pos.0 + 1, b.pos.1), dir: Direction::Down}),

        (Direction::Left, b'-' | b'.') => v.push(Beam {pos: (b.pos.0, b.pos.1 - 1), dir: Direction::Left}),
        (Direction::Left, b'/') => v.push(Beam {pos: (b.pos.0 + 1, b.pos.1), dir: Direction::Down}),
        (Direction::Left, b'\\') => v.push(Beam {pos: (b.pos.0 - 1, b.pos.1), dir: Direction::Up}),

        (Direction::Right | Direction::Left, b'|') => {
            v.push(Beam {pos: (b.pos.0 - 1, b.pos.1), dir: Direction::Up});
            v.push(Beam {pos: (b.pos.0 + 1, b.pos.1), dir: Direction::Down});
        },

        (Direction::Down, b'|' | b'.') => v.push(Beam {pos: (b.pos.0 + 1, b.pos.1), dir: Direction::Down}),
        (Direction::Down, b'/') => v.push(Beam {pos: (b.pos.0, b.pos.1 - 1), dir: Direction::Left}),
        (Direction::Down, b'\\') => v.push(Beam {pos: (b.pos.0, b.pos.1 + 1), dir: Direction::Right}),

        (Direction::Up, b'|' | b'.') => v.push(Beam {pos: (b.pos.0 - 1, b.pos.1), dir: Direction::Up}),
        (Direction::Up, b'/') => v.push(Beam {pos: (b.pos.0, b.pos.1 + 1), dir: Direction::Right}),
        (Direction::Up, b'\\') => v.push(Beam {pos: (b.pos.0, b.pos.1 - 1), dir: Direction::Left}),

        (Direction::Down | Direction::Up, b'-') => {
            v.push(Beam {pos: (b.pos.0, b.pos.1 + 1), dir: Direction::Right});
            v.push(Beam {pos: (b.pos.0, b.pos.1 - 1), dir: Direction::Left});
        },
        _ => unreachable!()
    }
    v
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
    fn test_process() {
        assert_eq!(
            vec![Beam {pos: (1, 0), dir: Direction::Down}],
            process(&Beam {pos: (0, 0),  dir: Direction::Right}, b'\\'));
        assert_eq!(
            vec![Beam {pos: (0, 1), dir: Direction::Right}],
            process(&Beam {pos: (0, 0),  dir: Direction::Right}, b'-'));
        assert_eq!(
            vec![Beam {pos: (-1, 0), dir: Direction::Up}],
            process(&Beam {pos: (0, 0),  dir: Direction::Right}, b'/'));
        assert_eq!(
            vec![Beam {pos: (-1, 0), dir: Direction::Up}, Beam {pos: (1, 0), dir: Direction::Down}],
            process(&Beam {pos: (0, 0),  dir: Direction::Right}, b'|'));
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
