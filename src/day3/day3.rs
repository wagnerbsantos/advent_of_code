use crate::common::file::get_str_from_file;
pub fn part_2() -> u32 {
    let text: String = get_str_from_file(3);
    let mut donts = text.split("don't()");
    let valid1 = donts.next().unwrap();
    let mut sum = run_valid(valid1);
    while let Some(don) = donts.next() {
        let mut does = don.split("do()");
        does.next();
        while let Some(valid) = does.next() {
            sum += run_valid(valid)
        }
    }
    sum
}

pub fn part_1() -> u32 {
    let text: String = get_str_from_file(3);
    run_valid(&text)
}

fn run_valid(text: &str) -> u32 {
    let lines = text.split("mul(");
    let mut sum = 0;
    for report in lines {
        let mut level_string = report.split(')');
        let important = level_string.next().unwrap();
        let mut numbers = important.split(',');
        let n1 = numbers.next();
        let n2 = numbers.next();
        if n1.is_some_and(is_number)
            && n2.is_some_and(is_number)
            && !important.contains(' ')
            && numbers.next().is_none()
        {
            sum += n1.unwrap().parse::<u32>().unwrap() * n2.unwrap().parse::<u32>().unwrap();
        }
    }
    sum
}

fn is_number(x: &str) -> bool {
    return x.parse::<u32>().is_ok();
}
