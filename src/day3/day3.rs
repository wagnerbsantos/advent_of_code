use std::{fs::File, io::Read, num::NonZeroI128};

pub fn get_str_from_file(path: &str) -> String{
    let file = File::open(path);
    let mut text = String::new();
    match file {
       Ok(mut file) => {
          let result = file.read_to_string(&mut text);
          match result {
             Ok(size) => println!("Read {} bytes", size),
             Err(e) => println!("{:?}", e)
          }
       },
       Err(e) => {println!("error {:?}", e)},
    }
    return text
 }

pub fn part_1 () -> u32{
    let text = get_str_from_file("/home/kazye/projects/advent_of_code/src/day3/input.txt");

    let mut linessplit = text.split(char::is_whitespace);
    let mut lines = linessplit.clone().enumerate();

    while let Some((pos, line)) = lines.next(){
        let chars = line.chars();
        let mut en = chars.enumerate();
        let mut start: Option<usize> = None;
        let mut end: Option<usize> = None;

        while let Some((charpos, ch)) = en.next(){
            println!("{}, {}, {}", pos, charpos, ch);
            if ch.is_numeric() {
                end = Some(charpos+ 1);
                if start == None{
                    start = Some(charpos);
                }
            } else {
                if end != None {
                    let nextline = text.get((pos+ 1)* (line.len()+1)..((pos+ 2)* (line.len()+1)-1));

                    println!("{} {:?}", pos, nextline);
                    match nextline{
                        Some(l) => {
                            println!("{}", l);
                        }
                        None => {}
                    }
                    let number = 0;
                    let strnumber = line.split_at(start.unwrap()).1.split_at(end.unwrap()).0;
                    println!("{} {:?} {:?}", strnumber, start, end);
                    start = None;
                    end = None;
                }
            }

        }
    }

    return 0
}