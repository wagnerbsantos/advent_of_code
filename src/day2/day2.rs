use crate::common::file::get_str_from_file;

pub fn part_1() -> u64 {
    let text: String = get_str_from_file(2);
    let lines: Vec<&str> = text.split(",").collect();
    let mut result = 0;

    for line in lines {
        let splitted: Vec<&str> = line.split("-").collect();
        println!("{:?}", line);
        let mut first = splitted[0].parse::<u64>().unwrap();
        let last = splitted[1].parse::<u64>().unwrap();
        while first <= last {
            let string = first.to_string();
            if string.len() % 2 != 0 {
                first += 1;
                continue;
            }
            let spl = string.split_at(string.len() / 2);
            if spl.0 == spl.1 {
                result += first;
            }

            first += 1;
        }
    }

    return result as u64;
}

pub fn part_2() -> u64 {
    let text: String = get_str_from_file(2);
    let lines: Vec<&str> = text.split(",").collect();
    let mut result = 0;

    for line in lines {
        let splitted: Vec<&str> = line.split("-").collect();
        let mut first = splitted[0].parse::<u64>().unwrap();
        let last = splitted[1].parse::<u64>().unwrap();
        while first <= last {
            let string = first.to_string();
            let mut size = 1;
            let mut is_equal = false;
            while size <= string.len() / 2 && !is_equal {
                is_equal = search_repeat(size, &string);
                if is_equal {
                    result += first;
                }
                size += 1;
            }
            first += 1;
        }
    }

    return result as u64;
}

fn search_repeat(size: usize, string: &String) -> bool {
    if string.len() % size != 0 {
        return false;
    }
    let compare = string.split_at(size);
    if !compare.1.ends_with(compare.0) {
        return false;
    }
    let splits: Vec<&str> = compare.1.split(compare.0).collect();
    let tamanho = splits.len();
    if tamanho == string.len() / size {
        return true;
    }

    return false;
}
