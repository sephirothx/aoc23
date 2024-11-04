use std::{convert::Infallible, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hailstone {
    pos: (i64, i64, i64),
    vel: (i64, i64, i64),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Input {
    hailstones: Vec<Hailstone>,
}

impl FromStr for Hailstone {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s
            .split("@")
            .map(|part| part
                .split(",")
                .map(|c| c.trim().parse::<i64>().unwrap())
                .collect::<Vec<_>>());
        let pos = {
            let p = parts.next().unwrap();
            (p[0], p[1], p[2])
        };
        let vel = {
            let v = parts.next().unwrap();
            (v[0], v[1], v[2])
        };
        Ok(Hailstone { pos, vel })
    }
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hailstones = s
            .lines()
            .map(|line| Hailstone::from_str(line).unwrap())
            .collect();
        Ok(Input { hailstones })
    }
}

pub fn part1(input: Input) -> i32 {
    const MIN: f32 = 200000000000000_f32;
    const MAX: f32 = 400000000000000_f32;
    count_intersections(input, MIN, MAX)
}

fn count_intersections(input: Input, min: f32, max: f32) -> i32 {
    let mut count = 0;
    for i in 0..input.hailstones.len() - 1 {
        for j in i+1..input.hailstones.len() {
            match intersect_2d(&input.hailstones[i], &input.hailstones[j]) {
                Some((x, y))
                if x > min && x < max
                && y > min && y < max => count += 1,
                _ => ()
            }
        }
    }
    count
}

fn intersect_2d(h1: &Hailstone, h2: &Hailstone) -> Option<(f32,f32)> {
    let (x1, y1, _) = h1.pos;
    let (vx1, vy1, _) = h1.vel;
    let (x2, y2, _) = h2.pos;
    let (vx2, vy2, _) = h2.vel;

    let denominator = vx1 * vy2 - vy1 * vx2;

    if denominator == 0 {
        return None;
    }

    let t = ((x2 - x1) * vy2 - (y2 - y1) * vx2) as f32 / denominator as f32;
    let s = ((x2 - x1) * vy1 - (y2 - y1) * vx1) as f32 / denominator as f32;

    if t < 0.0 || s < 0.0 {
        return None;
    }

    // Calculate intersection point using t or s
    let intersection_x = x1 as f32 + t * vx1 as f32;
    let intersection_y = y1 as f32 + t * vy1 as f32;

    Some((intersection_x, intersection_y))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn hailstone_from_str() {
        let h = Hailstone { pos: (19, 13, 30), vel: (-2,  1, -2) };
        assert_eq!(h, Hailstone::from_str("19, 13, 30 @ -2,  1, -2").unwrap());
    }

    #[test]
    fn test_part1() {
        assert_eq!(2, count_intersections(Input::from_str(INPUT).unwrap(), 7.0, 27.0));
    }
}
