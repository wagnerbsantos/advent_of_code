use std::time::Instant;

mod common;
mod day1;
mod day2;
mod day3;

fn main() {
    let before = Instant::now();
    let p1_result = day3::day3::part_1();
    let p1_elapsed = before.elapsed();

    let before = Instant::now();
    let p2_result = day3::day3::part_2();
    let p2_elapsed = before.elapsed();

    println!("Part 1: {}", p1_result);
    println!("Elapsed time: {:.2?}", p1_elapsed);
    println!("Part 2: {}", p2_result);
    println!("Elapsed time: {:.2?}", p2_elapsed);
}
