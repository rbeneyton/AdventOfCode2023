use crate::Solution;
use anyhow::{Context, Result};
use rustc_hash::{FxHashMap, FxHashSet};

pub fn solve(part: u8, input: &'static str) -> Result<Solution, anyhow::Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/25.input")
    };

    let mut names = FxHashSet::default();
    for line in input.lines() {
        let (node, tos) = line.split_once(':').expect("empty line");
        let node = node.trim();
        names.insert(node);
        for to in tos.split(' ').map(|x| x.trim()) {
            if to.len() > 0 {
                names.insert(to);
            }
        }
    }
    let n = names.len();
    let names = names
        .iter()
        .enumerate()
        .map(|(idx, x)| (*x, idx))
        .collect::<FxHashMap<&str, usize>>();
    // dbg!(&names);

    let mut nodes = FxHashMap::<usize, Vec<usize>>::default();
    for line in input.lines() {
        let (node, tos) = line.split_once(':').expect("empty line");
        let node = node.trim();
        let node = *names.get(&node).expect("inconcistency");
        for to in tos.split(' ').map(|x| x.trim()) {
            if to.len() > 0 {
                let to = *names.get(&to).expect("inconcistency");
                nodes
                    .entry(node)
                    .and_modify(|x| {
                        if !x.contains(&to) {
                            x.push(to);
                        }
                    })
                    .or_insert(vec![to]);
                // add also reverse link
                nodes
                    .entry(to)
                    .and_modify(|x| {
                        if !x.contains(&node) {
                            x.push(node);
                        }
                    })
                    .or_insert(vec![node]);
            }
        }
    }

    let mut links = FxHashMap::<(usize, usize), usize>::default();

    let mut dists = Vec::new();
    let mut froms = Vec::new();
    let mut tmp = Vec::new();

    for from in 0..n {
        dists.clear();
        froms.clear();
        tmp.clear();

        dists.resize(n, 0);
        froms.push(from);
        dists[from] = 1;

        // bfs
        loop {
            for from in &froms {
                for dst in nodes.get(from).unwrap() {
                    if dists[*dst] == 0 {
                        dists[*dst] = dists[*from] + 1;
                        tmp.push(*dst);
                    }
                }
            }
            if tmp.len() == 0 {
                break;
            }
            std::mem::swap(&mut tmp, &mut froms);
            tmp.clear();
        }
        // paths
        for to in (0..n).filter(|x| *x != from) {
            let mut step = to;
            loop {
                for dst in nodes.get(&step).unwrap() {
                    let dst = *dst;
                    if dists[dst] == dists[step] - 1 {
                        let (min, max) = (std::cmp::min(step, dst), std::cmp::max(step, dst));
                        links
                            .entry((min, max))
                            .and_modify(|x| *x = *x + 1)
                            .or_insert(1);
                        step = dst;
                        break;
                    }
                }
                if step == from {
                    break;
                }
            }
        }
    }

    // extract most used steps
    let m1 = links.iter().max_by(|a, b| a.1.cmp(&b.1)).expect("no links");
    let n1 = m1.0;
    let names1 = names
        .iter()
        .filter(|(_, v)| **v == n1.0 || **v == n1.1)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    //dbg!(m1, names1);
    let m2 = links
        .iter()
        .filter(|x| *x != m1)
        .max_by(|a, b| a.1.cmp(&b.1))
        .expect("no links");
    let n2 = m2.0;
    let names2 = names
        .iter()
        .filter(|(_, v)| **v == n2.0 || **v == n2.1)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    //dbg!(m2, names2);
    let m3 = links
        .iter()
        .filter(|x| *x != m1 && *x != m2)
        .max_by(|a, b| a.1.cmp(&b.1))
        .expect("no links");
    let n3 = m3.0;
    let names3 = names
        .iter()
        .filter(|(_, v)| **v == n3.0 || **v == n3.1)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    //dbg!(m3, names3);

    // fitered nodes
    let nodes = nodes
        .iter()
        .map(|(from, tos)| {
            if *from == n1.0 {
                return (
                    *from,
                    tos.iter()
                        .cloned()
                        .filter(|x| *x != n1.1)
                        .collect::<Vec<_>>(),
                );
            }
            if *from == n1.1 {
                return (
                    *from,
                    tos.iter()
                        .cloned()
                        .filter(|x| *x != n1.0)
                        .collect::<Vec<_>>(),
                );
            }
            if *from == n2.0 {
                return (
                    *from,
                    tos.iter()
                        .cloned()
                        .filter(|x| *x != n2.1)
                        .collect::<Vec<_>>(),
                );
            }
            if *from == n2.1 {
                return (
                    *from,
                    tos.iter()
                        .cloned()
                        .filter(|x| *x != n2.0)
                        .collect::<Vec<_>>(),
                );
            }
            if *from == n3.0 {
                return (
                    *from,
                    tos.iter()
                        .cloned()
                        .filter(|x| *x != n3.1)
                        .collect::<Vec<_>>(),
                );
            }
            if *from == n3.1 {
                return (
                    *from,
                    tos.iter()
                        .cloned()
                        .filter(|x| *x != n3.0)
                        .collect::<Vec<_>>(),
                );
            }
            (*from, tos.iter().cloned().collect::<Vec<_>>())
        })
        .collect::<FxHashMap<usize, Vec<usize>>>();

    // build clusters
    let mut clusters = FxHashSet::default();
    let mut dists = Vec::new();
    for from in 0..n {
        dists.clear();
        froms.clear();
        tmp.clear();

        dists.resize(n, false);
        froms.push(from);
        dists[from] = true;

        // bfs
        loop {
            for from in &froms {
                for dst in nodes.get(from).unwrap() {
                    if !dists[*dst] {
                        dists[*dst] = true;
                        tmp.push(*dst);
                    }
                }
            }
            if tmp.len() == 0 {
                break;
            }
            std::mem::swap(&mut tmp, &mut froms);
            tmp.clear();
        }

        // compute reachable cluster size
        let cluster = dists.iter().filter(|x| **x).count();
        clusters.insert(cluster);
    }
    debug_assert_eq!(clusters.len(), 2);

    let res = clusters.iter().product::<usize>();

    if part == 1 {
        Ok(Solution::U64(res as u64))
    } else {
        Ok(Solution::U64(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = r"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn part_1() {
        assert_eq!(solve(1, SAMPLE).unwrap(), Solution::U64(54));
    }
}
