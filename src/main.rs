use std::env;

mod _2015;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Open input file specified by command line arg and split on newline
    assert!(args.len()==3);
    let src = std::fs::read_to_string(&args[1]).unwrap();
    let src2: Vec<&str> = src.split("\n").collect();
    _2015::day_07::driver(src2,&args[2]);
}
