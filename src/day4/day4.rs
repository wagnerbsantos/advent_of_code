use crate::common::file::get_str_from_file;
const ROW_SIZE: usize = 140;

pub fn part_1() -> u32 {
    let text: String = get_str_from_file(4);
    let lines: Vec<char> = text.chars().filter(|c| !c.is_whitespace()).collect();
    let rows = lines.len() / ROW_SIZE;
    let mut sum = 0;

    for row in 0..rows {
        for column in 0..ROW_SIZE {
            if lines[row * ROW_SIZE + column] == 'X' {
                sum += check_xmas(&lines, row, column)
            }
        }
    }
    sum
}

fn check_xmas(text: &Vec<char>, row: usize, column: usize) -> u32 {
    let mut sum = 0;

    sum += check_direction(text, row, column, (0, -1), 'M');
    sum += check_direction(text, row, column, (-1, -1), 'M');
    sum += check_direction(text, row, column, (-1, 0), 'M');
    sum += check_direction(text, row, column, (-1, 1), 'M');
    sum += check_direction(text, row, column, (0, 1), 'M');
    sum += check_direction(text, row, column, (1, -1), 'M');
    sum += check_direction(text, row, column, (1, 1), 'M');
    sum += check_direction(text, row, column, (1, 0), 'M');
    println!("{:?}, {:?}, {:?}, ", row, column, sum);
    sum
}

fn check_direction(
    text: &Vec<char>,
    row: usize,
    column: usize,
    direction: (i32, i32),
    next_char: char,
) -> u32 {
    let cur_row = row as i32 + direction.0;
    let cur_column = column as i32 + direction.1;
    let position = cur_row * ROW_SIZE as i32 + cur_column;
    let number_of_rows = text.len() as i32 / ROW_SIZE as i32;
    if cur_row < 0 || cur_column < 0 || cur_column >= ROW_SIZE as i32 || cur_row >= number_of_rows {
        return 0;
    }
    if text[position as usize] == next_char {
        let next = get_next(next_char);
        if next == ' ' {
            return 1;
        }
        return check_direction(text, cur_row as usize, cur_column as usize, direction, next);
    }
    return 0;
}

fn get_next(ch: char) -> char {
    return match ch {
        'X' => 'M',
        'M' => 'A',
        'A' => 'S',
        _ => ' ',
    };
}
