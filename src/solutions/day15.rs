use crate::Solution;
use anyhow::{Context, Result};
use rustc_hash::FxHashMap;

pub fn solve(part: u8, input: &'static str) -> Result<Solution, anyhow::Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/15.input")
    };

    if part == 1 {
        let mut res = 0;
        let mut h = 0;
        let mut c = input.chars();
        loop {
            match c.next() {
                None => {
                    res += h;
                    break;
                }
                Some('\n') => {}
                Some(',') => {
                    res += h;
                    h = 0;
                }
                Some(c) => {
                    debug_assert!(c.is_ascii());
                    h += c as u32;
                    h *= 17;
                    h %= 256;
                }
            }
        }

        Ok(Solution::U64(res as u64))
    } else {
        #[derive(Default)]
        struct Boxe {
            // key: label, value: (counter-at-insert, lens)
            pool: FxHashMap<&'static str, (usize, u8)>,
            counter: usize,
        }
        impl Boxe {
            fn remove(&mut self, lbl: &str) {
                self.pool.remove(lbl);
            }
            fn add(&mut self, lbl: &'static str, lens: &str) {
                let lens = lens.parse::<u8>().unwrap();
                debug_assert!(lens < 10);
                self.pool
                    .entry(lbl)
                    .and_modify(|x| *x = (x.0, lens))
                    .or_insert((self.counter, lens));
                self.counter += 1;
            }
            fn local_focus(&self) -> usize {
                let mut res = 0;
                let mut values = self.pool.values().collect::<Vec<_>>();
                values.sort_by(|a, b| a.0.cmp(&b.0)); // reverse
                for (idx, (_, lens)) in values.iter().enumerate() {
                    res += (idx + 1) * (*lens as usize);
                }
                res
            }
        }
        let mut boxes = Vec::new();
        boxes.resize_with(256, || Boxe::default());

        for tok in input.split(',') {
            let label = tok
                .split_inclusive(&['-', '='])
                .next()
                .context("no label")?;
            let (label, op) = label.split_at(label.len() - 1);

            let mut h = 0;
            for c in label.chars() {
                debug_assert!(c.is_ascii());
                h += c as u32;
                h *= 17;
                h %= 256;
            }
            let h = h;

            let boxe = &mut boxes[h as usize];
            match op {
                "-" => boxe.remove(label),
                "=" => boxe.add(
                    label,
                    tok.split(op).skip(1).next().context("invalid focal")?,
                ),
                _ => unreachable!(),
            };
        }

        let mut res = 0;
        for (idx, boxe) in boxes.iter().enumerate() {
            res += (idx + 1) * boxe.local_focus();
        }

        Ok(Solution::U64(res as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn part_1() {
        assert_eq!(solve(1, SAMPLE).unwrap(), Solution::U64(1320));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, SAMPLE).unwrap(), Solution::U64(145));
    }
}
