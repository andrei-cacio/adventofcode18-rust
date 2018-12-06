use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
	println!("--== Advent of Code 2018 ==--"); 

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let v: Vec<&str> = contents.split_terminator('\n').collect();
    let res: i32 = v.iter().fold(0, |acc, x| acc + x.parse::<i32>().unwrap());

    println!("\nDay1 result: {:?}", res); 
}
