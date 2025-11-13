use std::ascii::EscapeDefault;
use std::io::{Error, Read};
use std::collections::HashSet;

pub fn day_3_driver() {
   let input = get_input_string("src/_2015/_3_input.txt").unwrap();
   assert!(input.len() > 0);
   let mut houses = HashSet::<House>::new();
   let mut santas_pos = Coord::new(0,0);
   let mut robos_pos = santas_pos.clone();
   let santas_instructions = input.bytes().step_by(2);
   let robos_instructions = input.bytes().skip(1).step_by(2);
   for (santa, robo) in santas_instructions.zip(robos_instructions) {
      houses.insert(House::new(santas_pos));
      houses.insert(House::new(robos_pos));
      santas_pos.update(&santa);
      robos_pos.update(&robo);   
   }
   houses.insert(House::new(santas_pos));
   houses.insert(House::new(robos_pos));
   if input.len() % 2 == 1 {
      santas_pos.update(&input.bytes().last().unwrap());
      houses.insert(House::new(santas_pos));
   }
   println!("NUM UNIQ HOUSES: {}",houses.len());
}

#[derive(Eq,Hash,PartialEq,Clone,Copy)]
struct Coord {
   x:isize,
   y:isize,
}

impl Coord {
   fn update(&mut self, ch:&u8) {
      match ch {
         b'^' => {self.y += 1},
         b'v' => {self.y -= 1},
         b'>' => {self.x += 1},
         b'<' => {self.x -= 1},
         _ => {panic!("BAD CHAR: {ch}")},
      }
   }

   fn new(x:isize,y:isize) -> Self {
      Coord {x,y}
   }
}

#[derive(Eq,Hash,PartialEq)]
struct House {
   address:Coord
}

impl House {
   fn new<'a> (addr:Coord) -> Self {
      House {address: addr}
   }
}

fn get_input_string(path:&str) -> Result<String, Error> {
   use std::fs;
   let mut f = fs::File::open(path)?;
   let mut buf = String::new();
   f.read_to_string(&mut buf)?;
   Ok(buf)
}