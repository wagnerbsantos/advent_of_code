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
        if (chars[pos] as u32) < 10 {
            end = Some(pos);
            if start.is_none() {
                start = Some(pos);
            }
        } else if let Some(end_of_number) = end {
            if start.is_some() {
                let is_part_number =
                    calculate_adjacency(&chars, start.unwrap(), end_of_number, line, width);
                if is_part_number {
                    sum += get_number(&chars, start.unwrap(), end_of_number);
                }
            } else {
                println!("Error, number didnt end");
            }
            start = None;
            end = None;
        }
    }
    println!("Sum: {}", sum);
    sum
}

pub fn calculate_adjacency(
    chars: &[char],
    start: usize,
    end: usize,
    line: usize,
    width: usize,
) -> bool {
    let start_compare = max(0, start as i32 - 1) as usize;
    let offset: u32 = (end as u32).abs_diff(line as u32 * width as u32);
    let end_compare = line * width + min((offset as usize) + 1, width - 1);
    for pos in start_compare..end_compare + 1 {
        if pos > 0 && !is_digit(chars[pos]) && chars[pos] != '.' {
            return true;
        }
        if pos + width < chars.len() && !is_digit(chars[pos + width]) && chars[pos + width] != '.' {
            return true;
        }
        if (pos as i32 - width as i32) > 0 && !is_digit(chars[pos - width]) && chars[pos - width] != '.' {
            return true;
        }
    }

    false
}

pub fn get_number(chars: &[char], start: usize, end: usize) -> u32 {
    let mut sum = 0;
    for pos in start..end + 1 {
        let multi = end - pos;
        sum += chars[pos] as u32 * (10_u32).pow(multi as u32);
    }
    sum
}

pub fn is_digit(char: char) -> bool {
    (char as u32) < 10
}
