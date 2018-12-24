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
    let mut counts2_map: HashMap<&str, i32> = HashMap::new();
    let mut counts3_map: HashMap<&str, i32> = HashMap::new();

    db.iter().for_each(|id| {
        let mut letters: HashMap<char, i32> = HashMap::new();
        for ch in id.chars() {
            let mut new_freq;
            match letters.get(&ch) {
                Some(mut old_freq) => new_freq = old_freq + 1,
                None => new_freq = 1,
            };
            letters.insert(ch, new_freq);
        }
        

        for key in letters.keys() {
            let freq: i32 = *letters.get(&key).unwrap();
            if freq == 2 {
                counts2_map.insert(id, 2);
            }
            if freq == 3 {
                counts3_map.insert(id, 3);
            }
        }
    });

    println!("\nDay2.1 result: {:?}", counts2_map.keys().count() * counts3_map.keys().count()); 
}
