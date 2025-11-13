use std::io::{Error, Read};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {

    let buf = get_file_reader("input.txt").unwrap();
    let mut inputs:Vec<Vec<usize>> = Vec::new();
    for dim_str in buf.lines() {
        let dims: Vec<usize> = dim_str
            .unwrap()
            .split('x')
            .map(|dim| dim.parse::<usize>().expect("Parse Error: {dim}"))
            .collect();
        assert!(dims.len() == 3);
        inputs.push(dims);
    }
    let mut line_counter = 0usize;
    let mut total = 0usize; 
    for dims in &inputs {
        line_counter+=1;
        let first_face = 2*dims[0]*dims[1];
        let second_face = 2*dims[1]*dims[2];
        let third_face = 2*dims[0]*dims[2];
        let mut min_face = first_face;
        if second_face < min_face {
            min_face = second_face;}
        if third_face < min_face {
            min_face = third_face;
        }
        min_face /= 2;
        let addend = first_face + second_face + third_face + min_face;
        total += addend;
        // println!("LN:{:5} FF:{:9}, SF:{:9}, TF:{:9}, MF:{:9}, AD:{:9}, TOT:{:9}",line_counter,first_face,second_face,third_face,min_face,addend,total);
    }
    println!("Wrapping Paper: {total}");
    let mut ribbon = 0;
    for dims in inputs {
        let mut dim_sort = dims.clone();
        dim_sort.sort();
        let smallest_perim = dim_sort[0] + dim_sort[0] + dim_sort[1] + dim_sort[1];
        let vol = dim_sort.into_iter().fold(1, |acc,x| acc*x);
        ribbon += smallest_perim + vol;
    }
    println!("Ribbon: {ribbon}");
}

fn read_stdin_into_vec() -> Result<Vec<String>, Error> {
    use std::io;
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    let v = buf
        .split_ascii_whitespace()
        .map(|s| String::from(s))
        .collect();
    return Ok(v);
}

fn get_file_reader(path_str:&str) -> Result<BufReader<File>, Error> {
    let file = File::open(path_str)?;
    Ok(BufReader::new(file))
}