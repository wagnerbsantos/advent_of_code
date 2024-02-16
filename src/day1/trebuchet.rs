use std::cmp::min;
use std::fs::File;
use std::io::Read;

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

pub fn day1() -> u32 {

   let mut text = get_str_from_file("/home/kazye/projects/advent_of_code/src/day1/input.txt");
   
   return add_thing(text);

}

fn add_thing(text: String) -> u32 {
   let mut first: Option<u32> = None;
   let mut last: Option<u32> = None;
   let mut sum = 0;
   let mut chars = text.chars().enumerate();
   while let Some((pos, ch)) = chars.next(){

      let forward = text.split_at(pos).1;
      let word = forward.split_at(std::cmp::min(forward.len(), 5)).0;
      let mut txtnumber = try_word(&word);
      if ch.is_numeric() {
         txtnumber = Some(ch.to_digit(10).unwrap());
      }
      match txtnumber {
         Some(x) => {
            if first == None {
               first = Some(x);
               last = Some(x)
            } else {
               last = Some(x)
            }
         }
         None => {}
      }
      if txtnumber != None{

      }
      if ch.is_whitespace() && last != None && first != None {
         let number = first.unwrap() * 10 + last.unwrap();
         sum = sum + number;
         println!("{:?} {:?}", first, number);
         last = None;
         first = None;
      }
   }
   let number = first.unwrap() * 10 + last.unwrap();
   println!("{}", number);
   sum = sum + number;
   return sum;
}

fn try_word(word: &str) -> Option<u32>{
   let mut pos = word.len();
   if let Some(z) = word.find(char::is_whitespace) {
      pos = z;
   }
   let x = word.split_at(pos).0;
   let size3 = word.split_at(min(x.len(), 3)).0;
   let size4 = word.split_at(min(x.len(), 4)).0;
   let size5 = word.split_at(min(x.len(), 5)).0;
   if size4.contains("zero") {
      return Some(0)
   }
   if size3.contains("one") {
      return Some(1)
   }
   if size3.contains("two") {
      return Some(2)
   }
   if size5.contains("three") {
      return Some(3)
   }
   if size4.contains("four") {
      return Some(4)
   }
   if size4.contains("five") {
      return Some(5)
   }
   if size3.contains("six") {
      return Some(6)
   }
   if size5.contains("seven") {
      return Some(7)
   }
   if size5.contains("eight") {
      return Some(8)
   }
   if size4.contains("nine") {
      return Some(9)
   }
   
   return None;
}