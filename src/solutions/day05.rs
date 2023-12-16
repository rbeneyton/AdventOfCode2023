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

    // {{{ parsing into vec of (key: src, value: range)

    #[derive(Clone, Copy)]
    struct Range {
        start: i64,
        end: i64, // non inclusive
        dst: i64,
    }

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

        let (dst, start, range) = line
            .split(' ')
            .map(|x| x.parse::<i64>().expect("Map::new parse error"))
            .collect_tuple()
            .expect("no 3 token");
        let end = start + range;
        map.insert(start, Range { start, end, dst });
    }

    // }}}
    // {{{ check non-overlapping map

    for map in &maps {
        for (a, b) in map.values().tuple_windows() {
            debug_assert!(a.end <= b.start, "overlapping map!");
        }
    }

    // }}}
    // {{{ fill identity ranges to get contiguous map to avoid all crappy cases

    let mut maps2 = Vec::new();
    for map in &maps {
        let mut map2 = BTreeMap::new();
        let first = map.iter().next().unwrap().1;
        if first.start != 0 {
            map2.insert(
                0,
                Range {
                    start: 0,
                    end: first.start,
                    dst: 0,
                },
            );
        }
        for (a, b) in map.values().tuple_windows() {
            map2.insert(a.start, *a);
            if a.end != b.start {
                map2.insert(
                    a.end,
                    Range {
                        start: a.end,
                        end: b.start,
                        dst: a.end,
                    },
                );
            }
        }
        let last = map.iter().rev().next().unwrap().1;
        map2.insert(last.start, *last);
        if last.end != i64::MAX {
            map2.insert(
                last.end,
                Range {
                    start: last.end,
                    end: i64::MAX,
                    dst: last.end,
                },
            );
        }
        maps2.push(map2);
    }
    let maps = maps2;

    // }}}
    // {{{ check contiguous map

    for map in &maps {
        for (a, b) in map.values().tuple_windows() {
            debug_assert!(a.end == b.start, "overlapping map!");
        }
        debug_assert_eq!(
            map.iter().next().unwrap().1.start,
            0,
            "map doesn't start at 0!"
        );
        debug_assert_eq!(
            map.iter().rev().next().unwrap().1.end,
            i64::MAX,
            "map doesn't end at i64::MAX!"
        );
    }

    // }}}

    if part == 1 {
        let do_map = |seed: i64| {
            let mut stuff = seed;
            for map in &maps {
                let (start, range) = map.range(..=stuff).rev().next().expect("no initial range");
                assert!(*start <= stuff);
                let off = stuff - range.start;
                if off >= 0 && off < range.end - range.start {
                    stuff = range.dst + off;
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
        struct Span {
            start: i64,
            end: i64, // non inclusive
        }
        let do_map = |start: i64, depth: i64| {
            let end = start + depth;
            let mut spans = vec![Span { start, end }];
            let mut tmp = Vec::new();
            let mut work = Vec::<Range>::new();
            for map in &maps {
                for span in &spans {
                    work.clear();

                    // initial range: lower bound or manually crafted
                    let (start, range) = map
                        .range(..=span.start)
                        .rev()
                        .next()
                        .expect("no initial range");
                    if *start < span.start {
                        let off = span.start - range.start;
                        work.push(Range {
                            start: span.start,
                            end: std::cmp::min(span.end, range.end),
                            dst: range.dst + off,
                        });
                    }
                    // process all remaining ranges
                    for (start, range) in map.range((span.start)..(span.end)) {
                        let start = *start;
                        let end = std::cmp::min(span.end, range.end);
                        let dst = range.dst;
                        work.push(Range { start, end, dst });
                    }
                    // check contiguity
                    for (a, b) in work.iter().tuple_windows() {
                        debug_assert_eq!(a.end, b.start);
                    }

                    // do the destination mapping operation and append onto our current results
                    tmp.extend(work.iter().map(|range| {
                        let start = range.dst;
                        let end = range.dst + range.end - range.start;
                        Span { start, end }
                    }));
                }

                tmp.sort_by(|a, b| a.start.cmp(&b.start));
                // check non overlapping
                for (a, b) in tmp.iter().tuple_windows() {
                    debug_assert!(a.end <= b.start, "overlapping tmp!");
                }
                // TODO optimization: coalescing

                std::mem::swap(&mut tmp, &mut spans);
                tmp.clear();
            }
            // return the min
            spans[0].start
        };

        // seeds are pairs
        let res = seeds
            .iter()
            .tuples()
            .map(|(seed, length)| do_map(*seed, *length))
            .min()
            .unwrap();

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
