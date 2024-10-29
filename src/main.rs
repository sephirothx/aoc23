#![allow(dead_code)]

mod days;
mod input;
mod math;
mod geometry;

use std::str::FromStr;
use days::day21::{Input, *};
use input::read_from_file;

fn main() {
    let input = Input::from_str(&read_from_file(21)).unwrap();
    use std::time::Instant;
    let now = Instant::now();
    {
        println!("{}", part1(input, 26501365));
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
