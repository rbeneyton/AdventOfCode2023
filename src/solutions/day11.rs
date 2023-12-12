use crate::Solution;
use anyhow::{Context, Result};
use std::collections::{BTreeMap, BTreeSet};

pub fn solve(part: u8, input: &'static str) -> Result<Solution, anyhow::Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/11.input")
    };

    // collect galaxies locations
    let mut galaxies = Vec::new();
    let mut galaxies_rows = BTreeSet::new();
    let mut galaxies_cols = BTreeSet::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                let (row, col) = (row as isize, col as isize);
                galaxies.push((row, col));
                galaxies_rows.insert(row);
                galaxies_cols.insert(col);
            }
        }
    }

    // do the expand parts
    let expand = if part == 1 { 2 } else { 10 };
    let expand = if part == 1 { 2 } else { 100 };
    let expand = if part == 1 { 2 } else { 1000000 };
    // rows
    let row_min = *galaxies_rows.first().unwrap();
    let row_max = *galaxies_rows.last().unwrap();
    let mut shift = 0;
    let mut shifts = BTreeMap::new();
    for row in row_min..=row_max {
        shift += if galaxies_rows.contains(&row) {
            1
        } else {
            expand
        };
        shifts.insert(row, shift);
    }
    for galaxy in galaxies.iter_mut() {
        galaxy.0 = *shifts.get(&galaxy.0).context("unseen row")?;
    }
    // cols
    let col_min = *galaxies_cols.first().unwrap();
    let col_max = *galaxies_cols.last().unwrap();
    let mut shift = 0;
    let mut shifts = BTreeMap::new();
    for col in col_min..=col_max {
        shift += if galaxies_cols.contains(&col) {
            1
        } else {
            expand
        };
        shifts.insert(col, shift);
    }
    for galaxy in galaxies.iter_mut() {
        galaxy.1 = *shifts.get(&galaxy.1).context("unseen col")?;
    }

    // compute distances
    let n = galaxies.len();
    let mut distances = 0;
    for g1 in 0..n {
        for g2 in (g1 + 1)..n {
            let (g1, g2) = (galaxies[g1], galaxies[g2]);
            let (drow, dcol) = (g1.0 - g2.0, g1.1 - g2.1);
            distances += drow.abs() + dcol.abs();
        }
    }

    Ok(Solution::U64(distances as u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn part_1_sample() {
        assert_eq!(solve(1, SAMPLE).unwrap(), Solution::U64(374));
    }

    // #[test]
    // fn part_2() {
    //     //assert_eq!(solve(2, SAMPLE).unwrap(), Solution::U64(1030));
    //     //assert_eq!(solve(2, SAMPLE).unwrap(), Solution::U64(8410));
    // }
}
