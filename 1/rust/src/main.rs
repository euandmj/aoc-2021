use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let vec = get_lines("../input.txt");

    let mut incr = 0;
    let mut decr = 0;
    let mut last_depth = vec.first().unwrap();

    println!("received {} lines", vec.len());
    for depth in &vec[1..] {
        if depth > last_depth {
            incr+=1;
        } else {
            decr+=1;
        }
        last_depth = depth;
    }
    
    println!("got: {} increments, {} decrements", incr, decr);
}

fn get_lines(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename)
        .expect("failed to read file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| -> i32 {
            l.unwrap().parse::<i32>().unwrap()
        })
        .collect()
}