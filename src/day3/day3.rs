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

pub fn find_number(chars: &[char], anchor: usize, width: usize) -> (u32, usize) {
    let mut start = anchor;
    let mut end = anchor;
    let mut sum = 0;
    while start as i32-1 != -1 && chars[start-1].is_digit(10) {
        start -= 1;
    }
    while ((end+1) % width) > end % width && chars[end+1].is_digit(10) {
        end += 1;
    }
    for pos in start..end+1 {
        let multi = end - pos;
        sum += chars[pos].to_digit(10).unwrap() * (10_u32).pow(multi as u32);
    }
    (sum, end)
}

pub fn is_digit(char: char) -> bool {
    (char as u32) < 10
}

pub fn part_2() -> u32 {
    let text = get_str_from_file(3);
    let mut sum = 0;
    let chars: Vec<char> = text.chars().filter(|c| !c.is_whitespace()).collect();
    let width = 140;
    // let width = 10;

    for pos in 0..chars.len() {
        let line = pos / width;
        if chars[pos] == '*' {
            let mut number_index = 0;
            let mut number_array = [0;2];
            let mut looking_line:i32 = -1;
            let mut next:i32 = -1;
            if line == 0 {
                looking_line = 0;
            }
            while looking_line <2 {
                next = -1;
                if pos % width == 0 {
                    next = 0;
                }
                while next < 2 {
                    let looking = pos as i32 + width as i32* looking_line + next;
                    let char = chars[looking as usize];
                    if (looking > 0 && char.is_digit(10)) {
                        let (number, end) = find_number(&chars, looking as usize, width);
                        next = next + 1 + (end as i32 - looking);
                        if number_index <=1 {
                            number_array[number_index] = number;
                        }
                        number_index += 1;
                    } else {
                        next +=1;
                    }
                }
                looking_line += 1;
            }
            if number_index == 2 {
                sum += number_array[0] * number_array[1];
            }
        }
    }
    println!("Sum: {}", sum);
    sum
}