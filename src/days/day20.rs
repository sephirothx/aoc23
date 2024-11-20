use std::{collections::{HashMap, VecDeque}, convert::Infallible, str::FromStr};

use crate::math;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Module {
    t: ModuleType,
    name: String,
    next: Vec<String>,
    on: bool,
    previous: HashMap<String, bool>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    modules: HashMap<String, Module>,
}

impl Module {
    fn process_pulse(&mut self, from: String, value: bool) -> Option<bool> {
        match self.t {
            ModuleType::Broadcaster => Some(value),
            ModuleType::FlipFlop if value => None,
            ModuleType::FlipFlop => {
                self.on = !self.on;
                Some(self.on)
            },
            ModuleType::Conjunction => {
                self.previous.entry(from).and_modify(|v| *v = value);
                Some(!self.previous.values().all(|v| *v))
            }
        }
    }
}

impl Input {
    fn press_button(&mut self, monitor: &str) -> (usize, usize, bool) {
        let mut q = VecDeque::new();
        let mut lo = 0;
        let mut hi = 0;
        let mut triggered = false;
        q.push_back((String::new(), String::from("broadcaster"), false));
        while let Some((from, to, pulse)) = q.pop_front() {
            if pulse { hi += 1; } else { lo += 1; }
            if let Some(module) = self.modules.get_mut(&to) {
                if pulse && to == monitor {
                    triggered = true;
                }
                if let Some(next_pulse) = module.process_pulse(from, pulse) {
                    q.extend(module.next.iter().map(|nm| (to.clone(), nm.clone(), next_pulse)));
                }
            }
        }
        (lo, hi, triggered)
    }
}

impl FromStr for Module {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (module, next) = s.split_once(" -> ").unwrap();
        let (t, name) = match module.as_bytes() {
            [b'%', rest @ ..] => (ModuleType::FlipFlop, String::from_utf8_lossy(rest).into_owned()),
            [b'&', rest @ ..] => (ModuleType::Conjunction, String::from_utf8_lossy(rest).into_owned()),
            _ => (ModuleType::Broadcaster, String::from("broadcaster")),
        };
        let next = next
            .split(", ")
            .map(|n| String::from(n))
            .collect();
        Ok(Module { t, name, next, on: false, previous: HashMap::new() })
    }
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut modules: HashMap<_,_> = s.lines()
            .map(|line| Module::from_str(line).unwrap())
            .map(|m| (m.name.clone(), m))
            .collect();
        for m in modules.clone().values() {
            for n in &m.next {
                if let Some(nm) = modules.get_mut(n) {
                    if nm.t == ModuleType::Conjunction {
                        nm.previous.insert(m.name.clone(), false);
                    }
                }
            }
        }
        Ok(Input { modules })
    }
}

pub fn part1(input: Input) -> usize {
    let mut input = input;
    let (mut lo, mut hi) = (0, 0);
    for _ in 0..1000 {
        let out = input.press_button("");
        lo += out.0;
        hi += out.1;
    }
    lo * hi
}

pub fn part2(input: Input) -> i64 {
    let mut input = input;
    let mut periods = Vec::new();
    let mut count = 0;
    while periods.len() < 4 {
        count += 1;
        let (.., triggered) = input.press_button("hf");
        if triggered {
            periods.push(count);
        }
    }
    math::lcm_vec(periods)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const INPUT2: &str = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn test_part1() {
        assert_eq!(32000000, part1(INPUT1.parse().unwrap()));
        assert_eq!(11687500, part1(INPUT2.parse().unwrap()));
    }
}
