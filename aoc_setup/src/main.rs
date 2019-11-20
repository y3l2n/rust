use std::fs::File;
use std::io::{Read};

#[cfg(test)]
mod part1_tests {
    use super::*;    
    #[test]
    fn first() {
        let input = String::from("+1\n+1\n+1");
        assert_eq!(solve(&input), 3);
    }
    #[test]
    fn second() {
        let input = String::from("+1\n+1\n-2");
        assert_eq!(solve(&input), 0);
    }
    #[test]
    fn third() {
        let input = String::from("-1\n-2\n-3");
        assert_eq!(solve(&input), -6);
    }    
}

fn solve(s: &str) -> i32 {
    let numbers: Vec<i32> = s.split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();
    return numbers.iter().sum();
}


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
    println!("output: {}", path);
}
