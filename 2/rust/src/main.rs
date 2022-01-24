use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = get_lines("../input.txt");

    // let result = part1(&lines);
    let result = part2(&lines);

    println!("output: {}", result);
}

#[allow(dead_code)]
fn part1(vec: &Vec<String>) -> i32 {
    let mut h = 0;
    let mut d = 0;

    for l in vec {
        let (dir, value) = parse_line(l);

        match dir.as_ref() {
            "forward" => h += value,
            "up" => d -= value,
            "down" => d += value,
            _ => println!("unexpected match on '{}'", dir),
        }
    }

    h * d
}

#[allow(dead_code)]
fn part2(vec: &Vec<String>) -> i32 {
    let mut h = 0;
    let mut d = 0;
    let mut aim = 0;

    for l in vec {
        let (dir, value) = parse_line(l);

        match dir.as_ref() {
            "forward" => {
                h += value;
                d += aim * value
            },
            "up" => aim -= value,
            "down" => aim += value,
            _ => println!("unexpected match on '{}'", dir),
        }
    }

    h * d
}

fn parse_line(line: &String) -> (String, i32) {
    let split = line.split(" ").collect::<Vec<&str>>();

    (split[0].to_string(), split[1].parse::<i32>().unwrap())
}

fn get_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("failed to read file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("unable to parse line"))
        .collect()
}
