pub use crate::{Day, Solution};
use anyhow::{bail, Error, Result};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
// pub mod day08;
// pub mod day09;
// pub mod day10;
// pub mod day11;
// pub mod day12;
// pub mod day13;
// pub mod day14;
// pub mod day15;
// pub mod day16;
// pub mod day17;
// pub mod day18;
// pub mod day19;
// pub mod day20;
// pub mod day21;
// pub mod day22;
// pub mod day23;
// pub mod day24;
// pub mod day25;

pub fn solve(day: Day, part: u8, input: &'static str) -> Result<Solution, Error> {
    match day {
        1 => day01::solve(part, input),
        2 => day02::solve(part, input),
        3 => day03::solve(part, input),
        4 => day04::solve(part, input),
        5 => day05::solve(part, input),
        6 => day06::solve(part, input),
        7 => day07::solve(part, input),
        // 8 => day08::solve(part, input),
        // 9 => day09::solve(part, input),
        // 10 => day10::solve(part, input),
        // 11 => day11::solve(part, input),
        // 12 => day12::solve(part, input),
        // 13 => day13::solve(part, input),
        // 14 => day14::solve(part, input),
        // 15 => day15::solve(part, input),
        // 16 => day16::solve(part, input),
        // 17 => day17::solve(part, input),
        // 18 => day18::solve(part, input),
        // 19 => day19::solve(part, input),
        // 20 => day20::solve(part, input),
        // 21 => day21::solve(part, input),
        // 22 => day22::solve(part, input),
        // 23 => day23::solve(part, input),
        // 24 => day24::solve(part, input),
        // 25 => day25::solve(part, input),
        _ => bail!("day no yet implemented"),
    }
}
