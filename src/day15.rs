use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    sync::Arc,
};

use everybodycodes_helper::register;

type Pos = (usize, usize);
type Dir = (isize, isize);

const LEFT: Dir = (-1, 0);
const RIGHT: Dir = (1, 0);
const UP: Dir = (0, -1);
const DOWN: Dir = (0, 1);

struct W<T>(T);

impl<T> PartialEq for W<T> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<T> Eq for W<T> {}

impl<T> PartialOrd for W<T> {
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl<T> Ord for W<T> {
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}

fn parse(grid: &[&[u8]]) -> (Pos, Pos, Vec<Pos>) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut sulphurs = Vec::new();

    for (y, l) in grid.iter().enumerate() {
        for (x, &b) in l.iter().enumerate() {
            if b == b'S' {
                start = (x, y);
            } else if b == b'E' {
                end = (x, y);
            } else if b == b'@' {
                sulphurs.push((x, y));
            }
        }
    }

    (start, end, sulphurs)
}

fn turn_left(dir: Dir) -> Dir {
    (dir.1, -dir.0)
}

fn turn_right(dir: Dir) -> Dir {
    (-dir.1, dir.0)
}

fn apply_dir(pos: Pos, dir: Dir) -> Pos {
    (
        pos.0.wrapping_add_signed(dir.0),
        pos.1.wrapping_add_signed(dir.1),
    )
}

fn shortest_paths(grid: &[&[u8]], start: Pos, dir: Dir) -> HashMap<(Pos, Dir), u64> {
    let mut distances = HashMap::new();
    let mut seen = HashSet::new();
    let mut queue = BinaryHeap::from([(Reverse(0), start, dir)]);

    while let Some((Reverse(d), pos, dir)) = queue.pop() {
        if !seen.insert((pos, dir)) {
            continue;
        }

        let b = grid[pos.1][pos.0];
        if pos != start && (b == b'@' || b == b'E') {
            distances.insert((pos, dir), d);
            continue;
        }

        let mut add = |pos: Pos, dir: Dir| {
            let b = grid[pos.1][pos.0];
            if b != b'#' && b != b'S' {
                queue.push((Reverse(d + 1), pos, dir));
            }
        };

        add(apply_dir(pos, dir), dir);

        let left = turn_left(dir);
        add(apply_dir(pos, left), left);

        let right = turn_right(dir);
        add(apply_dir(pos, right), right);
    }

    distances
}

fn solve_for_goal(input: &str, goal: u64) -> u64 {
    let grid = input
        .lines()
        .map(|l| l.trim().as_bytes())
        .collect::<Vec<_>>();

    let (start, end, sulphurs) = parse(&grid);

    let mut edges = HashMap::new();
    edges.insert((start, RIGHT), shortest_paths(&grid, start, RIGHT));
    for &pos in &sulphurs {
        for dir in [LEFT, RIGHT, UP, DOWN] {
            edges.insert((pos, dir), shortest_paths(&grid, pos, dir));
        }
    }

    let seen_this_none = Arc::new(HashMap::<(Pos, Dir), (u64, u64)>::new());

    let mut seen = HashMap::new();
    let mut queue = BinaryHeap::from([(Reverse(0), start, RIGHT, 0u64, W(Some(seen_this_none)))]);
    let mut min = u64::MAX;

    while let Some((Reverse(mut c), pos, dir, mut visited, W(mut seen_this))) = queue.pop() {
        if pos == end {
            min = min.min(c);
            continue;
        }

        if c >= min {
            continue;
        }

        if seen.get(&(pos, dir)).is_some_and(|&v| v >= visited) {
            continue;
        }
        seen.insert((pos, dir), visited);

        if let Some(s) = &mut seen_this {
            if let Some(&(prev_c, prev_visited)) = s.get(&(pos, dir)) {
                let diff_visited = visited - prev_visited;
                let diff_cost = c - prev_c;
                let cycles = (goal - visited) / diff_visited;

                c += diff_cost * cycles.saturating_sub(1);
                visited += diff_visited * cycles.saturating_sub(1);
                seen_this = None;
            } else {
                Arc::make_mut(s).insert((pos, dir), (c, visited));
            }
        }

        for (&(npos, ndir), &d) in &edges[&(pos, dir)] {
            if c + d < min && (npos != end || visited >= goal) {
                queue.push((
                    Reverse(c + d),
                    npos,
                    ndir,
                    visited + 1,
                    W(seen_this.clone()),
                ));
            }
        }
    }

    min
}

#[register]
pub fn part1(input: &str) -> u64 {
    let grid = input
        .lines()
        .map(|l| l.trim().as_bytes())
        .collect::<Vec<_>>();

    let (start, end, sulphurs) = parse(&grid);
    let pos_to_sulphur = sulphurs
        .iter()
        .enumerate()
        .map(|(i, &s)| (s, i))
        .collect::<HashMap<_, _>>();

    let mut edges = HashMap::new();
    edges.insert((start, RIGHT), shortest_paths(&grid, start, RIGHT));
    for &pos in &sulphurs {
        for dir in [LEFT, RIGHT, UP, DOWN] {
            edges.insert((pos, dir), shortest_paths(&grid, pos, dir));
        }
    }

    let all_visited = (1 << sulphurs.len()) - 1;
    let mut seen = HashSet::new();
    let mut queue = BinaryHeap::from([(Reverse(0), start, RIGHT, 0u64)]);

    while let Some((Reverse(c), pos, dir, visited)) = queue.pop() {
        if !seen.insert((pos, dir, visited)) {
            continue;
        }

        if visited == all_visited && pos == end {
            return c;
        }

        for (&(npos, ndir), &d) in &edges[&(pos, dir)] {
            if let Some(&s) = pos_to_sulphur.get(&npos) {
                if visited & (1 << s) == 0 {
                    queue.push((Reverse(c + d), npos, ndir, visited | (1 << s)));
                }
            } else if visited == all_visited {
                queue.push((Reverse(c + d), npos, ndir, visited));
            }
        }
    }

    unreachable!()
}

#[register]
pub fn part2(input: &str) -> u64 {
    solve_for_goal(input, 2026)
}

#[register]
pub fn part3(input: &str) -> u64 {
    solve_for_goal(input, 202620262026)
}
