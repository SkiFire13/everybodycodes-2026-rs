use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use everybodycodes_helper::register;

#[register]
pub fn part1(input: &str) -> u64 {
    let grid = input
        .lines()
        .map(|l| l.trim().as_bytes())
        .collect::<Vec<_>>();

    let (w, h) = (grid[0].len(), grid.len());
    let start = (0, 0);
    let end = (w - 1, h - 1);

    let mut seen = HashSet::new();
    let mut queue = BinaryHeap::from([(Reverse(0), start)]);

    while let Some((Reverse(c), pos @ (x, y))) = queue.pop() {
        if !seen.insert(pos) {
            continue;
        }

        if pos == end {
            return c;
        }

        let mut add = |x: usize, y: usize| {
            if x < w && y < h && grid[y][x] == b'#' {
                queue.push((Reverse(c + 1), (x, y)));
            }
        };

        add(x.wrapping_add_signed(1), y);
        add(x.wrapping_add_signed(-1), y);
        add(x, y.wrapping_add_signed(1));
        add(x, y.wrapping_add_signed(-1));
    }

    unreachable!();
}

#[register]
pub fn part2(input: &str) -> u64 {
    let grid = input
        .lines()
        .map(|l| l.trim().as_bytes())
        .collect::<Vec<_>>();

    let (w, h) = (grid[0].len(), grid.len());

    let mut tot = 0;

    for (y, l) in grid.iter().enumerate() {
        for (x, &b) in l.iter().enumerate() {
            if b == b'#' {
                let mut add = |x: usize, y: usize| {
                    if !(x < w && y < h && grid[y][x] == b'#') {
                        tot += 1;
                    }
                };

                add(x.wrapping_add_signed(1), y);
                add(x.wrapping_add_signed(-1), y);
                add(x, y.wrapping_add_signed(1));
                add(x, y.wrapping_add_signed(-1));
            }
        }
    }

    tot
}

#[register]
pub fn part3(input: &str) -> u64 {
    let grid = input
        .lines()
        .map(|l| l.trim().as_bytes())
        .collect::<Vec<_>>();

    let (w, h) = (grid[0].len(), grid.len());

    let solve = |queue: &mut Vec<(usize, usize)>, seen: &mut HashSet<(usize, usize)>| {
        let mut cost = 0;
        while let Some(pos @ (x, y)) = queue.pop() {
            if !seen.insert(pos) {
                continue;
            }

            let mut handle = |x: usize, y: usize, edge: bool| {
                if x < w && y < h {
                    if grid[y][x] == b'.' {
                        queue.push((x, y));
                    } else if edge {
                        cost += 1;
                    }
                }
            };

            handle(x.wrapping_add_signed(1), y, true);
            handle(x.wrapping_add_signed(-1), y, true);
            handle(x, y.wrapping_add_signed(1), true);
            handle(x, y.wrapping_add_signed(-1), true);
            handle(x.wrapping_add_signed(1), y.wrapping_add_signed(1), false);
            handle(x.wrapping_add_signed(-1), y.wrapping_add_signed(1), false);
            handle(x.wrapping_add_signed(1), y.wrapping_add_signed(-1), false);
            handle(x.wrapping_add_signed(-1), y.wrapping_add_signed(-1), false);
        }
        cost
    };

    let is_air = |&(x, y): &(usize, usize)| grid[y][x] == b'.';

    let mut queue = Vec::new();
    let mut seen = HashSet::new();

    let mut base = w + h;
    queue.extend((0..w).map(|x| (x, 0)).filter(is_air));
    queue.extend((0..h).map(|y| (w - 1, y)).filter(is_air));
    base -= queue.len();
    let top_cost = base as u64 + solve(&mut queue, &mut seen);

    queue.clear();
    seen.clear();

    let mut base = w + h;
    queue.extend((0..w).map(|x| (x, h - 1)).filter(is_air));
    queue.extend((0..h).map(|y| (0, y)).filter(is_air));
    base -= queue.len();
    let bottom_cost = base as u64 + solve(&mut queue, &mut seen);

    top_cost.min(bottom_cost)
}
