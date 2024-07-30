use std::env;
use std::fs::File;
use std::io::Read;
mod _2015;

fn main() {
    let args: Vec<String> = env::args().collect();
    let src = std::fs::read_to_string(&args[1]).unwrap();
    _2015::day_5::naughty_strings(src.split_whitespace().collect());
}
