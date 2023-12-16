use crate::Solution;
use anyhow::{Context, Result};
use enumset::{EnumSet, EnumSetType};

pub fn solve(part: u8, input: &'static str) -> Result<Solution, anyhow::Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/16.input")
    };

    let w = input.lines().next().context("no 1st line")?.len();
    let data = input.chars().filter(|x| *x != '\n').collect::<Vec<_>>();
    assert!(data.len() % w == 0);
    let h = data.len() / w;

    // easy bounds
    let w = w as isize;
    let h = h as isize;

    let idx = |row, col| (row * w + col) as usize;
    let valid = |row, col| row >= 0 && row < h && col >= 0 && col < w;

    #[derive(EnumSetType)]
    enum State {
        U,
        D,
        R,
        L,
    }
    #[derive(Clone, Copy)]
    struct Light {
        state: State,
        row: isize,
        col: isize,
    }
    let step = |light: &Light| {
        let mut o = light.clone();
        let (mut gen_dup, mut dup) = (false, light.clone());
        match data[idx(o.row, o.col)] {
            '.' => match o.state {
                State::U => o.row -= 1,
                State::D => o.row += 1,
                State::R => o.col += 1,
                State::L => o.col -= 1,
                _ => unreachable!(),
            },
            '/' => match o.state {
                State::U => {
                    o.state = State::R;
                    o.col += 1
                }
                State::D => {
                    o.state = State::L;
                    o.col -= 1
                }
                State::R => {
                    o.state = State::U;
                    o.row -= 1
                }
                State::L => {
                    o.state = State::D;
                    o.row += 1
                }
                _ => unreachable!(),
            },
            '\\' => match o.state {
                State::U => {
                    o.state = State::L;
                    o.col -= 1
                }
                State::D => {
                    o.state = State::R;
                    o.col += 1
                }
                State::R => {
                    o.state = State::D;
                    o.row += 1
                }
                State::L => {
                    o.state = State::U;
                    o.row -= 1
                }
                _ => unreachable!(),
            },
            '|' => match o.state {
                State::U => o.row -= 1,
                State::D => o.row += 1,
                State::R | State::L => {
                    gen_dup = true;
                    dup.state = State::U;
                    dup.row -= 1;
                    o.state = State::D;
                    o.row += 1;
                }
                _ => unreachable!(),
            },
            '-' => match o.state {
                State::R => o.col += 1,
                State::L => o.col -= 1,
                State::U | State::D => {
                    gen_dup = true;
                    dup.state = State::R;
                    dup.col += 1;
                    o.state = State::L;
                    o.col -= 1;
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
        let o = if valid(o.row, o.col) { Some(o) } else { None };
        let dup = if gen_dup && valid(dup.row, dup.col) {
            Some(dup)
        } else {
            None
        };

        [o, dup]
    };

    if part == 1 {
        let mut seen = Vec::new();
        seen.resize(data.len(), EnumSet::new());

        let mut moving = Vec::new();
        moving.push(Light {
            state: State::R,
            row: 0,
            col: 0,
        });
        seen[0].insert(State::R);
        let mut tmp = Vec::new();

        loop {
            for light in &moving {
                for i in step(light) {
                    if let Some(i) = i {
                        if !seen[idx(i.row, i.col)].contains(i.state) {
                            seen[idx(i.row, i.col)].insert(i.state);
                            tmp.push(i);
                        }
                    }
                }
            }
            if tmp.len() == 0 {
                break;
            }
            std::mem::swap(&mut tmp, &mut moving);
            tmp.clear()
        }

        let res = seen.iter().filter(|x| !x.is_empty()).count();

        Ok(Solution::U64(res as u64))
    } else {
        let mut res = 0;
        let mut seen = Vec::new();
        let mut moving = Vec::new();
        let mut tmp = Vec::new();

        for (state, row_start, row_end, col_start, col_end) in &[
            (State::R, 0, h - 1, 0, 0),
            (State::L, 0, h - 1, w - 1, w - 1),
            (State::D, 0, 0, 0, w - 1),
            (State::U, h - 1, h - 1, 0, w - 1),
        ] {
            for row in *row_start..=*row_end {
                for col in *col_start..=*col_end {
                    seen.clear();
                    seen.resize(data.len(), EnumSet::new());
                    moving.clear();
                    moving.push(Light {
                        state: *state,
                        row,
                        col,
                    });
                    seen[idx(row, col)].insert(*state);

                    loop {
                        for light in &moving {
                            for i in step(light) {
                                if let Some(i) = i {
                                    if !seen[idx(i.row, i.col)].contains(i.state) {
                                        seen[idx(i.row, i.col)].insert(i.state);
                                        tmp.push(i);
                                    }
                                }
                            }
                        }
                        if tmp.len() == 0 {
                            break;
                        }
                        std::mem::swap(&mut tmp, &mut moving);
                        tmp.clear()
                    }

                    res = std::cmp::max(seen.iter().filter(|x| !x.is_empty()).count(), res);
                }
            }
        }

        Ok(Solution::U64(res as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn part_1() {
        assert_eq!(solve(1, SAMPLE).unwrap(), Solution::U64(46));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, SAMPLE).unwrap(), Solution::U64(51));
    }
}
