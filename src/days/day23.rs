use std::{collections::{HashMap, HashSet, VecDeque}, convert::Infallible, i32, str::FromStr};

use crate::geometry::{get_neighbors, Direction, DOWN, LEFT, RIGHT, UP};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Path,
    Tree,
    Slope(Direction),
}

impl Tile {
    fn can_move(&self, dir: Direction, ignore_slopes: bool) -> bool {
        match self {
            Tile::Path => true,
            Tile::Tree => false,
            Tile::Slope(d) => ignore_slopes || d.reverse() != dir,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    map: Vec<Vec<Tile>>,
    size: (i32, i32),
    start: (i32, i32),
    end: (i32, i32),
}

impl Input {
    fn is_in(&self, pos: (i32, i32)) -> bool {
        pos.0 >= 0 && pos.0 < self.size.0 &&
        pos.1 >= 0 && pos.1 < self.size.1
    }

    fn get_at(&self, pos: (i32, i32)) -> Tile {
        self.map[pos.0 as usize][pos.1 as usize]
    }

    fn get_neighbors(&self, pos: (i32, i32), ignore_slopes: bool) -> Vec<(i32, i32)> {
        Direction::iter()
            .filter_map(|d| {
                let next = (pos.0 + d.0, pos.1 + d.1);
                if self.is_in(next) && self.get_at(next).can_move(d, ignore_slopes) {
                    Some(next)
                }
                else {
                    None
                }
            })
            .collect()
    }

    fn get_nodes(&self) -> Vec<(i32, i32)> {
        let mut v = Vec::new();
        v.push(self.start);
        for row in 1..self.size.0 - 1 {
            for col in 1..self.size.1 - 1 {
                if self.get_at((row, col)) == Tile::Path &&
                    get_neighbors((row, col)).all(|n| matches!(self.get_at(n), Tile::Slope(_) | Tile::Tree)) {
                        v.push((row, col));
                    }
            }
        }
        v.push(self.end);
        v
    }

    fn build_graph(&self, ignore_slopes: bool) -> HashMap<(i32, i32), Vec<((i32, i32), i32)>> {
        let mut h = HashMap::new();
        let nodes = self.get_nodes();
        for node in nodes.clone() {
            let mut v = Vec::new();
            let mut q = VecDeque::new();
            let mut visited = HashSet::new();
            q.push_back((node, 0));
            while let Some((current_pos, distance)) = q.pop_front() {
                visited.insert(current_pos);
                if current_pos != node && nodes.contains(&current_pos) {
                    v.push((current_pos, distance));
                    continue;
                }
                for next in self.get_neighbors(current_pos, ignore_slopes) {
                    if visited.contains(&next) { continue; }
                    q.push_back((next, distance + 1));
                }
            }
            h.insert(node, v);
        }
        h
    }
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|line| line.chars()
                .map(|c| match c {
                    '.' => Tile::Path,
                    '#' => Tile::Tree,
                    '>' => Tile::Slope(RIGHT),
                    '<' => Tile::Slope(LEFT),
                    'v' => Tile::Slope(DOWN),
                    '^' => Tile::Slope(UP),
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let size = (map.len() as i32, map[0].len() as i32);
        let start = (0, map[0].iter().position(|t| *t == Tile::Path).unwrap() as i32);
        let end = (size.0 - 1, map.last().unwrap().iter().position(|t| *t == Tile::Path).unwrap() as i32);
        Ok(Input { map, size, start, end })
    }
}

pub fn part1(input: Input) -> i32 {
    let graph: HashMap<(i32, i32), Vec<((i32, i32), i32)>> = input.build_graph(false);
    dfs(&graph, input.start, input.end, &mut Vec::new())
}

pub fn part2(input: Input) -> i32 {
    let graph = input.build_graph(true);
    dfs(&graph, input.start, input.end, &mut Vec::new())
}

fn dfs(
    adj_list: &HashMap<(i32, i32), Vec<((i32, i32), i32)>>,
    curr: (i32, i32),
    end: (i32, i32),
    visited: &mut Vec<(i32, i32)>
) -> i32 {
    if curr == end { return 0; }
    if visited.contains(&curr) { return i32::MIN; }
    visited.push(curr);
    let mut max = i32::MIN;
    for &(pos, dist) in &adj_list[&curr] {
        max = max.max(dist + dfs(adj_list, pos, end, visited));
    }
    visited.pop();
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn input_from_str() {
        let input = Input::from_str(INPUT).unwrap();
        assert_eq!((23, 23), input.size);
        assert_eq!((0, 1), input.start);
        assert_eq!((22, 21), input.end);
        assert_eq!(Tile::Path, input.get_at((5, 16)));
        assert_eq!(Tile::Tree, input.get_at((2, 11)));
        assert_eq!(Tile::Slope(DOWN), input.get_at((4, 11)));
    }

    #[test]
    fn input_get_neighbors() {
        let input = Input::from_str(INPUT).unwrap();
        assert_eq!(vec![(1, 1)], input.get_neighbors((0, 1), true));
        assert_eq!(vec![(3, 12), (4, 11)], input.get_neighbors((3, 11), false));
    }

    #[test]
    fn tile_can_move() {
        assert_eq!(true, Tile::Path.can_move(RIGHT, false));
        assert_eq!(false, Tile::Tree.can_move(RIGHT, false));
        assert_eq!(true, Tile::Slope(RIGHT).can_move(RIGHT, false));
        assert_eq!(false, Tile::Slope(RIGHT).can_move(LEFT, false));
        assert_eq!(true, Tile::Slope(RIGHT).can_move(LEFT, true));
    }

    #[test]
    fn test_part1() {
        assert_eq!(94, part1(Input::from_str(INPUT).unwrap()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(154, part2(Input::from_str(INPUT).unwrap()));
    }
}
