use crate::Solution;
use anyhow::{Context, Result};
use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

pub fn solve(part: u8, input: &'static str) -> Result<Solution, anyhow::Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/18.input")
    };

    let mut points = vec![(0isize, 0isize)];
    for line in input.lines() {
        let (row, col) = points.last().unwrap();
        let (row, col) = (*row, *col);

        let (dir, sz) = if part == 1 {
            let (dir, sz, _) = line
                .split(' ')
                .collect_tuple()
                .with_context(|| format!("cannot split '{line}'"))?;
            let dir = dir
                .chars()
                .next()
                .with_context(|| format!("invalid dir '{dir}'"))?;
            let sz = sz
                .parse::<isize>()
                .with_context(|| format!("invalid size '{sz}'"))?;
            (dir, sz)
        } else {
            let (_, _, instr) = line
                .split(' ')
                .collect_tuple()
                .with_context(|| format!("cannot split '{line}'"))?;
            let (_, instr, _) = instr
                .split(&['(', ')'])
                .collect_tuple()
                .with_context(|| format!("cannot split '{instr}'"))?;
            debug_assert_eq!(instr.chars().next(), Some('#'));
            debug_assert_eq!(instr.chars().count(), 7);
            let sz = instr.chars().skip(1).take(5).collect::<String>();
            let sz = isize::from_str_radix(sz.as_str(), 16)
                .with_context(|| format!("invalid hex number '{sz}'"))?;
            let dir = instr
                .chars()
                .skip(6)
                .next()
                .with_context(|| format!("invalid dir in '{instr}'"))?;
            let dir = match dir {
                '0' => 'R',
                '2' => 'L',
                '3' => 'U',
                '1' => 'D',
                _ => unreachable!(),
            };
            (dir, sz)
        };

        points.push(match dir {
            'R' => (row, col + sz),
            'L' => (row, col - sz),
            'U' => (row - sz, col),
            'D' => (row + sz, col),
            _ => unreachable!(),
        });
    }

    // switch to indirect coordinates
    let mut rows = BTreeSet::new();
    let mut cols = BTreeSet::new();
    for (row, col) in &points {
        rows.insert(*row);
        cols.insert(*col);
    }
    let (rows, cols) = (rows, cols);

    // build bands
    // rows
    let mut rows_pos_to_idx = BTreeMap::new();
    let mut rows_idx_to_size = BTreeMap::new();
    let mut i = 1isize; // 0 will be used by border
    for (a, b) in rows.iter().tuple_windows() {
        let (a, b) = (*a, *b);
        if !rows_pos_to_idx.contains_key(&a) {
            rows_pos_to_idx.insert(a, i);
            rows_idx_to_size.insert(i, 1);
            i += 1;
        }
        let d = b - 1 - a;
        if d > 0 {
            rows_pos_to_idx.insert(a + 1, i);
            rows_idx_to_size.insert(i, d);
            i += 1;
        }
        rows_pos_to_idx.insert(b, i);
        rows_idx_to_size.insert(i, 1);
        i += 1;
    }

    // cols
    let mut cols_pos_to_idx = BTreeMap::new();
    let mut cols_idx_to_size = BTreeMap::new();
    let mut i = 1isize; // 0 will be used by border
    for (a, b) in cols.iter().tuple_windows() {
        let (a, b) = (*a, *b);
        if !cols_pos_to_idx.contains_key(&a) {
            cols_pos_to_idx.insert(a, i);
            cols_idx_to_size.insert(i, 1);
            i += 1;
        }
        let d = b - 1 - a;
        if d > 0 {
            cols_pos_to_idx.insert(a + 1, i);
            cols_idx_to_size.insert(i, d);
            i += 1;
        }
        cols_pos_to_idx.insert(b, i);
        cols_idx_to_size.insert(i, 1);
        i += 1;
    }

    // migrate points to new coordinates
    let mut points2 = Vec::with_capacity(points.len());
    for (row, col) in &points {
        let row = *rows_pos_to_idx.get(row).context("row mapping error")?;
        let col = *cols_pos_to_idx.get(col).context("col mapping error")?;
        points2.push((row, col));
    }
    let points = points2;

    // build map
    let w = cols_idx_to_size.len() + 2;
    let h = rows_idx_to_size.len() + 2;
    let mut map = Vec::new();
    // 0: empty, 1: wall
    map.resize(w * h, 0u8);
    let w = w as isize;
    let h = h as isize;
    let idx = |row, col| (row * w + col) as usize;
    let valid = |row, col| row >= 0 && row < h && col >= 0 && col < w;
    for ((rowa, cola), (rowb, colb)) in points.iter().tuple_windows() {
        let (rowa, cola) = (*rowa as isize, *cola as isize);
        let (rowb, colb) = (*rowb as isize, *colb as isize);
        let (rowa, rowb) = (std::cmp::min(rowa, rowb), std::cmp::max(rowa, rowb));
        let (cola, colb) = (std::cmp::min(cola, colb), std::cmp::max(cola, colb));
        // lines, no square
        debug_assert!(rowb == rowa || colb == cola);
        for row in rowa..=rowb {
            for col in cola..=colb {
                map[idx(row, col)] = 1;
            }
        }
    }

    // fill '2' from outside
    let mut states = vec![(0, 0)];
    map[0] = 2;
    let mut tmp = Vec::new();
    loop {
        for state in &states {
            for (drow, dcol) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (row, col) = (state.0 + drow, state.1 + dcol);
                if !valid(row, col) {
                    continue;
                }
                let idx = idx(row, col);
                if map[idx] == 0 {
                    map[idx] = 2;
                    tmp.push((row, col));
                }
            }
        }
        if tmp.len() == 0 {
            break;
        }
        std::mem::swap(&mut tmp, &mut states);
        tmp.clear();
    }

    // scan inner pixel and do size mapping
    let res = (0..map.len())
        .filter(|x| map[*x] != 2)
        .map(|x| x as isize)
        .map(|x| {
            let (row, col) = (x / w, x % w);
            assert!(row > 0 && col > 0); // no in border
            let row_sz = rows_idx_to_size.get(&row).expect("mapping error");
            let col_sz = cols_idx_to_size.get(&col).expect("mapping error");
            row_sz * col_sz
        })
        .sum::<isize>() as u64;

    Ok(Solution::U64(res))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn part_1() {
        assert_eq!(solve(1, SAMPLE).unwrap(), Solution::U64(62));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, SAMPLE).unwrap(), Solution::U64(952408144115));
    }
}
