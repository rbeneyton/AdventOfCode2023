use crate::Solution;
use anyhow::{Context, Result};
use itertools::Itertools;
use num::integer::lcm;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::BTreeMap;

pub fn solve(part: u8, input: &'static str) -> Result<Solution, anyhow::Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/20.input")
    };

    // {{{ names_to_id

    let mut names = FxHashMap::default();
    for (idx, line) in input.lines().enumerate() {
        let lbl = match line.chars().next().expect("empty line") {
            '%' | '&' => line.split_at(1).1,
            _ => line,
        };
        let (lbl, _) = lbl.split(" -> ").collect_tuple().context("deconstruct")?;
        names.insert(lbl, idx);
    }

    // }}}

    const DEADEND: usize = usize::MAX - 1;
    const RX: usize = usize::MAX;

    #[derive(Debug)]
    enum Action {
        FlipFlop(bool),                     // %, state
        Conjunction(BTreeMap<usize, bool>), // &, key:id, value:last pulse
        BroadCaster,
    };
    #[derive(Debug)]
    struct Mod {
        op: Action,
        to: Vec<usize>,
    }

    // {{{ module parsing

    impl Mod {
        fn new(line: &str, names: &FxHashMap<&str, usize>) -> Self {
            let (op, lbl) = match line.chars().next().expect("empty line") {
                '%' => (Action::FlipFlop(false), line.split_at(1).1),
                '&' => (Action::Conjunction(BTreeMap::new()), line.split_at(1).1),
                _ => (Action::BroadCaster, line),
            };
            let (lbl, to) = lbl.split(" -> ").collect_tuple().expect("deconstruct");
            let to = to
                .split(',')
                .map(|x| x.trim())
                .inspect(|x| {
                    if !names.contains_key(x) {
                        println!("no module '{}'", *x);
                    }
                })
                .map(|x| {
                    *names
                        .get(&x)
                        .unwrap_or_else(|| if x == "rx" { &RX } else { &DEADEND })
                })
                .collect::<Vec<_>>();
            Self { op, to }
        }
    };

    // }}}

    if part == 1 {
        struct Mods {
            mods: Vec<Mod>,
            start: usize,
            // working sandboxes to minimize allocations
            signals: Vec<(usize, usize, bool)>, // from, to, high?
            tmp: Vec<(usize, usize, bool)>,
            acc_signal: (usize, usize),
        };

        impl Mods {
            // {{{ modules parsing

            fn new(input: &str, names: &FxHashMap<&str, usize>) -> Self {
                let mut mods = Vec::with_capacity(names.len());
                let start = *names.get("broadcaster").expect("no broadcaster");
                for line in input.lines() {
                    mods.push(Mod::new(line, &names));
                }
                // initial states for conjunction
                for id in 0..(mods.len()) {
                    // clone due to BC in this case /o\
                    for to in mods[id].to.clone() {
                        if to >= DEADEND {
                            continue;
                        }
                        let mod_to = &mut mods[to];
                        if let Action::Conjunction(ref mut map) = mod_to.op {
                            map.insert(id, false);
                        }
                    }
                }
                Self {
                    mods,
                    start,
                    signals: Vec::new(),
                    tmp: Vec::new(),
                    acc_signal: (0, 0),
                }
            }

            // }}}
            // {{{ act

            fn act(&mut self) {
                self.signals.clear();
                self.tmp.clear();
                // start with an up pulse on broadcaster
                self.signals.push((usize::MAX, self.start, false));
                self.tmp.clear();
                loop {
                    self.acc_signal.0 += self.signals.iter().filter(|(_, _, x)| *x).count();
                    self.acc_signal.1 += self.signals.iter().filter(|(_, _, x)| !*x).count();
                    for (from, id, pulse) in &self.signals {
                        let (from, id, pulse) = (*from, *id, *pulse);
                        if id >= DEADEND {
                            continue;
                        }
                        let m = &mut self.mods[id];
                        match m.op {
                            Action::FlipFlop(ref mut state) => {
                                // If a flip-flop module receives a high pulse, it is ignored and nothing happens.
                                // However, if a flip-flop module receives a low pulse, it flips between on and off.
                                if !pulse {
                                    *state = !*state;
                                    // If it was off, it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse.
                                    self.tmp.extend(m.to.iter().map(|x| (id, *x, *state)));
                                }
                            }
                            Action::Conjunction(ref mut map) => {
                                // When a pulse is received, the conjunction module first updates its memory for that input.
                                map.insert(from, pulse);
                                // Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
                                let pulse = map.iter().all(|(_, x)| *x);
                                self.tmp.extend(m.to.iter().map(|x| (id, *x, !pulse)));
                            }
                            Action::BroadCaster => {
                                self.tmp.extend(m.to.iter().map(|x| (id, *x, pulse)));
                            }
                        };
                    }
                    if self.tmp.len() == 0 {
                        break;
                    }
                    std::mem::swap(&mut self.tmp, &mut self.signals);
                    self.tmp.clear();
                }
            }

            // }}}
        }

        let mut mods = Mods::new(input, &names);
        for turn in 0..1000 {
            mods.act();
        }
        let res = mods.acc_signal.0 * mods.acc_signal.1;

        Ok(Solution::U64(res as u64))
    } else {
        #[derive(Debug)]
        struct Mods {
            turn: usize,
            mods: Vec<Mod>,
            start: usize,
            // working sandboxes to minimize allocations
            signals: Vec<(usize, usize, bool)>, // from, to, high?
            tmp: Vec<(usize, usize, bool)>,
        };

        impl Mods {
            // {{{ modules parsing

            fn new(input: &str, names: &FxHashMap<&str, usize>) -> Self {
                let mut mods = Vec::with_capacity(names.len());
                let start = *names.get("broadcaster").expect("no broadcaster");
                for line in input.lines() {
                    mods.push(Mod::new(line, &names));
                }
                // initial states for conjunction
                for id in 0..(mods.len()) {
                    // clone due to BC in this case /o\
                    for to in mods[id].to.clone() {
                        if to >= DEADEND {
                            continue;
                        }
                        let mod_to = &mut mods[to];
                        if let Action::Conjunction(ref mut map) = mod_to.op {
                            map.insert(id, false);
                        }
                    }
                }
                Self {
                    turn: 0,
                    mods,
                    start,
                    signals: Vec::new(),
                    tmp: Vec::new(),
                }
            }

            // }}}
            // {{{ act

            fn act(&mut self, parents: &mut FxHashMap<usize, Vec<usize>>) {
                self.signals.clear();
                self.tmp.clear();
                // start with an up pulse on broadcaster
                self.signals.push((usize::MAX, self.start, false));
                self.tmp.clear();
                loop {
                    for (from, id, pulse) in &self.signals {
                        let (from, id, pulse) = (*from, *id, *pulse);
                        if parents.contains_key(&id) && !pulse {
                            // println!("turn:{} id:{} pulse:{}", self.turn, id, pulse);
                            parents.entry(id).and_modify(|x| x.push(self.turn));
                        }
                        if id >= DEADEND {
                            continue;
                        }
                        let m = &mut self.mods[id];
                        match m.op {
                            Action::FlipFlop(ref mut state) => {
                                // If a flip-flop module receives a high pulse, it is ignored and nothing happens.
                                // However, if a flip-flop module receives a low pulse, it flips between on and off.
                                if !pulse {
                                    *state = !*state;
                                    // If it was off, it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse.
                                    self.tmp.extend(m.to.iter().map(|x| (id, *x, *state)));
                                }
                            }
                            Action::Conjunction(ref mut map) => {
                                // When a pulse is received, the conjunction module first updates its memory for that input.
                                map.insert(from, pulse);
                                // Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
                                let pulse = map.iter().all(|(_, x)| *x);
                                self.tmp.extend(m.to.iter().map(|x| (id, *x, !pulse)));
                            }
                            Action::BroadCaster => {
                                self.tmp.extend(m.to.iter().map(|x| (id, *x, pulse)));
                            }
                        };
                    }
                    if self.tmp.len() == 0 {
                        break;
                    }
                    std::mem::swap(&mut self.tmp, &mut self.signals);
                    self.tmp.clear();
                }
                self.turn += 1;
            }

            // }}}
        }

        let mut mods = Mods::new(input, &names);
        // find multiple parent of rx (of Conjunction)
        let mut parents = vec![RX];
        loop {
            parents = mods
                .mods
                .iter()
                .enumerate()
                .filter_map(|(idx, x)| {
                    if x.to.contains(&parents[0]) {
                        Some(idx)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            println!(
                "{}",
                itertools::join(parents.iter().map(|x| format!("{}", x)), ",")
            );
            if parents.len() > 1 {
                break;
            }
        }

        // store all nope pulses on these ids
        let mut parents = parents
            .iter()
            .map(|x| (*x, Vec::new()))
            .collect::<FxHashMap<usize, Vec<usize>>>();

        for _ in 0..30000 {
            mods.act(&mut parents);
        }
        dbg!(&parents);
        // extrapolate until convergence using classic lcm
        // when planet will collide ???
        let mut periods = Vec::new();
        for planet in parents.iter().map(|(_, x)| x) {
            // check mono-mode cycle
            let cycles = planet
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect::<FxHashSet<usize>>();
            assert_eq!(cycles.len(), 1);
            periods.push(*cycles.iter().next().unwrap());
        }
        let full_period = periods.iter().fold(1usize, |acc, x| lcm(acc, *x));
        let origin = parents.iter().map(|(_, x)| x[0]).min().unwrap();
        dbg!(&origin, &periods, &full_period);
        let origin = 0; // in fact all state are down at 0 so no need to shift
        let res = origin + full_period;

        Ok(Solution::U64(res as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &'static str = r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const SAMPLE2: &'static str = r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn part_1() {
        assert_eq!(solve(1, SAMPLE1).unwrap(), Solution::U64(32000000));
        assert_eq!(solve(1, SAMPLE2).unwrap(), Solution::U64(11687500));
    }
}
