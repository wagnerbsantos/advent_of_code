use crate::common::file::get_str_from_file;
pub fn part_1() -> u32 {
    let text: String = get_str_from_file(2);
    let lines = text.lines();

    let mut sum = 0;
    for report in lines {
        let mut level_string = report.split(' ');
        let mut last_value: i32 = level_string.next().unwrap().parse().unwrap();
        let mut multiplier = 0;
        let mut level = 0;
        while let Some(current) = level_string.next() {
            let current_value: i32 = current.parse().unwrap();
            let new_multiplier: i32 = last_value - current_value;
            if (new_multiplier < 0 && multiplier > 0)
                || (new_multiplier > 0 && multiplier < 0)
                || new_multiplier.abs() > 3
                || new_multiplier == 0
            {
                level = 0;
                break;
            }
            if level == 0 {
                multiplier = last_value - current_value
            }
            level += 1;
            last_value = current_value;
        }
        if level != 0 {
            sum += 1;
        }
    }
    sum
}

pub fn part_2() -> u32 {
    let text: String = get_str_from_file(2);
    let lines = text.lines();

    let mut sum = 0;
    for report in lines {
        let level = fun_name(report, true, None);
        if level != 0 {
            sum += 1;
        }
    }
    sum
}

fn fun_name(report: &str, should_try: bool, skip_index: Option<usize>) -> i32 {
    let mut level_string = report.split(' ');
    if let Some(i) = skip_index {
        if i == 0 {
            level_string.next();
        }
    }
    let mut last_value: i32 = level_string.next().unwrap().parse().unwrap();
    let mut multiplier = 0;
    let mut level = 0;
    let mut index = 0;
    while let Some(mut current) = level_string.next() {
        if let Some(i) = skip_index {
            if i == index + 1 {
                if let Some(str) = level_string.next() {
                    current = str;
                } else {
                    println!("{}, {}, {}", report, index, current);
                    return level;
                }
                index += 1;
            }
        }
        let current_value: i32 = current.parse().unwrap();
        let new_multiplier: i32 = last_value - current_value;
        if (new_multiplier < 0 && multiplier > 0)
            || (new_multiplier > 0 && multiplier < 0)
            || new_multiplier.abs() > 3
            || new_multiplier == 0
        {
            if should_try {
                level = fun_name(report, !should_try, Some(index));
                if level == 0 {
                    level = fun_name(report, !should_try, Some(index + 1))
                }
                if level == 0 && index != 0 {
                    level = fun_name(report, !should_try, Some(index - 1));

                    if level != 0 {
                        println!("{}, {} - {}, {}", report, index, index - 1, level);
                    }
                }
                if level != 0 {
                    // println!("{}, {} + {}, {}", report, index, index + 1, level);
                    return level;
                }
            }
            level = 0;
            break;
        }
        if level == 0 {
            multiplier = last_value - current_value
        }
        level += 1;
        index += 1;
        last_value = current_value;
    }
    level
}
