use std::{collections::VecDeque, ops::Deref};

use quest_helper::register;

const UP_LEFT: (isize, isize) = (-1, -1);
const UP_RIGHT: (isize, isize) = (1, -1);
const DONW_LEFT: (isize, isize) = (-1, 1);
const DOWN_RIGHT: (isize, isize) = (1, 1);
const LEFT: (isize, isize) = (-2, 0);
const RIGHT: (isize, isize) = (2, 0);

fn get_stages<T: Deref<Target = [u8]>>(
    levels: &[T],
) -> [((usize, usize), (isize, isize), (isize, isize)); 6] {
    let midy = levels.len() / 2;
    let lx = levels[0].iter().position(|&b| b != b'.').unwrap();
    let rx = levels[0].iter().rposition(|&b| b != b'.').unwrap();

    let mid_left = (0, midy);
    let mid_right = (levels[midy].len() - 1, midy);
    let top_right = (rx, 0);
    let top_left = (lx, 0);

    [
        (mid_left, DOWN_RIGHT, UP_RIGHT),
        (mid_right, DONW_LEFT, UP_LEFT),
        (top_right, LEFT, DOWN_RIGHT),
        (mid_right, UP_LEFT, DONW_LEFT),
        (mid_left, UP_RIGHT, DOWN_RIGHT),
        (top_left, RIGHT, DONW_LEFT),
    ]
}

fn simulate_stage<T: Deref<Target = [u8]>>(
    levels: &[T],
    (mut sx, mut sy): (usize, usize),
    (dx, dy): (isize, isize),
    (nx, ny): (isize, isize),
    mut on_light: impl FnMut(usize, usize),
) {
    'outer: while levels[sy][sx] != b'.' {
        let (mut x, mut y) = (sx, sy);

        let mut shadow = 0;
        while let Some(l) = levels.get(y)
            && let Some(&n) = l.get(x)
            && n != b'.'
        {
            let n = (n - b'0') as u64;
            if n >= shadow {
                on_light(x, y);
            }
            shadow = shadow.saturating_sub(1).max(n);
            (x, y) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
        }

        (sx, sy) = (sx.wrapping_add_signed(nx), sy.wrapping_add_signed(ny));

        if let Some(l) = levels.get(sy)
            && let Some(&n) = l.get(sx)
            && n != b'.'
        {
            continue;
        }

        loop {
            (sx, sy) = (sx.wrapping_add_signed(dx), sy.wrapping_add_signed(dy));

            if let Some(l) = levels.get(sy)
                && let Some(&n) = l.get(sx)
            {
                if n != b'.' {
                    continue 'outer;
                }
            } else {
                break 'outer;
            }
        }
    }
}

#[register]
pub fn part1(input: &str) -> u64 {
    let levels = input
        .lines()
        .map(|s| s.trim_end().as_bytes())
        .collect::<Vec<_>>();

    let stages = get_stages(&levels);
    let (start, d, n) = stages[0];

    let mut tot = 0;
    simulate_stage(&levels, start, d, n, |_, _| tot += 1);
    tot
}

#[register]
pub fn part2(input: &str) -> u64 {
    let levels = input
        .lines()
        .map(|s| s.trim_end().as_bytes())
        .collect::<Vec<_>>();

    let stages = get_stages(&levels);

    let mut tot = 0;
    for (start, d, n) in stages {
        simulate_stage(&levels, start, d, n, |_, _| tot += 1);
    }
    tot
}

#[register]
pub fn part3(input: &str) -> u64 {
    let mut levels = input
        .lines()
        .map(|s| s.trim_end().as_bytes().to_vec())
        .collect::<Vec<_>>();

    let stages = get_stages(&levels);

    let mut sprouts = VecDeque::<Vec<(usize, usize)>>::from([vec![], vec![], vec![]]);
    let mut next_sprouts = Vec::<(usize, usize)>::new();
    let mut to_grow = Vec::<(usize, usize)>::new();
    let mut warriors = 0;
    let mut tot = 0;

    for &(start, d, n) in stages.iter().cycle() {
        simulate_stage(&levels, start, d, n, |x, y| to_grow.push((x, y)));

        for (x, y) in to_grow.drain(..) {
            if levels[y][x] == b'0' {
                continue;
            }

            levels[y][x] += 1;
            tot += 1;

            if levels[y][x] == b'9' + 1 {
                levels[y][x] = b'0';
                next_sprouts.push((x, y));
                warriors += 1;
            }
        }

        let mut to_sprout = sprouts.pop_front().unwrap();
        for (x, y) in to_sprout.drain(..) {
            levels[y][x] = b'1';
        }

        sprouts.push_back(next_sprouts);
        next_sprouts = to_sprout;

        if warriors >= 100000 {
            return tot;
        }
    }

    unreachable!()
}
