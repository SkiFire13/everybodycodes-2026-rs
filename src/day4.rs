use std::{cmp::Reverse, collections::HashMap};

use everybodycodes_helper::register;

fn parse_input(input: &str) -> impl Iterator<Item = (u64, u64, u64, u64)> {
    input.lines().map(|line| {
        let rest = line.strip_prefix("id=").unwrap();
        let (id, rest) = rest.split_once(" quest=").unwrap();
        let (quest, rest) = rest.split_once(" part=").unwrap();
        let (part, time) = rest.split_once(" time=").unwrap();

        let id = id.parse().unwrap();
        let quest = quest.parse().unwrap();
        let part = part.parse().unwrap();
        let time = time.parse().unwrap();
        (id, quest, part, time)
    })
}

fn iter_ties<T, K: Eq>(s: &[T], mut tie: impl FnMut(&T) -> K) -> impl Iterator<Item = (usize, &T)> {
    let mut pos = 0;
    s.chunk_by(move |a, b| tie(a) == tie(b)).flat_map(move |c| {
        let base_pos = pos;
        pos += c.len();
        c.iter().map(move |t| (base_pos, t))
    })
}

#[register]
pub fn part1(input: &str) -> u64 {
    let mut times = HashMap::new();

    for (id, _, _, time) in parse_input(input) {
        *times.entry(id).or_insert(0) += time;
    }

    let mut times = times.into_values().collect::<Vec<_>>();
    times.sort_unstable();

    times
        .iter()
        .enumerate()
        .map(|(i, &time)| (i as u64 + 1) * time)
        .sum()
}

#[register]
pub fn part2(input: &str) -> u64 {
    let mut times = HashMap::new();

    for (id, _, _, time) in parse_input(input) {
        let (parts, tot_time) = times.entry(id).or_insert((0, 0));
        *parts += 1;
        *tot_time += time;
    }

    let mut times = times.into_values().collect::<Vec<_>>();
    times.sort_unstable_by_key(|&(parts, time)| (Reverse(parts), time));

    times
        .iter()
        .enumerate()
        .map(|(i, &(_, time))| (i as u64 + 1) * time)
        .sum()
}

#[register]
pub fn part3(input: &str) -> u64 {
    let mut times = HashMap::new();

    for (id, quest, part, time) in parse_input(input) {
        let times = times
            .entry(quest)
            .or_insert_with(HashMap::new)
            .entry(id)
            .or_insert_with(|| [0; 3]);

        times[part as usize - 1] = time;
    }

    let mut tot = HashMap::new();
    let mut acc = Vec::<(u64, u64)>::new();
    for quest_times in times.values() {
        for part in 0..3 {
            acc.clear();
            acc.reserve(quest_times.len());

            for (&id, &part_times) in quest_times {
                if part_times[part] != 0 {
                    let time = part_times[..=part].into_iter().sum();
                    acc.push((time, id));
                }
            }

            acc.sort_unstable();

            let base = (part as u64 + 1) * 50;
            for (pos, &(_, id)) in iter_ties(&acc, |&(t, _)| t) {
                if base > pos as u64 {
                    *tot.entry(id).or_insert(0) += base - pos as u64;
                }
            }
        }
    }

    let mut points = tot.into_values().collect::<Vec<_>>();
    points.sort_unstable_by_key(|&points| Reverse(points));

    iter_ties(&points, |&p| p)
        .map(|(i, &points)| (i as u64 + 1) * points)
        .sum()
}
