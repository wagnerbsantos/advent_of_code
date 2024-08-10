use std::{fs::File, io::Read};

pub fn get_str_from_file(day: u32) -> String {
    let path = format!(
        "/home/wagner/projects/advent_of_code/src/day{}/input.txt",
        day
    );
    println!("{}", path);
    let file = File::open(path);
    let mut text = String::new();
    match file {
        Ok(mut file) => {
            let result = file.read_to_string(&mut text);
            match result {
                Ok(size) => println!("Read {} bytes", size),
                Err(e) => println!("{:?}", e),
            }
        }
        Err(e) => {
            println!("error {:?}", e)
        }
    }
    text
}
