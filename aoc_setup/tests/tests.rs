use aoc_setup::{solve};

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