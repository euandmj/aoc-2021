use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let vec = get_lines("../input.txt");

    println!("received {} lines", vec.len());

    // let result = part1(&vec);
    let result = part2(&vec);

    println!("got: {} increments.", result);
}

#[allow(dead_code)]
fn part1(vec: &Vec<i32>) -> i32 {
    let mut incr: i32 = 0;
    let mut _decr: i32 = 0;
    let mut last_depth = vec.first().unwrap();

    for depth in &vec[1..] {
        if depth > last_depth {
            incr += 1;
        } else {
            _decr += 1;
        }
        last_depth = depth;
    }
    incr
}

#[allow(dead_code)]
fn part2(vec: &Vec<i32>) -> i32 {
    let mut incr: i32 = 0;
    let mut _decr: i32 = 0;
    let mut last_sum = i32::MAX;

    for (i, _) in vec.iter().enumerate() {
        if i < 2 {
            continue;
        }
        let sum = vec[i] + vec[i - 1] + vec[i - 2];

        if sum > last_sum {
            incr += 1;
        }
        last_sum = sum;
    }
    incr
}

fn get_lines(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("failed to read file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| -> i32 { l.unwrap().parse::<i32>().unwrap() })
        .collect()
}
