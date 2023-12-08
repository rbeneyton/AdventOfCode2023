use crate::Solution;
use anyhow::Result;
use itertools::Itertools;
use num::integer::lcm;
use rustc_hash::FxHashMap;

pub fn solve(part: u8, input: &'static str) -> Result<Solution, anyhow::Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/08.input")
    };

    let direction = input.lines().take(1).next().unwrap();
    if part == 1 {
        let directions = direction.chars().cycle();

        let mut map = FxHashMap::default();
        for (idx, base) in input.lines().skip(2).enumerate() {
            let label = base.split(' ').next().unwrap();
            map.insert(label, idx);
        }
        let zzz = *map.get("ZZZ").unwrap();

        let mut bases = Vec::with_capacity(map.len());
        for base in input.lines().skip(2) {
            let (right, left) = base
                .split(&[' ', '=', '(', ',', ')'])
                .skip(1)
                .filter(|x| x.len() > 0)
                .map(|x| *map.get(x).unwrap())
                .collect_tuple()
                .unwrap();
            bases.push((right, left));
        }

        let mut pos = *map.get("AAA").unwrap();
        let mut res = 0;
        for (idx, dir) in directions.enumerate() {
            if pos == zzz {
                res = idx;
                break;
            }
            let next = &bases[pos];
            match dir {
                'L' => pos = next.0,
                'R' => pos = next.1,
                _ => unreachable!(),
            }
        }

        Ok(Solution::U64(res as u64))
    } else {
        let mut map = FxHashMap::default();
        let mut veca = Vec::new();
        for (idx, base) in input.lines().skip(2).enumerate() {
            let label = base.split(' ').next().unwrap();
            map.insert(label, idx);
            if label.chars().rev().next() == Some('A') {
                veca.push(idx);
            }
        }

        let mut bases = Vec::with_capacity(map.len());
        for base in input.lines().skip(2) {
            let label = base.split(' ').next().unwrap();
            let z = label.chars().rev().next() == Some('Z');
            let (right, left) = base
                .split(&[' ', '=', '(', ',', ')'])
                .skip(1)
                .filter(|x| x.len() > 0)
                .map(|x| *map.get(x).unwrap())
                .collect_tuple()
                .unwrap();
            bases.push((z, right, left));
        }

        assert_eq!(veca.len(), bases.iter().filter(|x| x.0).count());

        // there are cycles in the data (not suggested by text), so we use lcm on part 1 solutions
        // to find where they align themselves.
        let mut res = 1;
        for a in veca.iter_mut() {
            let directions = direction.chars().cycle();
            for (idx, dir) in directions.enumerate() {
                let next = &bases[*a];
                match dir {
                    'L' => *a = next.1,
                    'R' => *a = next.2,
                    _ => unreachable!(),
                }
                if next.0 {
                    res = lcm(res, idx);
                    break;
                }
            }
        }

        Ok(Solution::U64(res as u64))
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
                r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            )
            .unwrap(),
            Solution::U64(6)
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            solve(
                2,
                r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            )
            .unwrap(),
            Solution::U64(6)
        );
    }
}
