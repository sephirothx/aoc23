#![allow(dead_code)]

mod days;
mod input;
mod math;
use std::str::FromStr;

use days::day9::{Input, *};
use input::read_from_file;

fn main() {
    let input = Input::from_str(&read_from_file(9)).unwrap();
    use std::time::Instant;
    let now = Instant::now();
    {
        println!("{}", part2(input));
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
