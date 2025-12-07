use crate::common::file::get_str_from_file;

pub fn part_1() -> u32 {
    let text: String = get_str_from_file(3);
    let lines: Vec<&str> = text.split("\n").collect();
    let mut result = 0;

    for line in lines {
        let mut esquerda = 0;
        let mut direita = 0;
        for (index, letra) in line.chars().enumerate() {
            let next = letra.to_digit(10).unwrap();
            if next > esquerda && index < line.len() - 1 {
                esquerda = next;
                direita = 0;
            } else if next > direita {
                direita = next;
            }
        }
        println!("{}, {}", esquerda, direita);
        result += esquerda * 10 + direita;
    }

    return result;
}

pub fn part_2() -> u64 {
    let text: String = get_str_from_file(3);
    let lines: Vec<&str> = text.split("\n").collect();
    let mut result: u64 = 0;

    for line in lines {
        let mut jolts: [u64; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        for (index, letra) in line.chars().enumerate() {
            let next: u64 = letra.to_digit(10).unwrap().into();
            for i in 0..jolts.len() {
                if next > jolts[i] && index <= line.len() - jolts.len() + i {
                    jolts[i] = next;
                    for j in i + 1..jolts.len() {
                        jolts[j] = 0
                    }
                    break;
                }
            }
        }
        println!("{:?}", jolts);
        for i in 0..jolts.len() {
            result += jolts[i] * (10 as u64).pow(11 - i as u32)
        }
    }

    return result;
}
