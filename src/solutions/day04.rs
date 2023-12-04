use crate::Solution;
use anyhow::{Context, Result};
use rustc_hash::FxHashMap;

pub fn solve(part: u8, input: &'static str) -> Result<Solution, anyhow::Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/04.input")
    };

    if part == 1 {
        let mut res = 0;
        for line in input.lines() {
            let (_, line) = line.split_once(": ").context("no : separator")?;
            let (winnings, numbers) = line.split_once(" | ").context("no | separator")?;
            let (mut wins, mut mines) = (0u128, 0u128);
            for (dst, src) in [(&mut wins, winnings), (&mut mines, numbers)] {
                for i in src
                    .split(" ")
                    .filter(|x| x.len() > 0)
                    .map(|x| x.parse::<u8>().context(format!(" cannot parse number {x}")))
                {
                    *dst |= 1 << i?;
                }
            }
            let win = wins & mines;
            if win != 0 {
                res += 1 << (win.count_ones() - 1);
            }
        }

        Ok(Solution::U64(res))
    } else {
        let mut res = 0;
        let mut copies = FxHashMap::default();
        for (id, line) in input.lines().enumerate() {
            let (_, line) = line.split_once(": ").context("no : separator")?;
            let (winnings, numbers) = line.split_once(" | ").context("no | separator")?;
            let (mut wins, mut mines) = (0u128, 0u128);
            for (dst, src) in [(&mut wins, winnings), (&mut mines, numbers)] {
                for i in src
                    .split(" ")
                    .filter(|x| x.len() > 0)
                    .map(|x| x.parse::<u8>().context(format!(" cannot parse number {x}")))
                {
                    *dst |= 1 << i?;
                }
            }
            let win = wins & mines;
            let win = win.count_ones() as usize;

            // number of current card
            let n: u64 = 1 + copies.remove(&id).or(Some(0)).unwrap();
            res += n;

            // propagate copies
            for copy in (id + 1)..=(id + win) {
                copies.entry(copy).and_modify(|x| *x += n).or_insert(n);
            }
        }
        Ok(Solution::U64(res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part_1() {
        assert_eq!(solve(1, SAMPLE).unwrap(), Solution::U64(13));
    }

    #[test]
    #[allow(unused)]
    fn part_2() {
        assert_eq!(solve(2, SAMPLE).unwrap(), Solution::U64(30));
    }
}
