use std::{str::FromStr, string::ParseError};

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    strings: Vec<String>
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let strings = input.split(",").map(|s| s.to_owned()).collect();
        Ok(Input {strings})
    }
}

pub fn part1(input: Input) -> usize {
    input.strings.iter().map(|s| compute_hash(s)).sum()
}

pub fn part2(input: Input) -> usize {
    let mut solution = 0;
    let mut boxes: [Vec<(String, usize)>; 256] = std::array::from_fn(|_| Vec::new());
    for s in input.strings {
        if let Some((k, n)) = s.split_once("=") {
            let n = n.parse::<usize>().unwrap();
            let b = &mut boxes[compute_hash(k)];
            if let Some(ref mut focus) = b.iter_mut().find(|(s, _)| s == k) {
                focus.1 = n;
            } else {
                b.push((k.to_owned(), n));
            }
        } else {
            let k = s.replace("-", "");
            let b = &mut boxes[compute_hash(&k)];
            *b = b.iter().filter(|(s, _)| *s != k).cloned().collect();
        }
    }
    for (i, b) in boxes.iter().enumerate() {
        for (j, (_, f)) in b.iter().enumerate() {
            solution += (i + 1) * (j + 1) * f;
        }
    }
    solution
}

pub fn compute_hash(s: &str) -> usize {
    let mut result: usize = 0;
    for c in s.as_bytes() {
        result += *c as usize;
        result *= 17;
        result %= 256;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_compute_hash() {
        assert_eq!(52, compute_hash("HASH"));
        assert_eq!(30, compute_hash("rn=1"));
        assert_eq!(253, compute_hash("cm-"));
        assert_eq!(0, compute_hash("rn"));
        assert_eq!(1, compute_hash("qp"));
        assert_eq!(3, compute_hash("pc"));
    }

    #[test]
    fn test_part1() {
        assert_eq!(1320, part1(Input::from_str(INPUT).unwrap()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(145, part2(Input::from_str(INPUT).unwrap()));
    }
}
