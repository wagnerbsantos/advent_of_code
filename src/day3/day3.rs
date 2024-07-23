use std::cmp::{max, min};

use crate::common::file::get_str_from_file;

pub fn part_1() -> u32 {
    let text = get_str_from_file(3);
    let mut sum = 0;
    let chars: Vec<char> = text.chars().filter(|c| !c.is_whitespace()).collect();
    let width = 140;

    let mut start: Option<usize> = None;
    let mut end: Option<usize> = None;

    for pos in 0..chars.len() {
        let line = pos / width;
        let ch = chars[pos];
        // println!("{}, {}, {}, {}", line, column, pos, ch);
        if ch.is_numeric() {
            end = Some(pos);
            if start == None {
                start = Some(pos);
            }
        } else {
            let is_end_of_number = end != None;
            if is_end_of_number {
                if start.is_some() && end.is_some() {
                    let is_part_number =
                        calculate_adjacency(&chars, start.unwrap(), end.unwrap(), line, width);
                    if is_part_number {
                        sum += get_number(&chars, start.unwrap(), end.unwrap());
                        println!(
                            "Found number: {}",
                            get_number(&chars, start.unwrap(), end.unwrap())
                        )
                    }
                } else {
                    println!("Error, number didnt end");
                }
                start = None;
                end = None;
            }
        }
    }
    println!("Sum: {}", sum);
    return sum;
}

pub fn calculate_adjacency(
    chars: &Vec<char>,
    start: usize,
    end: usize,
    line: usize,
    width: usize,
) -> bool {
    let start_compare = max(0, start as i32 - 1) as usize;
    let offset: u32 = (end as u32).abs_diff(line as u32 * width as u32);
    let end_compare = line * width + min((offset as usize) + 1, width - 1);
    for pos in start_compare..end_compare + 1 {
        if pos > 0 {
            if !chars[pos].is_digit(10) && chars[pos] != '.' {
                return true;
            }
        }
        if pos + width < chars.len() {
            if !chars[pos + width].is_digit(10) && chars[pos + width] != '.' {
                return true;
            }
        }
        if (pos as i32 - width as i32) as i32 > 0 {
            if !chars[pos - width].is_digit(10) && chars[pos - width] != '.' {
                return true;
            }
        }
    }

    return false;
}

pub fn get_number(chars: &Vec<char>, start: usize, end: usize) -> u32 {
    let mut sum = 0;
    for pos in start..end + 1 {
        let multi = end - pos;
        if let Some(digit) = chars[pos].to_digit(10) {
            sum += digit * (10 as u32).pow(multi as u32);
        }
    }
    return sum;
}
