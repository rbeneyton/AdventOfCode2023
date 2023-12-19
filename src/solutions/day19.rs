use crate::Solution;
use anyhow::{Context, Result};
use itertools::Itertools;
use rustc_hash::FxHashMap;

pub fn solve(part: u8, input: &'static str) -> Result<Solution, anyhow::Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/19.input")
    };

    // {{{ rules name_to_id

    let mut rule_names = FxHashMap::default();
    for (idx, line) in input.lines().enumerate() {
        if line.len() == 0 {
            break;
        }

        let (lbl, _) = line.split_once('{').context("no rule name")?;
        rule_names.insert(lbl, idx);
    }
    let start = *rule_names.get("in").context("no start rule named 'in'")?;

    // }}}

    enum Action {
        Rule(usize), // next rule index
        A,
        R,
    };
    type Num = u16;
    struct Op {
        what: u8, // 0..4
        less: bool,
        than: Num,
    }
    struct Rule {
        op: Option<Op>,
        to: Action,
    }

    // {{{ Rule parsing

    impl Rule {
        fn new(workflow: &str, rule_names: &FxHashMap<&str, usize>) -> Self {
            let (op, to) = if workflow.contains(':') {
                let (op, to) = workflow.split_once(':').expect("invalid workflow");
                let (what, than) = op
                    .split(&['<', '>'])
                    .collect_tuple()
                    .expect("invalid workflow");
                let what = match what.chars().next().expect("xmas") {
                    'x' => 0,
                    'm' => 1,
                    'a' => 2,
                    's' => 3,
                    _ => unreachable!(),
                };
                let less = op.contains('<'); // else >
                let than = than.parse::<Num>().expect("invalid 'than' value");
                (Some(Op { what, less, than }), to)
            } else {
                (None, workflow)
            };
            let to = match to {
                "A" => Action::A,
                "R" => Action::R,
                _ => Action::Rule(*rule_names.get(to).expect("unknown 'to' value")),
            };

            Self { op, to }
        }
    };

    // }}}
    // {{{ workflow parsing

    let mut workflows = Vec::with_capacity(rule_names.len());
    let mut linei = input.lines();
    loop {
        let line = linei.next().context("no empty line")?;
        if line.len() == 0 {
            break;
        }

        let (_, workflow, _) = line
            .split(&['{', '}'])
            .collect_tuple()
            .context("invalid line")?;
        workflows.push(
            workflow
                .split(',')
                .map(|x| Rule::new(x, &rule_names))
                .collect::<Vec<_>>(),
        );
    }

    // }}}

    if part == 1 {
        struct Part([Num; 4]); // xmas

        fn accepted(workflows: &Vec<Vec<Rule>>, start: usize, part: &Part) -> bool {
            let mut i = start;
            'workflow: loop {
                'rule: for rule in &workflows[i] {
                    if let Some(op) = &rule.op {
                        let what = part.0[op.what as usize];
                        let pass = if op.less {
                            what < op.than
                        } else {
                            what > op.than
                        };
                        if !pass {
                            continue 'rule;
                        }
                    }
                    return match rule.to {
                        Action::Rule(to) => {
                            i = to;
                            continue 'workflow;
                        } // next rule index
                        Action::A => true,
                        Action::R => false,
                    };
                }
            }
        }

        // scan each parts
        let mut res = 0;
        loop {
            let line = linei.next();
            if line.is_none() {
                break;
            }

            let (_, data, _) = line
                .unwrap()
                .split(&['{', '}'])
                .collect_tuple()
                .context("invalid line")?;
            let (x, m, a, s) = data
                .split(',')
                .map(|x| {
                    x.split('=')
                        .skip(1)
                        .next()
                        .expect("no =")
                        .parse::<Num>()
                        .expect("parse error")
                })
                .collect_tuple()
                .context("no x/m/a/s values")?;
            let part = Part([x, m, a, s]);
            if accepted(&workflows, start, &part) {
                res += part.0.iter().map(|x| *x as u64).sum::<u64>();
            }
        }

        Ok(Solution::U64(res))
    } else {
        #[derive(Default, Clone, Copy)]
        struct Range {
            ranges: [(Num, Num); 4], // (min, max) inclusive of x/m/a/s
            wi: usize,               // workflow index
            ri: usize,               // rule index
        };
        let mut states = vec![Range {
            ranges: [(1, 4000); 4],
            wi: start,
            ..Default::default()
        }];
        let mut tmp = Vec::new();
        let mut accepted = Vec::new();

        loop {
            for state in &states {
                let workflow = &workflows[state.wi];
                let rule = &workflow[state.ri];
                let (mut pass, mut nope) = (*state, *state);
                if let Some(op) = &rule.op {
                    let what = op.what as usize;
                    let (min, max) = state.ranges[what];

                    if min < op.than && op.than < max {
                        // split case
                        if op.less {
                            pass.ranges[what].1 = op.than - 1;
                            nope.ranges[what].0 = op.than;
                        } else {
                            pass.ranges[what].0 = op.than + 1;
                            nope.ranges[what].1 = op.than;
                        }
                        nope.ri += 1;
                        tmp.push(nope);
                    }
                }
                match rule.to {
                    Action::Rule(to) => tmp.push(Range {
                        wi: to,
                        ri: 0,
                        ranges: pass.ranges,
                    }),
                    Action::A => accepted.push(pass),
                    Action::R => (),
                };
            }
            if tmp.len() == 0 {
                break;
            }
            std::mem::swap(&mut tmp, &mut states);
            tmp.clear();
        }

        // no overlapping by design (\o/)
        let mut res = accepted
            .iter()
            .map(|x| {
                x.ranges
                    .iter()
                    .map(|x| (x.1 - x.0 + 1) as u64)
                    .product::<u64>()
            })
            .sum();

        Ok(Solution::U64(res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn part_1() {
        assert_eq!(solve(1, SAMPLE).unwrap(), Solution::U64(19114));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, SAMPLE).unwrap(), Solution::U64(167409079868000));
    }
}
