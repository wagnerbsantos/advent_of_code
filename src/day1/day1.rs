use std::str::FromStr;
use itertools::Itertools;
use crate::common::file::get_str_from_file;
use crate::common::string::get_number;

const NUMBER_SIZE: usize = 5;
const LINE_SIZE: usize = 2*NUMBER_SIZE + 3 + 1;

pub fn part_1()-> u32 {
    let text:String = get_str_from_file(1);
    let chars:Vec<char> = text.chars().collect();
    let height:usize = (chars.len() +1) / LINE_SIZE;
    let mut numbers_left: Vec<i32> = vec![0;height];
    let mut numbers_right: Vec<i32> = vec![0;height];
    for i in 0..height {
        numbers_left[i] = get_number(&chars,i* LINE_SIZE, i* LINE_SIZE+NUMBER_SIZE) as i32;
        numbers_right[i] = get_number(&chars,i* LINE_SIZE+NUMBER_SIZE+3, i* LINE_SIZE+NUMBER_SIZE+3+NUMBER_SIZE) as i32;
    }
    let mut left_iter = numbers_left.iter().sorted();
    let mut right_iter = numbers_right.iter().sorted();
    let mut sum:i32 = 0;
    for i in 0..height {
        let left = left_iter.next().unwrap();
        let right =  right_iter.next().unwrap();
        let numb = (left-right).abs();
        sum += numb;
    }
    println!("{}", sum);
    return sum as u32;
}

pub fn part_2()-> u32 {
    let text:String = get_str_from_file(1);
    let chars:Vec<char> = text.chars().collect();
    let height:usize = (chars.len() +1) / LINE_SIZE;
    let mut numbers_left: Vec<i32> = vec![0;height];
    let mut numbers_right: Vec<i32> = vec![0;height];
    for i in 0..height {
        numbers_left[i] = get_number(&chars,i* LINE_SIZE, i* LINE_SIZE+NUMBER_SIZE) as i32;
        numbers_right[i] = get_number(&chars,i* LINE_SIZE+NUMBER_SIZE+3, i* LINE_SIZE+NUMBER_SIZE+3+NUMBER_SIZE) as i32;
    }
    let mut sum:usize = 0;
    for i in 0..height {
        let left = numbers_left[i];
        let times_on_right = numbers_right.iter().filter(|x| x == &&left).count();
        sum += left as usize * times_on_right;
    }
    println!("{}", sum);
    return sum as u32;
}

