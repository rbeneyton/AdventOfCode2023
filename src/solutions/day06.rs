use crate::Solution;
use anyhow::{Context, Result};

pub fn solve(part: u8, input: &'static str) -> Result<Solution, anyhow::Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/06.input")
    };

    if part == 1 {
        let mut lines = input.lines();
        let times = lines
            .next()
            .unwrap()
            .split(' ')
            .filter(|x| x.len() > 0)
            .skip(1)
            .map(|x| x.parse::<i64>().expect("times"))
            .collect::<Vec<_>>();

        let distances = lines
            .next()
            .unwrap()
            .split(' ')
            .filter(|x| x.len() > 0)
            .skip(1)
            .map(|x| x.parse::<i64>().expect("times"))
            .collect::<Vec<_>>();

        debug_assert_eq!(times.len(), distances.len());
        let n = times.len();

        let race = |time, distance| {
            let (mut min_hold, mut max_hold) = (None, None);
            for hold in 1..time {
                if hold * (time - hold) > distance {
                    if min_hold.is_none() {
                        min_hold = Some(hold);
                    }
                    max_hold = Some(hold);
                }
            }
            1 + max_hold.unwrap() - min_hold.unwrap()
        };
        let res = (0..n)
            .into_iter()
            .map(|x| race(times[x], distances[x]) as u64)
            .product();

        Ok(Solution::U64(res))
    } else {
        let mut lines = input.lines();
        let (_, time) = lines.next().unwrap().split_once(':').context("no :")?;
        let time = time.to_string();
        let time = time.replace(" ", "");
        let time = time.parse::<i64>().expect("time");

        let (_, distance) = lines.next().unwrap().split_once(':').context("no :")?;
        let distance = distance.to_string();
        let distance = distance.replace(" ", "");
        let distance = distance.parse::<i64>().expect("distance");

        let race = |time, distance| {
            let (mut min_hold, mut max_hold) = (None, None);
            for hold in 1..time {
                if hold * (time - hold) > distance {
                    if min_hold.is_none() {
                        min_hold = Some(hold);
                    }
                    max_hold = Some(hold);
                }
            }
            1 + max_hold.unwrap() - min_hold.unwrap()
        };
        let res = race(time, distance) as u64;

        Ok(Solution::U64(res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = r"Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part_1() {
        assert_eq!(solve(1, SAMPLE).unwrap(), Solution::U64(288));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, SAMPLE).unwrap(), Solution::U64(71503));
    }
}
