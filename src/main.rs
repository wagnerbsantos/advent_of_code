use std::collections::HashMap;
use itertools::Itertools;
use std::time::Instant;
use crate::common::file::get_str_from_file;

mod common;
mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    // println!("{}", day1::trebuchet::day1());
    // println!("{}", day3::day3::part_1());
    // let before = Instant::now();
    // println!("Elapsed time: {:.2?}", before.elapsed());
    let before = Instant::now();
    let result  = day1::part1::part_1();
    let elapsed = before.elapsed();
    println!("{}", result );
    println!("Elapsed time: {:.2?}", elapsed );
    // println!("{}", day2::day2::part_1());
    // println!("{}", day2::day2::part_2());
}

