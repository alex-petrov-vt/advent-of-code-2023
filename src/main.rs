use advent_of_code_2023::{day1};
use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    println!("=== DAY 1 ===");
    let contents = fs::read_to_string("input/day1.txt").unwrap();
    println!("{}", day1::part1(contents.lines()));
    println!("{}", day1::part2(contents.lines()));

    let elapsed = start.elapsed().as_millis();
    println!("Total execution time is {} milliseconds", elapsed);
}
