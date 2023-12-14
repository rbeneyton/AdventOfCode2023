use crate::Solution;
use anyhow::{Context, Result};

pub fn solve(part: u8, input: &'static str) -> Result<Solution, anyhow::Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/14.input")
    };

    let w = input.lines().next().context("no 1st line")?.len();
    let mut data = input
        .chars()
        .filter(|x| *x != '\n')
        .map(|x| match x {
            '.' => 0u8,
            '#' => 1u8,
            'O' => 2u8,
            _ => panic!("invalid character"),
        })
        .collect::<Vec<_>>();
    assert!(data.len() % w == 0);
    let h = data.len() / w;

    // easy bounds
    let w = w as isize;
    let h = h as isize;
    let idx = |row, col| (row * w + col) as usize;

    let north = |data: &mut Vec<u8>| {
        for col in 0..w {
            let mut free = None;
            let mut row = 0;
            loop {
                match data[idx(row, col)] {
                    0 => {
                        if free == None {
                            free = Some(row);
                        }
                    }
                    1 => free = None,
                    2 => {
                        if let Some(x) = free {
                            data.swap(idx(row, col), idx(x, col));
                            row = x;
                            free = None;
                            continue;
                        }
                    }
                    _ => unreachable!(),
                }
                row += 1;
                if row == h {
                    break;
                }
            }
        }
    };
    let south = |data: &mut Vec<u8>| {
        for col in 0..w {
            let mut free = None;
            let mut row = h - 1;
            loop {
                match data[idx(row, col)] {
                    0 => {
                        if free == None {
                            free = Some(row);
                        }
                    }
                    1 => free = None,
                    2 => {
                        if let Some(x) = free {
                            data.swap(idx(row, col), idx(x, col));
                            row = x;
                            free = None;
                            continue;
                        }
                    }
                    _ => unreachable!(),
                }
                if row == 0 {
                    break;
                }
                row -= 1;
            }
        }
    };
    let west = |data: &mut Vec<u8>| {
        for row in 0..h {
            let mut free = None;
            let mut col = 0;
            loop {
                match data[idx(row, col)] {
                    0 => {
                        if free == None {
                            free = Some(col);
                        }
                    }
                    1 => free = None,
                    2 => {
                        if let Some(x) = free {
                            data.swap(idx(row, col), idx(row, x));
                            col = x;
                            free = None;
                            continue;
                        }
                    }
                    _ => unreachable!(),
                }
                col += 1;
                if col == w {
                    break;
                }
            }
        }
    };
    let east = |data: &mut Vec<u8>| {
        for row in 0..h {
            let mut free = None;
            let mut col = w - 1;
            loop {
                match data[idx(row, col)] {
                    0 => {
                        if free == None {
                            free = Some(col);
                        }
                    }
                    1 => free = None,
                    2 => {
                        if let Some(x) = free {
                            data.swap(idx(row, col), idx(row, x));
                            col = x;
                            free = None;
                            continue;
                        }
                    }
                    _ => unreachable!(),
                }
                if col == 0 {
                    break;
                }
                col -= 1;
            }
        }
    };

    let score = |data: &Vec<u8>| {
        let mut res = 0;
        for (jdx, row) in (0..h).enumerate() {
            let weigth = (h as usize) - jdx;
            res += (0..w)
                .map(|x| data[idx(row, x)])
                .filter(|x| *x == 2)
                .count()
                * weigth;
        }
        res
    };

    if part == 1 {
        north(&mut data);
        let res = score(&data);

        Ok(Solution::U64(res as u64))
    } else {
        let mut scores = Vec::new();
        for idx in 1..=400 {
            north(&mut data);
            west(&mut data);
            south(&mut data);
            east(&mut data);
            let score = score(&data);
            scores.push(score);
        }
        let n = scores.len();
        let mut res = 0;
        for cycle in 2..100 {
            let n1 = n - cycle;
            let n2 = n - 2 * cycle;
            let n3 = n - 3 * cycle;
            let n4 = n - 4 * cycle;
            if scores[n1..n] == scores[n2..n1]
                && scores[n2..n1] == scores[n3..n2]
                && scores[n3..n2] == scores[n4..n3]
            {
                println!(
                    "detected cycle of {cycle}: {}",
                    itertools::join(scores[n1..n].iter().map(|x| format!("{}", x)), ",")
                );
                let rem = (1000000000 - n) % cycle;
                res = scores[n - 1 - cycle + rem];
                break;
            }
        }
        Ok(Solution::U64(res as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn part_1() {
        assert_eq!(solve(1, SAMPLE).unwrap(), Solution::U64(136));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, SAMPLE).unwrap(), Solution::U64(64));
    }
}
