#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn solve(s: &str) -> i32 {
    let numbers: Vec<i32> = s.split_whitespace().map(|s| s.parse().unwrap()).collect();
    return numbers.iter().sum();
}
