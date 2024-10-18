#![allow(dead_code)]

mod days;
mod input;
use days::day6::{self, INPUT2};

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    {
        println!("{}", day6::part1_2(INPUT2));
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
