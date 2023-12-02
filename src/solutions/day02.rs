use crate::Solution;
use anyhow::{Context, Result};

pub fn solve(part: u8, input: &'static str) -> Result<Solution, anyhow::Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/02.input")
    };

    if part == 1 {
        let mut res = 0;
        for line in input.lines() {
            // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            let line = line.strip_prefix("Game ").context("no prefix 'Game '")?;
            let (id, line) = line.split_once(':').context("no :")?;
            let id = id
                .parse::<usize>()
                .with_context(|| format!("cannot get game id in '{id}'"))?;
            let mut valid = true;
            'game: for turn in line.split(";") {
                for color in turn.split(",") {
                    let color = color.trim_start();
                    let (n, color) = color
                        .split_once(" ")
                        .with_context(|| format!("no space in color token {color}"))?;
                    let n = n
                        .parse::<usize>()
                        .with_context(|| format!("cannot get n in '{color}'"))?;
                    if (color.contains("red") && n > 12)
                        || (color.contains("green") && n > 13)
                        || (color.contains("blue") && n > 14)
                    {
                        valid = false;
                        break 'game;
                    }
                }
            }
            if valid {
                res += id;
            }
        }
        Ok(Solution::USIZE(res))
    } else {
        let mut res = 0;
        for line in input.lines() {
            // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            let line = line.strip_prefix("Game ").context("no prefix 'Game '")?;
            let (_, line) = line.split_once(':').context("no :")?;
            let (mut red, mut green, mut blue) = (0, 0, 0);
            'game: for turn in line.split(";") {
                for color in turn.split(",") {
                    let color = color.trim_start();
                    let (n, color) = color
                        .split_once(" ")
                        .with_context(|| format!("no space in color token {color}"))?;
                    let n = n
                        .parse::<usize>()
                        .with_context(|| format!("cannot get n in '{color}'"))?;
                    if color.contains("red") {
                        red = std::cmp::max(red, n);
                    } else {
                        if color.contains("green") {
                            green = std::cmp::max(green, n);
                        } else {
                            if color.contains("blue") {
                                blue = std::cmp::max(blue, n);
                            }
                        }
                    }
                }
            }
            let power = red * green * blue;
            res += power;
        }
        Ok(Solution::USIZE(res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part_1() {
        assert_eq!(solve(1, SAMPLE).unwrap(), Solution::USIZE(8));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, SAMPLE).unwrap(), Solution::USIZE(2286));
    }
}
