use crate::Solution;
use anyhow::{Context, Result};
use rustc_hash::FxHashMap;

pub fn solve(part: u8, input: &'static str) -> Result<Solution, anyhow::Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/03.input")
    };

    let w = input.lines().next().context("no 1st line")?.len();
    let data = input.chars().filter(|x| *x != '\n').collect::<Vec<_>>();
    assert!(data.len() % w == 0);
    let h = data.len() / w;

    // easy bounds
    let w = w as isize;
    let h = h as isize;

    let idx = |row, col| (row * w + col) as usize;

    let number = |row, col_start, col_end| {
        let start = idx(row, col_start);
        let end = idx(row, col_end);
        let s = &data[start..=end];
        // avoid temporary string by doing manual parsing
        let mut res = 0;
        for d in s {
            res *= 10;
            res += d.to_digit(10).expect("digit already checked");
        }
        res as u64
    };

    if part == 1 {
        let is_symbol = |row, col| {
            if row >= 0 && row < h && col >= 0 && col < w {
                let c = data[idx(row, col)];
                !c.is_digit(10) && c != '.'
            } else {
                false
            }
        };
        let have_adjacent_symbol = |row, col_start, col_end| {
            for row in [row - 1, row + 1] {
                for col in (col_start - 1)..=(col_end + 1) {
                    if is_symbol(row, col) {
                        return true;
                    }
                }
            }
            is_symbol(row, col_start - 1) || is_symbol(row, col_end + 1)
        };

        let mut res = 0;
        for row in 0..h {
            let (mut col_start, mut col_end) = (None, None);
            for col in 0..=w {
                let digit = col != w && data[idx(row, col)].is_digit(10);
                match (digit, col_start) {
                    (true, None) => col_start = Some(col),
                    (true, Some(..)) => col_end = Some(col),
                    (false, None) => (),
                    (false, Some(start)) => {
                        let end = col_end.or(Some(start)).context("col_end undefined")?;
                        if have_adjacent_symbol(row, start, end) {
                            res += number(row, start, end);
                        }
                        (col_start, col_end) = (None, None);
                    }
                }
            }
        }
        Ok(Solution::U64(res))
    } else {
        // key: row, col value: count, product
        type Stars = FxHashMap<(isize, isize), (usize, u64)>;
        let mut stars: Stars = FxHashMap::default();

        let is_star = |row, col| {
            if row >= 0 && row < h && col >= 0 && col < w {
                let c = data[idx(row, col)];
                c == '*'
            } else {
                false
            }
        };
        let flag_star = |stars: &mut Stars, row, col, num| {
            if is_star(row, col) {
                stars
                    .entry((row, col))
                    .and_modify(|x| *x = (x.0 + 1, x.1 * num))
                    .or_insert((1, num));
            }
        };
        let flag_adjacent_stars = |stars: &mut Stars, row, start, end| {
            let num = number(row, start, end);
            for row in [row - 1, row + 1] {
                for col in (start - 1)..=(end + 1) {
                    flag_star(stars, row, col, num);
                }
            }
            flag_star(stars, row, start - 1, num);
            flag_star(stars, row, end + 1, num);
        };

        for row in 0..h {
            let (mut col_start, mut col_end) = (None, None);
            for col in 0..=w {
                let digit = col != w && data[idx(row, col)].is_digit(10);
                match (digit, col_start) {
                    (true, None) => col_start = Some(col),
                    (true, Some(..)) => col_end = Some(col),
                    (false, None) => (),
                    (false, Some(start)) => {
                        let end = col_end.or(Some(start)).context("col_end undefined")?;
                        flag_adjacent_stars(&mut stars, row, start, end);
                        (col_start, col_end) = (None, None);
                    }
                }
            }
        }

        let res = stars
            .iter()
            .filter_map(|(k, v)| if v.0 == 2 { Some(v.1) } else { None })
            .sum();
        Ok(Solution::U64(res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part_1() {
        assert_eq!(solve(1, SAMPLE).unwrap(), Solution::U64(4361));
    }

    #[test]
    #[allow(unused)]
    fn part_2() {
        assert_eq!(solve(2, SAMPLE).unwrap(), Solution::U64(467835));
    }
}
