use crate::common::file::get_str_from_file;
use crate::common::string::split_to_vec;

pub fn part_1() -> u32 {
    let text = get_str_from_file(2);
    let lines = split_to_vec(text, "\n");
    let mut sum = 0;
    for line in lines {
        let doispontos = split_to_vec(line, ": ");
        let game_text = doispontos.get(0).unwrap();
        let problem = doispontos.get(1).unwrap();
        let game_number: u32 = split_to_vec(game_text.clone(), " ")
            .get(1)
            .unwrap()
            .parse()
            .unwrap();
        let mut is_invalid = false;
        for situation in split_to_vec(problem.to_string(), "; ") {
            for ball in split_to_vec(situation.to_string(), ", ") {
                let number_color = split_to_vec(ball.to_string(), " ");
                let number: u32 = number_color.get(0).unwrap().parse().unwrap();
                let colorname = number_color.get(1).unwrap();
                if (colorname == "red" && number > 12)
                    || (colorname == "green" && number > 13)
                    || (colorname == "blue" && number > 14)
                {
                    is_invalid = true;
                }
            }
        }
        if !is_invalid {
            sum += game_number;
        }
    }
    println!("{}", sum);
    return 0;
}

pub fn part_2() -> u32 {
    let text = get_str_from_file(2);
    let lines = split_to_vec(text, "\n");
    let mut sum = 0;
    for line in lines {
        let doispontos = split_to_vec(line, ": ");
        let problem = doispontos.get(1).unwrap();
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;
        for situation in split_to_vec(problem.to_string(), "; ") {
            for ball in split_to_vec(situation.to_string(), ", ") {
                let number_color = split_to_vec(ball.to_string(), " ");
                let number: u32 = number_color.get(0).unwrap().parse().unwrap();
                let colorname = number_color.get(1).unwrap();
                if colorname == "red" && number > red {
                    red = number
                } else if colorname == "green" && number > green {
                    green = number
                } else if colorname == "blue" && number > blue {
                    blue = number
                }
            }
        }
        sum += red * green * blue;
    }
    println!("{}", sum);
    return 0;
}
