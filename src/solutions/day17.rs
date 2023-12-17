use crate::Solution;
use anyhow::{Context, Result};

pub fn solve(part: u8, input: &'static str) -> Result<Solution, anyhow::Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/17.input")
    };

    let w = input.lines().next().context("no 1st line")?.len();
    let data = input
        .chars()
        .filter(|x| *x != '\n')
        .map(|x| x.to_digit(10).expect("invalid digit") as u8)
        .collect::<Vec<_>>();
    assert!(data.len() % w == 0);
    let h = data.len() / w;

    // easy bounds
    let w = w as isize;
    let h = h as isize;

    let valid = |row, col| row >= 0 && row < h && col >= 0 && col < w;
    let idx = |row, col| (row * w + col) as isize;
    let pos = |idx: isize| (idx / w, idx % w);
    let dist = |idx1, idx2| {
        let (row1, col1) = pos(idx1);
        let (row2, col2) = pos(idx2);
        (row1 - row2).abs() + (col1 - col2).abs()
    };
    let target = (data.len() - 1) as isize;

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct State {
        pos: isize,
        from: isize,
        line: usize,
        heat: isize,
    }

    if part == 1 {
        // optimal path caching: per cell per direction per line
        let mut map = Vec::new();
        const MAX_DIR: usize = 4;
        const MAX_LINE: usize = 3;
        let off = |dir, line| dir * MAX_LINE + line;
        const DIM: usize = MAX_DIR * MAX_LINE;
        map.resize(data.len(), [State::default(); DIM]);

        let step = |i: usize, j: usize, map: &mut Vec<[State; DIM]>| {
            let this = map[i][j];
            let mut res = [None; 4];
            let mut k = 0;
            let (row, col) = pos(this.pos);
            let (from_row, from_col) = pos(this.from);
            let (prev_drow, prev_dcol) = (row - from_row, col - from_col);
            for (dir, new_state) in [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .into_iter()
                .enumerate()
                .map(|(dir, x)| (dir, x.0, x.1))
                .filter(|(_, drow, dcol)| !(*drow == -prev_drow && *dcol == -prev_dcol))
                .map(|(dir, drow, dcol)| (dir, drow, dcol, prev_drow == drow && prev_dcol == dcol))
                .map(|(dir, drow, dcol, dline)| (dir, row + drow, col + dcol, dline))
                .filter(|(_, row, col, _)| valid(*row, *col))
                .map(|(dir, row, col, dline)| (dir, idx(row, col), dline))
                .map(|(dir, pos, dline)| {
                    (
                        dir,
                        State {
                            pos,
                            from: this.pos,
                            line: if dline { this.line + 1 } else { 0 },
                            heat: this.heat + data[pos as usize] as isize,
                        },
                    )
                })
                .filter(|(_, x)| x.line <= 2)
            {
                let pos = new_state.pos as usize;
                let cache = &mut map[pos];
                let off = off(dir, new_state.line);
                let cache = &mut cache[off];

                if cache.heat == 0 || new_state.heat < cache.heat {
                    *cache = new_state;
                    // "collect into array" way
                    res[k] = Some((pos, off));
                    k += 1;
                }
            }
            res
        };

        // Dijkstra
        let mut states = vec![(0, 0)];
        let mut tmp = Vec::new();
        let mut res = isize::MAX;
        loop {
            for state in &states {
                tmp.extend(step(state.0, state.1, &mut map).iter().filter_map(|x| *x));
            }
            if tmp.len() == 0 {
                break;
            }
            std::mem::swap(&mut tmp, &mut states);
            tmp.clear();
        }

        let res = map[target as usize]
            .iter()
            .map(|x| x.heat)
            .filter(|x| *x != 0)
            .min()
            .unwrap();

        Ok(Solution::U64(res as u64))
    } else {
        // optimal path caching: per cell per direction per line
        let mut map = Vec::new();
        const MAX_DIR: usize = 4;
        const MAX_LINE: usize = 10; // 4 to 10
        let off = |dir, line| dir * MAX_LINE + line;
        const DIM: usize = MAX_DIR * MAX_LINE;
        map.resize(data.len(), [State::default(); DIM]);

        let step = |i: usize, j: usize, map: &mut Vec<[State; DIM]>| {
            let this = map[i][j];
            let mut res = [None; 4];
            let mut k = 0;
            let (row, col) = pos(this.pos);
            let (from_row, from_col) = pos(this.from);
            let (prev_drow, prev_dcol) = (row - from_row, col - from_col);
            for (dir, new_state) in [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .into_iter()
                .enumerate()
                .map(|(dir, x)| (dir, x.0, x.1))
                .filter(|(_, drow, dcol)| !(*drow == -prev_drow && *dcol == -prev_dcol))
                .map(|(dir, drow, dcol)| (dir, drow, dcol, prev_drow == drow && prev_dcol == dcol))
                .filter(|(_, _, _, dline)| {
                    if *dline {
                        this.line <= 8
                    } else {
                        this.pos == 0 || this.line >= 3
                    }
                })
                .map(|(dir, drow, dcol, dline)| (dir, row + drow, col + dcol, dline))
                .filter(|(_, row, col, _)| valid(*row, *col))
                .map(|(dir, row, col, dline)| (dir, idx(row, col), dline))
                .map(|(dir, pos, dline)| {
                    (
                        dir,
                        State {
                            pos,
                            from: this.pos,
                            line: if dline { this.line + 1 } else { 0 },
                            heat: this.heat + data[pos as usize] as isize,
                        },
                    )
                })
            {
                let pos = new_state.pos as usize;
                let cache = &mut map[pos];
                let off = off(dir, new_state.line);
                let cache = &mut cache[off];

                if cache.heat == 0 || new_state.heat < cache.heat {
                    *cache = new_state;
                    // "collect into array" way
                    res[k] = Some((pos, off));
                    k += 1;
                }
            }
            res
        };

        // Dijkstra
        let mut states = vec![(0, 0), (0, 1), (0, 2), (0, 3)];
        let mut tmp = Vec::new();
        let mut res = isize::MAX;
        loop {
            for state in &states {
                tmp.extend(step(state.0, state.1, &mut map).iter().filter_map(|x| *x));
            }
            if tmp.len() == 0 {
                break;
            }
            std::mem::swap(&mut tmp, &mut states);
            tmp.clear();
        }

        let res = map[target as usize]
            .iter()
            .map(|x| x.heat)
            .filter(|x| *x != 0)
            .min()
            .unwrap();

        Ok(Solution::U64(res as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn part_1() {
        assert_eq!(solve(1, SAMPLE).unwrap(), Solution::U64(102));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, SAMPLE).unwrap(), Solution::U64(94));
    }
}
