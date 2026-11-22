use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use quest_helper::register;

fn parse(input: &str) -> impl Iterator<Item = (usize, u64, u64)> {
    input.lines().map(|line| {
        let rest = line.strip_prefix("id=").unwrap();
        let (id, rest) = rest.split_once(", growth=").unwrap();
        let (growth, limit) = rest.split_once(", limit=").unwrap();

        let id = id.parse::<usize>().unwrap();
        let growth = growth.parse::<u64>().unwrap();
        let limit = limit.parse::<u64>().unwrap();

        (id, growth, limit)
    })
}

fn solve_leaves(growth: u64, limit: u64) -> u64 {
    let mut next = HashMap::from([(growth, (1, 0))]);
    let mut queue = BinaryHeap::from([Reverse(growth)]);
    let mut count = 1;

    if count >= limit {
        return 1;
    }

    while let Some(Reverse(i)) = queue.pop() {
        let (leaves_splitted, branches_splitted) = next.remove(&i).unwrap();
        count += leaves_splitted + branches_splitted;

        if count >= limit {
            return i;
        }

        let mut new_entry = |j: u64| {
            queue.push(Reverse(j));
            (0, 0)
        };

        let j = i + growth - 1;
        let (leaves, _) = next.entry(j).or_insert_with(|| new_entry(j));
        *leaves += leaves_splitted + branches_splitted;

        let j = i + growth - growth / 2;
        let (leaves, branches) = next.entry(j).or_insert_with(|| new_entry(j));
        *leaves += leaves_splitted;
        *branches += leaves_splitted + 2 * branches_splitted;
    }

    unreachable!();
}

#[register]
pub fn part1(input: &str) -> u64 {
    let mut tot = 0;

    for (_, growth, limit) in parse(input) {
        let mut i = 1;
        let mut count = 1;
        let mut counts = vec![0; growth as usize - 1];
        counts[0] = 1;

        while count < limit {
            count += counts.iter().copied().sum::<u64>();

            counts.rotate_right(1);
            let splitted = counts[0];
            counts[growth as usize / 2 - 1] += 2 * splitted;
            count += splitted;

            i += 1;
        }

        tot += i;
    }

    tot
}

#[register]
pub fn part2(input: &str) -> u64 {
    let mut tot = 0;

    for (_, growth, limit) in parse(input) {
        tot += solve_leaves(growth, limit);
    }

    tot
}

#[register]
pub fn part3(input: &str) -> u64 {
    let mut tot = 0;

    for (_, growth, limit) in parse(input) {
        tot += solve_leaves(growth, limit);
    }

    tot
}
