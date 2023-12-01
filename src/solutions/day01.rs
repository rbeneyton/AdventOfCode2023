use crate::Solution;
use anyhow::{Error, Result};

pub fn solve(part: u8, input: &'static str) -> Result<Solution, Error> {
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/01.input")
    };

    if part == 1 {
        let mut res = 0u32;
        for line in input.lines().filter(|x| x.len() > 1) {
            let first = line
                .chars()
                .filter(|x| x.is_digit(10))
                .next()
                .expect("no digit at all");
            let first = first.to_digit(10).unwrap();
            let last = line
                .chars()
                .rev()
                .filter(|x| x.is_digit(10))
                .next()
                .expect("no digit at all");
            let last = last.to_digit(10).unwrap();
            let num = first * 10 + last;
            res += num;
        }
        Ok(Solution::U32(res))
    } else {
        let mut res = 0u32;
        let nums = vec![
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];
        for line in input.lines().filter(|x| x.len() > 1) {
            let (_, first) = nums
                .iter()
                .filter_map(|(k, v)| line.find(k).map(|x| (x, v)))
                .min_by(|a, b| a.0.cmp(&b.0))
                .expect("no digit neither number at all");
            let (_, last) = nums
                .iter()
                .filter_map(|(k, v)| line.rfind(k).map(|x| (x, v)))
                .max_by(|a, b| a.0.cmp(&b.0))
                .expect("no digit neither number at all");

            let num = first * 10 + last;
            res += num;
        }
        Ok(Solution::U32(res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_last() {
        let input = r"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(solve(1, input).ok().unwrap(), Solution::U32(142));
    }

    #[test]
    fn part_2_last() {
        let input = r"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(solve(2, input).unwrap(), Solution::U32(281));
    }
}
