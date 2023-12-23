use crate::Solution;
use anyhow::{Context, Result};
use itertools::Itertools;
use rustc_hash::FxHashSet;

pub fn solve(part: u8, input: &'static str) -> Result<Solution, anyhow::Error> {
    let step_threshold = if part == 1 { 64 } else { 26501365 };
    solve_with_step(part, input, step_threshold)
}

pub fn solve_with_step(
    part: u8,
    input: &'static str,
    step_threshold: usize,
) -> Result<Solution, anyhow::Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/21.input")
    };

    let w = input.lines().next().context("no 1st line")?.len();
    let data = input.chars().filter(|x| *x != '\n').collect::<Vec<_>>();
    let n = data.len();
    assert!(n % w == 0);
    let h = n / w;

    // easy bounds
    let w = w as isize;
    let h = h as isize;
    dbg!(w, h);

    let valid = |row, col| row >= 0 && row < h && col >= 0 && col < w;
    let idx = |row, col| (row * w + col) as usize;
    let row_col = |idx| (idx / w, idx % w);

    let start = data
        .iter()
        .enumerate()
        .filter_map(|(idx, x)| if *x == 'S' { Some(idx) } else { None })
        .next()
        .context("no S found in data")? as isize;

    // movement
    let move_op = |dir, row, col| match dir {
        0 => (row - 1, col), // north
        1 => (row, col - 1), // west
        2 => (row + 1, col), // south
        3 => (row, col + 1), // east
        _ => unreachable!(),
    };

    if part == 1 {
        let mut mask = Vec::new();
        mask.resize(n, false);
        mask[start as usize] = true;
        let mut tmp = Vec::with_capacity(n);
        tmp.resize(n, false);
        let mut step = 0;
        loop {
            for from in (0..n).filter(|x| mask[*x]) {
                let (row, col) = row_col(from as isize);
                for direction in 0..4 {
                    let (row, col) = move_op(direction, row, col);
                    if !valid(row, col) {
                        continue;
                    }
                    let idx = idx(row, col);
                    if data[idx] != '#' {
                        tmp[idx] = true;
                    }
                }
            }
            step += 1;
            std::mem::swap(&mut tmp, &mut mask);
            if step == step_threshold {
                break;
            }
            tmp.clear();
            tmp.resize(n, false);
        }
        let res = (0..n).filter(|x| mask[*x]).count();

        Ok(Solution::U64(res as u64))
    } else {
        let mut results = vec![FxHashSet::default()]; // 0 move

        // compute 5x5 grid
        let scale = 5;
        let (w3, h3) = (w * scale, h * scale);
        let mut grid3 = Vec::new();
        grid3.resize(data.len() * ((scale * scale) as usize), ' ');
        let idx3 = |row, col| (row * w3 + col) as usize;
        let row_col3 = |idx| (idx / w3, idx % w3);
        let valid3 = |row, col| row >= 0 && row < h3 && col >= 0 && col < w3;
        for row in 0..h {
            for mul_row in 0..scale {
                for col in 0..w {
                    let c = data[idx(row, col)];
                    let c = if c == 'S' { '.' } else { c };
                    for mul_col in 0..scale {
                        grid3[idx3(row + mul_row * h, col + mul_col * h)] = c;
                    }
                }
            }
        }
        // do switch
        let (start_row, start_col) = (start / w, start % w);
        dbg!(start_row, start_col);
        let start = idx3(
            start_row + (scale - 1) / 2 * h,
            start_col + (scale - 1) / 2 * h,
        ) as isize;
        {
            let (w, h) = (w3, h3);
            let idx = idx3;
            let valid = valid3;
            let row_col = row_col3;
            let data = grid3;

            let mut res = [FxHashSet::<isize>::default(), FxHashSet::<isize>::default()];
            let mut mask = FxHashSet::default();
            mask.insert(start);
            let mut tmp = FxHashSet::default();
            let mut step = 0;
            'extend: loop {
                for from in &mask {
                    // if res[step % 2].contains(from) { continue; }
                    let (row, col) = row_col(*from);
                    for direction in 0..4 {
                        let (row, col) = move_op(direction, row, col);
                        if !valid(row, col) {
                            break 'extend;
                        }
                        let idx = idx(row, col);
                        if data[idx] != '#' {
                            tmp.insert(idx as isize);
                        }
                    }
                }
                step += 1;
                res[step % 2].extend(tmp.iter());
                std::mem::swap(&mut tmp, &mut mask);
                // println!("step:{step}, len:{}", res[step % 2].len());
                results.push(res[step % 2].clone());
                if false {
                    for col in 0..w {
                        for row in 0..h {
                            let idx = idx(row, col) as isize;
                            let seen = res[step % 2].contains(&idx) || mask.contains(&idx);
                            print!("{}", if seen { 'X' } else { data[idx as usize] });
                        }
                        println!("");
                    }
                }
                if step == step_threshold {
                    break;
                }
                tmp.clear();
            }
        }
        dbg!(results.len());

        // hidden assumptions in the data (this contest is really crap)
        // direct road from S to each 4 directions (see map), so radius is…
        debug_assert_eq!(w, h); // easy square only
        debug_assert_eq!(start_row, w / 2); // easy start from center only
        debug_assert_eq!(start_col, w / 2); // easy start from center only
        let w_usize = w as usize;
        let step_threshold = step_threshold as usize;
        let (radius, offset) = (step_threshold / w_usize, step_threshold % w_usize);
        dbg!(radius, offset);
        debug_assert_eq!(offset % w_usize, w_usize / 2); // easy symmetrical compute

        // by design
        assert_eq!(results.len(), (2 * w + start_row + 1) as usize);
        let simulation = &results[(2 * w + start_row) as usize];

        println!(
            "results: {}",
            itertools::join(
                results
                    .iter()
                    .enumerate()
                    .map(|(idx, r)| format!("{}:{}", idx, r.len())),
                ","
            )
        );
        // println!("[{}]", itertools::join(results.iter().map(|(_, r)| format!("{}", r.len())), ","));
        // let res = res[step_threshold % 2].len();

        debug_assert_eq!(step_threshold % 2, 1); // only odd layout

        let sz = |row_min, col_min| {
            simulation
                .iter()
                .filter(|x| {
                    let (row_min, row_max) = (row_min * h, (row_min + 1) * h);
                    let (col_min, col_max) = (col_min * h, (col_min + 1) * h);
                    let (row, col) = row_col3(**x);
                    row >= row_min && row < row_max && col > col_min && col < col_max
                })
                .count()
        };

        // {{{ main inner blocks
        let same_parity_block = sz(2, 2); // same parity block
        let diff_parity_block = sz(2, 1); // different parity block
        debug_assert_eq!(diff_parity_block, sz(2, 3)); // different parity block
        debug_assert_eq!(diff_parity_block, sz(1, 2)); // different parity block
        dbg!(same_parity_block, diff_parity_block);
        // }}}
        // {{{ corner blocks
        let corner_block_w = sz(0, 2);
        let corner_block_o = sz(4, 2);
        let corner_block_n = sz(2, 0);
        let corner_block_s = sz(2, 4);
        let corner_blocks = corner_block_w + corner_block_o + corner_block_n + corner_block_s;
        dbg!(
            corner_block_w,
            corner_block_o,
            corner_block_n,
            corner_block_s
        );
        // }}}
        // {{{ edge blocks
        let corner_block_wn = sz(1, 1);
        let corner_block_on = sz(3, 1);
        let corner_block_ws = sz(1, 3);
        let corner_block_os = sz(3, 3);
        let corner_block_2 = corner_block_wn + corner_block_on + corner_block_ws + corner_block_os;
        dbg!(
            corner_block_wn,
            corner_block_on,
            corner_block_ws,
            corner_block_os
        );
        // }}}
        // {{{ oppososie parity edge blocks
        let corner_block_wn_opp = sz(1, 0);
        let corner_block_on_opp = sz(1, 4);
        let corner_block_ws_opp = sz(3, 0);
        let corner_block_os_opp = sz(3, 4);
        debug_assert_eq!(sz(1, 0), sz(0, 1)); // different parity block
        debug_assert_eq!(sz(1, 4), sz(0, 3)); // different parity block
        debug_assert_eq!(sz(3, 0), sz(4, 1)); // different parity block
        debug_assert_eq!(sz(3, 4), sz(4, 3)); // different parity block
        let corner_block_2_opp =
            corner_block_wn_opp + corner_block_on_opp + corner_block_ws_opp + corner_block_os_opp;
        dbg!(
            corner_block_wn,
            corner_block_on,
            corner_block_ws,
            corner_block_os
        );
        // }}}

        for row in 0..scale {
            for col in 0..scale {
                println!(
                    "{}/{}: {}",
                    row,
                    col,
                    simulation
                        .iter()
                        .filter(|x| {
                            let (row_min, row_max) = (row * h, (row + 1) * h);
                            let (col_min, col_max) = (col * h, (col + 1) * h);
                            let (row, col) = row_col3(**x);
                            row >= row_min && row < row_max && col > col_min && col < col_max
                        })
                        .count()
                );
            }
        }

        // compute exact same block of each types
        let r = radius;
        let res = (r - 1) * (r - 1) * same_parity_block
            + r * r * diff_parity_block
            + corner_blocks
            + (r - 1) * corner_block_2
            + r * corner_block_2_opp;

        // 630989257278542 no
        Ok(Solution::U64(res as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = r"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn part_1() {
        assert_eq!(solve_with_step(1, SAMPLE, 6).unwrap(), Solution::U64(16));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_with_step(2, SAMPLE, 1).unwrap(), Solution::U64(2));
        assert_eq!(solve_with_step(2, SAMPLE, 6).unwrap(), Solution::U64(16));
        assert_eq!(solve_with_step(2, SAMPLE, 10).unwrap(), Solution::U64(50));
        assert_eq!(solve_with_step(2, SAMPLE, 49).unwrap(), Solution::U64(1594));
        assert_eq!(solve_with_step(2, SAMPLE, 50).unwrap(), Solution::U64(1594));
        assert_eq!(
            solve_with_step(2, SAMPLE, 100).unwrap(),
            Solution::U64(6536)
        );
        assert_eq!(
            solve_with_step(2, SAMPLE, 500).unwrap(),
            Solution::U64(167004)
        );
        assert_eq!(
            solve_with_step(2, SAMPLE, 1000).unwrap(),
            Solution::U64(668697)
        );
        assert_eq!(
            solve_with_step(2, SAMPLE, 5000).unwrap(),
            Solution::U64(16733044)
        );
    }
}
