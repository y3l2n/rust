use std::fs::File;
use std::io::{Read};
use aoc_setup::{solve};


fn run(path: &str) {
    let mut input = File::open(path).unwrap();
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    let solution = solve(&buffer);
    println!("{}", solution);
}

fn main() {
    let path: String = String::from("input.txt");
    run(&path);
}
