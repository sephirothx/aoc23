use std::fs;
mod days;
use days::day2;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    {
        let input = fs::read_to_string("./input/2.txt").unwrap();
        println!("{}", day2::part2(input));
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
