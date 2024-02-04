use std::fs::File;
use std::io::Read;

pub fn day1() -> u32 {

   let file = File::open("C:\\Users\\T-GAMER\\advent_of_code\\src\\day1\\input.txt");
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
   println!("text");

   
   return add_thing(text);

}

fn add_thing(mut text: String) -> u32 {
   let mut first: Option<u32> = None;
   let mut last: Option<u32> = None;
   let mut sum = 0;
   while let Some(ch) = text.pop(){
      if ch.is_numeric() {
         if first == None {
            first = Some(ch.to_digit(10).unwrap());
            last = Some(ch.to_digit(10).unwrap())
         } else {
            last = Some(ch.to_digit(10).unwrap())
         }
      }
      if ch.is_whitespace() && last != None && first != None {
         let number = last.unwrap() * 10 + first.unwrap();
         sum = sum + number;
         last = None;
         first = None;
      }
   }
   let number = last.unwrap() * 10 + first.unwrap();
   sum = sum + number;
   return sum;
}
