use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
	println!("--== Advent of Code 2018 ==--"); 

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let db: Vec<&str> = contents.split_terminator('\n').collect();
    let mut letters = HashMap::new();

    db.iter().for_each(|id| {
        for ch in id.chars() {
            match letters.get(&ch) {
                Some(mut x) => letters.insert(ch, x+1),
                None => letters.insert(ch, 1),
            };
        }
    });

    println!("{:?}", letters);

    // println!("\nDay1 result: {:?}", res); 
}
