#![allow(dead_code)]

mod days;
mod input;
use days::day6;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    {
        println!("{}", day6::part1_2(&[(56717999, 334113513502430)]));
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
