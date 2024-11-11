use std::{collections::HashMap, convert::Infallible, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
struct RuleSet {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Debug, PartialEq, Eq)]
enum Rule {
    LessThan(usize, i32, String),
    GreaterThan(usize, i32, String),
    Goto(String),
}

#[derive(Debug, PartialEq, Eq)]
struct Part {
    ratings: [i32; 4],
}

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    rulesets: HashMap<String, RuleSet>,
    parts: Vec<Part>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct RatingsRange {
    upper: [i32; 4],
    lower: [i32; 4],
}

impl RatingsRange {
    fn count_combinations(&self) -> i64 {
        self.upper
            .into_iter()
            .zip(self.lower)
            .fold(1, |prev, pair| prev * ((pair.0 as i64) - (pair.1 as i64) + 1))
    }
}

impl FromStr for Part {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s
            .trim_matches(['{', '}'])
            .split(',')
            .map(|rating| rating.split_once('=').unwrap().1.parse::<i32>().unwrap());
        let ratings = std::array::from_fn(|_| iter.next().unwrap());
        Ok(Part { ratings })
    }
}

impl FromStr for Rule {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rule = match s.chars().nth(1) {
            Some(x) if x == '<' || x == '>' => {
                let i = s.as_bytes()[0];
                let i = if i == b'a' { 2 } else { i % 4 } as usize;
                let (n, to) = s[2..].split_once(':').unwrap();
                let n = n.parse().unwrap();
                if x == '<' {
                    Rule::LessThan(i, n, to.to_owned())
                } else {
                    Rule::GreaterThan(i, n, to.to_owned())
                }
            }
            _ => Rule::Goto(s.to_owned()),
        };
        Ok(rule)
    }
}

impl FromStr for RuleSet {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rest) = s.split_once('{').unwrap();
        let rules = rest
            .trim_end_matches('}')
            .split(',')
            .map(|rule| Rule::from_str(rule).unwrap())
            .collect();
        Ok(RuleSet { name: name.to_owned(), rules})
    }
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rulesets, parts) = s.split_once("\n\n").unwrap();
        let rulesets = rulesets
            .lines()
            .map(|rs| {
                let ruleset = RuleSet::from_str(rs).unwrap();
                (ruleset.name.clone(), ruleset)
            })
            .collect();
        let parts = parts
            .lines()
            .map(|part| Part::from_str(part).unwrap())
            .collect();
        Ok(Input { rulesets, parts })
    }
}

pub fn part1(input: Input) -> i32 {
    let mut sol = 0;
    for part in input.parts {
        let mut current = String::from("in");
        while current != "A" && current != "R" {
            current = process_ruleset(&part, &input.rulesets[&current]);
        }
        if current == "A" {
            sol += part.ratings.iter().sum::<i32>();
        }
    }
    sol
}

pub fn part2(input: Input) -> i64 {
    let mut ranges = Vec::new();
    let starting_range = RatingsRange {
        upper: [4000, 4000, 4000, 4000],
        lower: [1, 1, 1, 1],
    };
    find_accepted_ranges(&input.rulesets, String::from("in"), starting_range, &mut ranges);
    ranges.into_iter().fold(0, |prev, rr| prev + rr.count_combinations())
}

fn process_ruleset(part: &Part, ruleset: &RuleSet) -> String {
    for rule in &ruleset.rules {
        match rule {
            Rule::Goto(to) => return to.clone(),
            Rule::LessThan(i, n, to) if part.ratings[*i] < *n => return to.clone(),
            Rule::GreaterThan(i, n, to) if part.ratings[*i] > *n => return to.clone(),
            _ => continue
        }
    }
    unreachable!()
}

fn find_accepted_ranges(
    rulesets: &HashMap<String, RuleSet>,
    current_ruleset: String,
    current_range: RatingsRange,
    ranges: &mut Vec<RatingsRange>
) {
    if current_ruleset == "A" {
        ranges.push(current_range);
    } else if current_ruleset != "R" {
        let mut current_range = current_range;
        let Some(ruleset) = rulesets.get(&current_ruleset) else { unreachable!() };
        for rule in &ruleset.rules {
            match rule {
                Rule::Goto(to) => find_accepted_ranges(rulesets, to.to_owned(), current_range, ranges),

                Rule::LessThan(i, n, _) if current_range.lower[*i] >= *n => (),
                Rule::GreaterThan(i, n, _) if current_range.upper[*i] <= *n => (),

                Rule::LessThan(i, n, to) if current_range.upper[*i] < *n
                    => find_accepted_ranges(rulesets, to.to_owned(), current_range, ranges),
                Rule::GreaterThan(i, n, to) if current_range.lower[*i] > *n
                    => find_accepted_ranges(rulesets, to.to_owned(), current_range, ranges),

                Rule::LessThan(i, n, to) => {
                    let mut next_range = current_range;
                    next_range.upper[*i] = *n - 1;
                    find_accepted_ranges(rulesets, to.to_owned(), next_range, ranges);
                    current_range.lower[*i] = *n;
                },

                Rule::GreaterThan(i, n, to) => {
                    let mut next_range = current_range;
                    next_range.lower[*i] = *n + 1;
                    find_accepted_ranges(rulesets, to.to_owned(), next_range, ranges);
                    current_range.upper[*i] = *n;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn rule_from_str() {
        assert_eq!(Rule::LessThan(2, 2006, String::from("qkq")), Rule::from_str("a<2006:qkq").unwrap());
        assert_eq!(Rule::GreaterThan(1, 2090, String::from("A")), Rule::from_str("m>2090:A").unwrap());
        assert_eq!(Rule::Goto(String::from("rfg")), Rule::from_str("rfg").unwrap());
    }

    #[test]
    fn part_from_str() {
        assert_eq!(Part { ratings: [787, 2655, 1222, 2876] }, Part::from_str("{x=787,m=2655,a=1222,s=2876}").unwrap());
    }

    #[test]
    fn test_process_ruleset() {
        let ruleset = RuleSet::from_str("in{s<1351:px,qqz}").unwrap();
        let part1 = Part::from_str("{x=787,m=2655,a=1222,s=2876}").unwrap();
        let part2 = Part::from_str("{x=1679,m=44,a=2067,s=496}").unwrap();
        assert_eq!(String::from("qqz"), process_ruleset(&part1, &ruleset));
        assert_eq!(String::from("px"), process_ruleset(&part2, &ruleset));
    }

    #[test]
    fn rr_count_combinations() {
        let range1 = RatingsRange {
            upper: [4000, 4000, 4000, 4000],
            lower: [1, 1, 1, 1],
        };
        let range2 = RatingsRange {
            upper: [1, 1, 1, 1],
            lower: [1, 1, 1, 1],
        };
        assert_eq!(i64::pow(4000, 4), range1.count_combinations());
        assert_eq!(1, range2.count_combinations());
    }

    #[test]
    fn test_part1() {
        assert_eq!(19114, part1(Input::from_str(INPUT).unwrap()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(167409079868000, part2(Input::from_str(INPUT).unwrap()));
    }
}
