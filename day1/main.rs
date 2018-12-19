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

    let v: Vec<&str> = contents.split_terminator('\n').collect();
    let nr_v: Vec<i32> = v.iter().map(|nr| nr.parse::<i32>().unwrap()).collect();
    let mut freqs_map = HashMap::new();

    let mut first_repeat_freq = 0;
    let mut first_freq = 0;
    let mut last_freq = 0;
    let mut iterations = 0;

    while first_repeat_freq == 0 {
        last_freq = nr_v.iter().fold(last_freq, |acc, x| {
            let freq = acc + x;
            
            if freqs_map.contains_key(&freq) && first_repeat_freq == 0 {
                first_repeat_freq = freq;
            } else {
                freqs_map.insert(freq, freq);    
            }

            return freq;
        });

        if first_freq == 0 {
            first_freq = last_freq;
        }

        iterations += 1;    
    }
    

    println!("\nDay1 1/2 result: {:?}", first_freq); 
    println!("Day1 2/2 result: {} [iterations: {}]", first_repeat_freq, iterations);
}
