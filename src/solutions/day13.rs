use crate::Solution;
use anyhow::Result;
use itertools::Itertools;

pub fn solve(part: u8, input: &'static str) -> Result<Solution, anyhow::Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/13.input")
    };

    if part == 1 {
        let mut res = 0;
        let mut lines = Vec::new();
        let mut cols = Vec::new();
        'pattern: for patterns in input.split("\n\n") {
            // horizontal mirror
            lines.clear();
            for line in patterns.lines() {
                lines.push(line);
            }
            'horizontal: for (idx, (a, b)) in lines.iter().tuple_windows().enumerate() {
                //if same_str(a, b) {
                if a == b {
                    // check other lines
                    let depth = std::cmp::min(idx + 1, lines.len() - idx - 1);
                    for i in 1..depth {
                        if lines[idx - i] != lines[idx + 1 + i] {
                            continue 'horizontal;
                        }
                    }
                    res += (idx + 1) * 100;
                    continue 'pattern;
                }
            }
            // vertical mirror
            let w = lines[0].len();
            let h = lines.len();
            cols.clear();
            cols.resize(w, String::with_capacity(h));
            // TODO pool allocator?
            for col in 0..w {
                for row in 0..h {
                    cols[col].push(lines[row].chars().skip(col).next().unwrap());
                }
            }
            'vertical: for (idx, (a, b)) in cols.iter().tuple_windows().enumerate() {
                //if same_str(a, b) {
                if a == b {
                    // check other lines
                    let depth = std::cmp::min(idx + 1, cols.len() - idx - 1);
                    for i in 1..depth {
                        if cols[idx - i] != cols[idx + 1 + i] {
                            continue 'vertical;
                        }
                    }
                    res += idx + 1;
                    continue 'pattern;
                }
            }
            panic!("no mirrors!");
        }

        Ok(Solution::U64(res as u64))
    } else {
        let diff_str = |a: &str, b: &str| {
            std::iter::zip(a.chars(), b.chars())
                .map(|(a, b)| (a != b) as u8)
                .sum::<u8>()
        };

        let mut res = 0;
        let mut lines = Vec::new();
        let mut cols = Vec::new();
        'pattern: for patterns in input.split("\n\n") {
            // horizontal mirror
            lines.clear();
            for line in patterns.lines() {
                lines.push(line);
            }
            'horizontal: for idx in 0..lines.len() {
                let mut total_diffs = 0;
                let depth = std::cmp::min(idx + 1, lines.len() - idx - 1);
                for i in 0..depth {
                    total_diffs += diff_str(lines[idx - i], lines[idx + 1 + i]);
                    if total_diffs > 1 {
                        continue 'horizontal;
                    }
                }
                if total_diffs == 1 {
                    res += (idx + 1) * 100;
                    continue 'pattern;
                }
            }
            // vertical mirror
            let w = lines[0].len();
            let h = lines.len();
            cols.clear();
            cols.resize(w, String::with_capacity(h));
            // TODO pool allocator?
            for col in 0..w {
                for row in 0..h {
                    cols[col].push(lines[row].chars().skip(col).next().unwrap());
                }
            }
            'vertical: for idx in 0..cols.len() {
                let mut total_diffs = 0;
                let depth = std::cmp::min(idx + 1, cols.len() - idx - 1);
                for i in 0..depth {
                    total_diffs += diff_str(cols[idx - i].as_str(), cols[idx + 1 + i].as_str());
                    if total_diffs > 1 {
                        continue 'vertical;
                    }
                }
                if total_diffs == 1 {
                    res += idx + 1;
                    continue 'pattern;
                }
            }
            panic!("no mirrors!");
        }

        Ok(Solution::U64(res as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn part_1() {
        assert_eq!(solve(1, SAMPLE).unwrap(), Solution::U64(405));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, SAMPLE).unwrap(), Solution::U64(400));
    }
}
