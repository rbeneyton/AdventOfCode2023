use crate::Solution;
use anyhow::Result;
use itertools::Itertools;

pub fn solve(part: u8, input: &'static str) -> Result<Solution, anyhow::Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/09.input")
    };

    let mut nums = Vec::new();
    let mut tmp = Vec::new();
    let mut last_values = Vec::new();
    let mut res = 0;
    for line in input.lines() {
        for num in line.split(' ') {
            nums.push(num.parse::<i64>().expect("invalid number"));
        }
        if part == 2 {
            nums.reverse();
        }
        last_values.push(*nums.last().unwrap());
        for depth in 0.. {
            tmp.clear();
            for (a, b) in nums.iter().tuple_windows() {
                tmp.push(*b - *a);
            }
            last_values.push(*tmp.last().unwrap());
            if tmp.iter().all(|x| *x == 0) {
                break;
            }
            std::mem::swap(&mut tmp, &mut nums);
        }
        nums.clear();
        res += last_values.iter().sum::<i64>();
        last_values.clear();
    }

    Ok(Solution::I64(res))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part_1() {
        assert_eq!(solve(1, SAMPLE).unwrap(), Solution::I64(114));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, SAMPLE).unwrap(), Solution::I64(2));
    }
}
