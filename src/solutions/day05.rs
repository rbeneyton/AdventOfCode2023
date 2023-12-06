use crate::Solution;
use anyhow::{Context, Result};
use itertools::Itertools;
use std::collections::BTreeMap;

pub fn solve(part: u8, input: &'static str) -> Result<Solution, anyhow::Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/05.input")
    };

    let mut lines = input.lines();
    let seeds = lines
        .next()
        .context("no seeds in 1st line")?
        .split(' ')
        .skip(1)
        .map(|x| x.parse::<i64>().expect("seed parse error"))
        .collect::<Vec<_>>();

    assert_eq!(lines.next().context("blank line")?.len(), 0);

    // vec of key: src value: range, dst
    let mut maps = Vec::new();
    for line in lines {
        if line.contains("map:") {
            maps.push(BTreeMap::new());
            continue;
        }
        if line.len() == 0 {
            continue;
        }

        let map = maps.last_mut().context("empty maps")?;

        let (dst, src, range) = line
            .split(' ')
            .map(|x| x.parse::<i64>().expect("Map::new parse error"))
            .collect_tuple()
            .expect("no 3 token");
        map.insert(src, (dst, range));

        // check non-overlapping map
        // if let Some((prev, (prev_range, _))) = map.range(..src).rev().next() {
        //     debug_assert!(prev + prev_range > src, "overlapping map: prev:{prev}+{prev_range} on src:{src}!");
        // }
        // debug_assert_eq!(maps.iter().filter(|x| x.get(&0).is_some()).collect::<Vec<_>>().len(), 1);
    }

    if part == 1 {
        let do_map = |seed| {
            let mut stuff = seed;
            for map in &maps {
                if let Some((src, (dst, range))) = map.range(..=stuff).rev().next() {
                    let off = stuff - src;
                    if off >= 0 && off < *range {
                        stuff = *dst + off
                    }
                } else {
                    // untouched value
                }
            }
            stuff
        };

        let res = seeds
            .iter()
            .map(|x| do_map(*x))
            .min()
            .context("no seeds?")? as u64;
        Ok(Solution::U64(res))
    } else {
        let do_map_and_min = |seed: i64, length: i64| {
            let mut stuffs = vec![(seed, length)];
            let mut tmp = Vec::new();
            for map in &maps {
                let mut map = map.clone();
                for (mut stuff, mut length) in &stuffs {
                    dbg!(&map);
                    let end = stuff + length;
                    // split at 'stuff'
                    dbg!(&stuff, length);
                    if let Some((src, (dst, range))) = map.range(..=stuff).rev().next() {
                        let (src, (dst, range)) = (*src, (*dst, *range));
                        if src != stuff {
                            debug_assert!(src < stuff);
                            let off = stuff - src;
                            if range > off {
                                map.get_mut(&src).unwrap().1 = off;
                                map.insert(stuff, (dst + off, range - off));
                                dbg!(&map);
                            }
                        }
                    }

                    for (mut src, (mut dst, mut range)) in map.range(stuff..end) {
                        dbg!(src, range, dst);
                        debug_assert!(*src >= stuff);
                        // if *src < *stuff {
                        //     let off = *stuff - *src;
                        //     src = stuff;
                        //     range -= off;
                        //     dst -= off;
                        // }
                        if *src + range >= end {
                            let off = end - range - *src;
                            range -= off;
                        }
                        if length < range {
                            let off = range - length;
                            range -= off;
                        }
                        if length > range {
                            let off = length - range;
                            length -= off;
                        }
                        tmp.push((dst, range));
                        length -= range;
                        stuff += range;
                    }
                    if length > 0 {
                        // untouched
                        tmp.push((stuff + length, length));
                    }
                }
                tmp.sort_by(|a, b| a.0.cmp(&b.0)); // reverse
                std::mem::swap(&mut tmp, &mut stuffs);
                dbg!(&stuffs);
                tmp.clear();
            }
            stuffs.iter().map(|(x, _)| *x).min().expect("no ranges")
        };
        // seeds are pairs
        let res = seeds
            .iter()
            .tuples()
            .map(|(seed, length)| do_map_and_min(*seed, *length))
            .min()
            .unwrap();
        dbg!(res);

        Ok(Solution::U64(res as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part_1() {
        assert_eq!(solve(1, SAMPLE).unwrap(), Solution::U64(35));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, SAMPLE).unwrap(), Solution::U64(46));
    }
}
