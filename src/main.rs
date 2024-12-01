use std::collections::HashMap;
use itertools::Itertools;
use std::time::Instant;
use crate::common::file::get_str_from_file;

mod common;
mod day1;

fn main() {
    let before = Instant::now();
    let result  = day1::day1::part_2();
    let elapsed = before.elapsed();
    println!("{}", result );
    println!("Elapsed time: {:.2?}", elapsed );
}

