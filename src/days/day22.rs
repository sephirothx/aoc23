use std::{cmp::Ordering, collections::{HashSet, VecDeque}, str::FromStr, string::ParseError};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Brick {
    bounds: Vec<(i32, i32, i32)>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    bricks: Vec<Brick>,
    bound: (i32, i32),
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        self.bounds[0].2.cmp(&other.bounds[0].2)
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Brick {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bounds = s
            .split('~')
            .map(|pos| {
                let coords = pos.split(',').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>();
                (coords[0], coords[1], coords[2])
            })
            .collect();
        Ok(Brick { bounds })
    }
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bound = (0, 0);
        let bricks = s.lines().map(|line| {
            let b = Brick::from_str(line).unwrap();
            let max_brick = b.bounds[1];
            bound = (bound.0.max(max_brick.0 + 1), bound.1.max(max_brick.1 + 1));
            b
        }).collect();
        Ok(Input { bricks, bound })
    }
}

pub fn part1(input: Input) -> usize {
    let (above, below) = compute_fall(&input);
    let mut destroyable = 0;
    for hs in above {
        let mut can_destroy = true;
        for b in hs {
            if below[b].len() < 2 {
                can_destroy = false;
            }
        }
        if can_destroy { destroyable += 1 }
    }
    destroyable
}

pub fn part2(input: Input) -> usize {
    let (above, below) = compute_fall(&input);
    let mut would_fall = Vec::new();
    for b in 0..input.bricks.len() {
        would_fall.push(how_many_would_fall(&above, &below, b));
    }
    would_fall.into_iter().sum()
}

fn compute_fall(input: &Input) -> (Vec<HashSet<usize>>, Vec<HashSet<usize>>) {
    let mut profile = vec![vec![(0, usize::MAX); input.bound.1 as usize]; input.bound.0 as usize];
    let mut sorted_bricks = input.bricks.clone();
    let mut below = vec![HashSet::new(); input.bricks.len()];
    let mut above = vec![HashSet::new(); input.bricks.len()];
    sorted_bricks.sort();
    for (i, brick) in sorted_bricks.into_iter().enumerate() {
        let mut max_h = 0;
        for x in brick.bounds[0].0..=brick.bounds[1].0 {
            for y in brick.bounds[0].1..=brick.bounds[1].1 {
                let (z, brick_id) = profile[x as usize][y as usize];
                if z == 0 { continue }
                if z > max_h {
                    below[i].clear();
                    max_h = z;
                }
                if z >= max_h {
                    below[i].insert(brick_id);
                }
            }
        }
        for x in brick.bounds[0].0..=brick.bounds[1].0 {
            for y in brick.bounds[0].1..=brick.bounds[1].1 {
                let height = brick.bounds[1].2 - brick.bounds[0].2 + 1;
                profile[x as usize][y as usize] = (max_h + height, i);
            }
        }
        for b in &below[i] {
            above[*b].insert(i);
        }
    }
    (above, below)
}

fn how_many_would_fall(above: &Vec<HashSet<usize>>, below: &Vec<HashSet<usize>>, brick: usize) -> usize {
    let mut queue = VecDeque::new();
    let mut hs = HashSet::new();
    queue.push_back(brick);
    while let Some(b) = queue.pop_front() {
        if hs.insert(b) {
            queue.extend(above[b]
                .iter()
                .filter(|&&bb| below[bb].is_subset(&hs)));
        }
    }
    hs.len() - 1
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn test_part1() {
        assert_eq!(5, part1(Input::from_str(INPUT).unwrap()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(7, part2(Input::from_str(INPUT).unwrap()));
    }

    #[test]
    fn brick_from_str() {
        assert_eq!(Brick { bounds: vec![(1,0,1), (1,2,1)] }, Brick::from_str("1,0,1~1,2,1").unwrap());
        assert_eq!(Brick { bounds: vec![(0,0,2), (2,0,2)] }, Brick::from_str("0,0,2~2,0,2").unwrap());
        assert_eq!(Brick { bounds: vec![(1,1,8), (1,1,9)] }, Brick::from_str("1,1,8~1,1,9").unwrap());
    }
}
