use std::collections::{HashMap, HashSet};

use quest_helper::register;

fn reach<'s>(dst: &'s str, graph: &HashMap<&'s str, Vec<&'s str>>) -> HashSet<&'s str> {
    let mut seen = HashSet::from([dst]);
    let mut queue = vec![dst];

    while let Some(dst) = queue.pop() {
        if let Some(srcs) = graph.get(&dst) {
            queue.extend(srcs.iter().copied().filter(|&src| seen.insert(src)));
        }
    }

    seen
}

#[register]
pub fn part1(input: &str) -> u64 {
    let mut recepies = HashMap::new();

    for line in input.lines() {
        let (dst, srcs) = line.split_once(':').unwrap();
        let srcs = srcs.split(',').collect::<Vec<_>>();
        recepies.insert(dst, srcs);
    }

    reach("NULL", &recepies).len() as u64 - 1
}

#[register]
pub fn part2(input: &str) -> u64 {
    let mut recepies = HashMap::new();

    for line in input.lines() {
        let (src, dsts) = line.split_once(':').unwrap();
        for dst in dsts.split(',') {
            recepies.entry(dst).or_insert_with(Vec::new).push(src);
        }
    }

    reach("NULL", &recepies).len() as u64 - 1
}

#[register]
pub fn part3(input: &str) -> u64 {
    let mut recepies = HashMap::new();

    for line in input.lines() {
        let (src, dsts) = line.split_once(':').unwrap();
        for dst in dsts.split(',') {
            recepies.entry(dst).or_insert_with(Vec::new).push(src);
        }
    }

    let mut reach1 = reach("NULLv1", &recepies);
    let mut reach2 = reach("NULLv2", &recepies);
    let mut reach3 = reach("NULLv3", &recepies);

    for k in ["NULLv1", "NULLv2", "NULLv3"] {
        for r in [&mut reach1, &mut reach2, &mut reach3] {
            r.remove(&k);
        }
    }

    let reach12 = reach1.intersection(&reach2).count();
    let reach13 = reach1.intersection(&reach3).count();
    let reach23 = reach2.intersection(&reach3).count();
    let reach123 = reach1
        .intersection(&reach2)
        .filter(|&k| reach3.contains(k))
        .count();

    (reach12 + reach13 + reach23 - 2 * reach123) as u64
}
