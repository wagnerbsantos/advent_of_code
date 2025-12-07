use crate::common::file::get_str_from_file;

pub fn part_1() -> u32 {
    let text: String = get_str_from_file(1);
    let lines: Vec<&str> = text.split("\n").collect();
    let mut dial = 50;
    let mut result = 0;

    for line in lines {
        let splitted = line.split_at(1);
        let letter = splitted.0;
        let number = splitted.1.parse::<i32>().unwrap();
        if letter == "L" {
            dial = (dial - number) % 100
        } else {
            dial = (dial + number) % 100
        }
        if dial == 0 {
            result += 1;
        }
    }

    return result;
}

pub fn part_2() -> u32 {
    let text: String = get_str_from_file(1);
    let lines: Vec<&str> = text.split("\n").collect();
    let mut dial = 50;
    let mut result = 0;

    for line in lines {
        let splitted = line.split_at(1);
        let letter = splitted.0;
        let mut number = splitted.1.parse::<i32>().unwrap();
        result += number / 100;
        number = number % 100;

        let mut temp_dial: i32 = if letter == "L" {
            number = -number;
            dial + number
        } else {
            dial + number
        };

        if (temp_dial < 0 || temp_dial > 100) && dial != 0 {
            result += 1;
        }

        println!("{} {} {} {}", number, dial, temp_dial, temp_dial.abs());
        if temp_dial < 0 {
            temp_dial = 100 + temp_dial;
        }
        dial = temp_dial.abs() % 100;
        if dial == 0 {
            result += 1;
        }
    }

    return result as u32;
}
