use crate::Solution;
use anyhow::{Context, Result};

pub fn solve(part: u8, input: &'static str) -> Result<Solution, anyhow::Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        "" // include_str!("../../data/XX.input")
    };

    if part == 1 {
        Ok(Solution::U64(0))
    } else {
        Ok(Solution::U64(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    #[allow(unused)]
    fn part_1_sample() {
        assert_eq!(solve(1, r"...").unwrap(), Solution::U64(0));
    }

    // #[test]
    #[allow(unused)]
    fn part_1() {
        assert_eq!(solve(1, "").unwrap(), Solution::U64(0));
    }

    // #[test]
    #[allow(unused)]
    fn part_2_sample() {
        assert_eq!(solve(2, r"...").unwrap(), Solution::U64(0));
    }

    // #[test]
    #[allow(unused)]
    fn part_2() {
        assert_eq!(solve(2, "").unwrap(), Solution::U64(0));
    }
}
