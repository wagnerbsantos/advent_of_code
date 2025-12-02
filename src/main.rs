use std::time::Instant;

mod common;
mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let before = Instant::now();
    let result = day4::day4::part_1();
    let elapsed = before.elapsed();
    println!("Part 1: {}", result);
    println!("Elapsed time: {:.2?}", elapsed);

    // let before = Instant::now();
    // let result = day3::day3::part_2();
    // let elapsed = before.elapsed();
    // println!("Part 2: {}", result);
    // println!("Elapsed time: {:.2?}", elapsed);
}
