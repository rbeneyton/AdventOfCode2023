use crate::Solution;
use anyhow::{Context, Result};

pub fn solve(part: u8, input: &'static str) -> Result<Solution, anyhow::Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/10.input")
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

    let start = data
        .iter()
        .enumerate()
        .filter_map(|(idx, x)| if *x == 'S' { Some(idx) } else { None })
        .next()
        .context("no S found in data")? as isize;
    let (start_row, start_col) = (start / w, start % w);

    // weird char () to allowed direction (north/west/south/east)
    let dir = |c| match c {
        '|' => [true, false, true, false], // is a vertical pipe connecting north and south.
        '-' => [false, true, false, true], // is a horizontal pipe connecting east and west.
        'L' => [true, false, false, true], // is a 90-degree bend connecting north and east.
        'J' => [true, true, false, false], // is a 90-degree bend connecting north and west.
        '7' => [false, true, true, false], // is a 90-degree bend connecting south and west.
        'F' => [false, false, true, true], // is a 90-degree bend connecting south and east.
        '.' => panic!("reach a ground mark"), // is ground; there is no pipe in this tile.
        'S' => [false, false, false, false], // is the starting position of the animal
        _ => unreachable!(),
    };
    // reverse direction
    let rev = |x| (x + 2) % 4;
    // movement
    let move_op = |dir, row, col| match dir {
        0 => (row - 1, col), // north
        1 => (row, col - 1), // west
        2 => (row + 1, col), // south
        3 => (row, col + 1), // east
        _ => unreachable!(),
    };

    // find pipe shape under S
    let mut s = [false; 4];
    for direction in 0..4 {
        let (row, col) = move_op(direction, start_row, start_col);
        if valid(row, col) {
            let c = data[idx(row, col)];
            if c == '.' {
                continue;
            }
            let pipe = dir(data[idx(row, col)]);
            if pipe[rev(direction)] {
                s[direction] = true;
            }
        }
    }
    let s = s;

    if part == 1 {
        let (mut row, mut col) = (start_row, start_col);
        let mut depth = 0;
        let mut direction = s
            .iter()
            .enumerate()
            .filter_map(|(idx, x)| if *x { Some(idx) } else { None })
            .next()
            .context("no initial direction")?;
        'path: loop {
            (row, col) = move_op(direction, row, col);
            if row == start_row && col == start_col {
                break;
            }
            let rev_direction = rev(direction);
            assert!(valid(row, col));
            direction = dir(data[idx(row, col)])
                .iter()
                .enumerate()
                .filter_map(|(idx, x)| if *x { Some(idx) } else { None })
                .filter(|x| *x != rev_direction)
                .next()
                .context("dead end!")?;
            depth += 1;
        }
        let res = (depth + 1) / 2;

        Ok(Solution::U64(res))
    } else {
        let mut walls = Vec::new();
        walls.resize(data.len(), false);

        let (mut row, mut col) = (start_row, start_col);
        let mut direction = s
            .iter()
            .enumerate()
            .filter_map(|(idx, x)| if *x { Some(idx) } else { None })
            .next()
            .context("no initial direction")?;
        'path: loop {
            (row, col) = move_op(direction, row, col);
            walls[idx(row, col)] = true;
            if row == start_row && col == start_col {
                break;
            }
            let rev_direction = rev(direction);
            assert!(valid(row, col));
            direction = dir(data[idx(row, col)])
                .iter()
                .enumerate()
                .filter_map(|(idx, x)| if *x { Some(idx) } else { None })
                .filter(|x| *x != rev_direction)
                .next()
                .context("dead end!")?;
        }

        // build walls in 3x3 grid
        // 0:empty, 1:wall
        let (w3, h3) = (w * 3, h * 3);
        let mut grid3 = Vec::new();
        grid3.resize(data.len() * 9, 0);
        let idx3 = |row, col| (row * w3 + col) as usize;
        let valid3 = |row, col| row >= 0 && row < h3 && col >= 0 && col < w3;
        for row in 0..h {
            for col in 0..w {
                // center
                if walls[idx(row, col)] {
                    let shape = data[idx(row, col)];
                    let direction = if shape == 'S' { s } else { dir(shape) };
                    let (row, col) = (3 * row + 1, 3 * col + 1);
                    grid3[idx3(row, col)] = 1;
                    if direction[0] {
                        grid3[idx3(row - 1, col)] = 1;
                    }
                    if direction[1] {
                        grid3[idx3(row, col - 1)] = 1;
                    }
                    if direction[2] {
                        grid3[idx3(row + 1, col)] = 1;
                    }
                    if direction[3] {
                        grid3[idx3(row, col + 1)] = 1;
                    }
                }
            }
        }

        // bfs to extend all from top left corner (outside of walls by design)
        // 0:empty, 1:wall, 2:outer-zone
        let mut front = Vec::new();
        grid3[idx3(0, 0)] = 2;
        front.push((0, 0));
        let mut tmp = Vec::new();
        loop {
            for (row, col) in &front {
                for (drow, dcol) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let (row, col) = (row + drow, col + dcol);
                    if valid3(row, col) && grid3[idx3(row, col)] == 0 {
                        grid3[idx3(row, col)] = 2;
                        tmp.push((row, col));
                    }
                }
            }
            front.clear();
            std::mem::swap(&mut front, &mut tmp);
            if front.len() == 0 {
                break;
            }
        }

        let mut res = 0;
        for row in 0..h {
            for col in 0..w {
                let (row, col) = (3 * row + 1, 3 * col + 1);
                res += (grid3[idx3(row, col)] == 0) as u64;
            }
        }

        Ok(Solution::U64(res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(
            solve(
                1,
                r".....
.S-7.
.|.|.
.L-J.
....."
            )
            .unwrap(),
            Solution::U64(4)
        );
        assert_eq!(
            solve(
                1,
                r"-....
.S-7.
.|.|.
.L-J.
....."
            )
            .unwrap(),
            Solution::U64(4)
        );
        assert_eq!(
            solve(
                1,
                r".....
|S-7.
.|.|.
.L-J.
....."
            )
            .unwrap(),
            Solution::U64(4)
        );
        assert_eq!(
            solve(
                1,
                r"....
S-7.
|.|.
L-J.
...."
            )
            .unwrap(),
            Solution::U64(4)
        );
        assert_eq!(
            solve(
                1,
                r"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
            )
            .unwrap(),
            Solution::U64(8)
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            solve(
                2,
                "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
            )
            .unwrap(),
            Solution::U64(4)
        );
        assert_eq!(
            solve(
                2,
                "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."
            )
            .unwrap(),
            Solution::U64(4)
        );
        assert_eq!(
            solve(
                2,
                ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
            )
            .unwrap(),
            Solution::U64(8)
        );
    }
}
