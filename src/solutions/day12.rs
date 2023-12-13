use crate::Solution;
use anyhow::{Context, Result};
use itertools::Itertools;

// fn valid(line: &str, nums: &Vec<u8>, sums: u8) -> bool {
//     let mut prev = ' ';
//     let (mut pounds, mut tok) = (0, 0);
//     let mut num = nums.iter().peekable();
//     let mut c = line.chars().peekable();
//     loop {
//         if let Some(c) = c.next() {
//             // let pound = c == '#';
//             // if pound {
//             //     pounds += 1;
//             //     if pounds > sums { return false; }
//             //     tok += 1;
//             //     if num.peek().is_none() || **num.peek().unwrap() < tok {
//             //         return false;
//             //     }
//             // }
//             // if !dash {
//             //     if prev == '#' && (num.peek().is_none() || *num.next().unwrap() != tok {
//             //         return false;
//             //     }
//             //     tok = 0;
//             // }
//             prev = c;
//         } else {
//             break;
//         }
//     }
//     true
//     // if prev == '#'
//     // num.next() == None
// }
// #[test]
// fn valid_test() {
//     assert_eq!(valid("", &vec![], 0), true);
//     assert_eq!(valid(".", &vec![], 0), true);
//     assert_eq!(valid("#", &vec![1], 1), true);
//     assert_eq!(valid("#.", &vec![1], 1), true);
//     assert_eq!(valid(".#", &vec![1], 1), true);
//     assert_eq!(valid("..#", &vec![1], 1), true);
//     assert_eq!(valid(".#.", &vec![1], 1), true);
//     assert_eq!(valid("#.#.###", &vec![1, 1, 3], 5), true);
//     assert_eq!(valid("#.#.###.", &vec![1, 1, 3], 5), true);
//     assert_eq!(valid("#.#.####", &vec![1, 1, 3], 5), false);
//     assert_eq!(valid("#.#.###.#", &vec![1, 1, 3], 5), false);
//     assert_eq!(valid("#.#.###.#", &vec![1, 1, 3], 6), false);
// }

pub fn solve(part: u8, input: &'static str) -> Result<Solution, anyhow::Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/12.input")
    };

    let mut nums = Vec::with_capacity(10);
    for line in input.lines() {
        let (pats, counts) = line.split(' ').collect_tuple().context("invalid pattern")?;
        // numbers
        nums.clear();
        for count in counts.split(',') {
            nums.push(count.parse::<u8>().context("invalid number")?);
        }
        let n = nums.iter().sum::<u8>();
        // patterns
        let questions = pats.chars().filter(|x| *x == '?').count() as u8;
        let dots = pats.chars().filter(|x| *x == '.').count() as u8;
        let dashs = pats.chars().filter(|x| *x == '#').count() as u8;
        debug_assert_eq!(questions + dots + dashs, pats.chars().count() as u8);
        debug_assert!(n <= dashs);

        let hidden_dash = n - dashs;
        debug_assert!(hidden_dash <= questions);
        let hidden_dot = questions - hidden_dash;
    }
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
