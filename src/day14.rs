use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use everybodycodes_helper::register;

const ROOT: u8 = b'\0';

fn build_adj(grid: &[&[u8]]) -> HashMap<u8, HashMap<u8, u64>> {
    let (w, h) = (grid[0].len(), grid.len());
    let mut seen = vec![false; w * h];

    let mut edges = HashMap::new();

    for (y, l) in grid.iter().enumerate() {
        for (x, &b) in l.iter().enumerate() {
            if seen[w * y + x] {
                continue;
            }
            seen[w * y + x] = true;

            let mut queue = vec![(x, y)];
            while let Some((x, y)) = queue.pop() {
                let mut add = |x: usize, y: usize| {
                    if x >= w || y >= h {
                        *edges.entry((ROOT, b)).or_insert(0) += 1;
                    } else if grid[y][x] == b {
                        if !seen[w * y + x] {
                            queue.push((x, y));
                            seen[w * y + x] = true;
                        }
                    } else if grid[y][x] > b {
                        *edges.entry((b, grid[y][x])).or_insert(0) += 1;
                    }
                };

                add(x.wrapping_add_signed(-1), y);
                add(x.wrapping_add_signed(1), y);
                add(x, y.wrapping_add_signed(-1));
                add(x, y.wrapping_add_signed(1));
            }
        }
    }

    let mut adj = HashMap::new();
    for (&(u, v), &c) in &edges {
        adj.entry(u).or_insert_with(HashMap::new).insert(v, c);
        adj.entry(v).or_insert_with(HashMap::new).insert(u, c);
    }

    adj
}

fn solve_with_charges(adj: &HashMap<u8, HashMap<u8, u64>>, is_charge: impl Fn(u8) -> bool) -> u64 {
    let mut reach = HashSet::new();
    let mut queue = BinaryHeap::from([(Reverse(0), ROOT)]);
    let mut tot = 0;

    while let Some((Reverse(c), v)) = queue.pop() {
        if reach.insert(v) {
            tot += c;
            queue.extend(adj[&v].iter().map(|(&u, &c)| {
                let zeroc = is_charge(v) || is_charge(u);
                (Reverse(if zeroc { 0 } else { c }), u)
            }));
        }
    }

    tot
}

#[register]
pub fn part1(input: &str) -> u64 {
    let grid = input
        .lines()
        .map(|l| l.trim().as_bytes())
        .collect::<Vec<_>>();

    let adj = build_adj(&grid);
    adj.values().flat_map(|m| m.values()).sum::<u64>() / 2
}

#[register]
pub fn part2(input: &str) -> u64 {
    let grid = input
        .lines()
        .map(|l| l.trim().as_bytes())
        .collect::<Vec<_>>();

    let adj = build_adj(&grid);

    solve_with_charges(&adj, |_| false)
}

#[register]
pub fn part3(input: &str) -> u64 {
    let grid = input
        .lines()
        .map(|l| l.trim().as_bytes())
        .collect::<Vec<_>>();

    let adj = build_adj(&grid);

    let mut min = u64::MAX;
    let charges = adj
        .keys()
        .copied()
        .filter(|&u| u != ROOT)
        .collect::<Vec<_>>();
    for (i, &ch1) in charges.iter().enumerate() {
        for &ch2 in &charges[i + 1..] {
            min = min.min(solve_with_charges(&adj, |u| u == ch1 || u == ch2));
        }
    }
    min
}
