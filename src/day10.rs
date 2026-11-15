use std::collections::HashMap;

use everybodycodes_helper::register;

type Pos = (usize, usize);
type Dir = (isize, isize);

const UP: Dir = (0, -1);
const DOWN: Dir = (0, 1);
const LEFT: Dir = (-1, 0);
const RIGHT: Dir = (1, 0);

fn turn_90_clock(dir: Dir) -> Dir {
    (-dir.1, dir.0)
}

fn turn_opposite(dir: Dir) -> Dir {
    (-dir.0, -dir.1)
}

fn apply_move(pos: Pos, dir: Dir) -> Pos {
    (
        pos.0.wrapping_add_signed(dir.0),
        pos.1.wrapping_add_signed(dir.1),
    )
}

fn parse(input: &str) -> (Vec<&[u8]>, Vec<(Pos, Dir)>) {
    let grid = input
        .lines()
        .map(|l| l.trim().as_bytes())
        .collect::<Vec<_>>();

    let ghosts = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &b)| (x, y, b)))
        .filter(|&(_, _, b)| matches!(b, b'<' | b'>' | b'^' | b'v'))
        .filter_map(|(x, y, b)| {
            let dir = match b {
                b'^' => UP,
                b'v' => DOWN,
                b'<' => LEFT,
                b'>' => RIGHT,
                _ => return None,
            };
            Some(((x, y), dir))
        })
        .collect::<Vec<_>>();

    (grid, ghosts)
}

fn simulate(
    grid: &[&[u8]],
    ghosts: &mut [(Pos, Dir)],
    collisions: &mut [u64],
    occupied_slots: &mut HashMap<Pos, Vec<usize>>,
    max_collisions: u64,
) -> usize {
    let mut dead = 0;

    for (i, (pos, dir)) in ghosts.iter_mut().enumerate() {
        if collisions[i] >= max_collisions {
            continue;
        }

        let new_pos @ (nx, ny) = apply_move(*pos, *dir);
        if grid[ny][nx] == b'#' {
            *dir = turn_90_clock(*dir);

            collisions[i] += 1;
            if collisions[i] >= max_collisions {
                dead += 1;
                continue;
            }
        } else {
            *pos = new_pos;
        }

        occupied_slots.entry(*pos).or_insert_with(Vec::new).push(i);
    }

    for overlaps in occupied_slots.values_mut() {
        if overlaps.len() <= 1 {
            overlaps.clear();
            continue;
        }

        for i in overlaps.drain(..) {
            let (_, dir) = &mut ghosts[i];
            *dir = turn_opposite(*dir);

            collisions[i] += 1;
            if collisions[i] >= max_collisions {
                dead += 1;
            }
        }
    }

    dead
}

#[register]
pub fn part1(input: &str) -> u64 {
    let (grid, mut ghosts) = parse(input);

    let mut occupied_slots = HashMap::new();
    let mut collisions = vec![0; ghosts.len()];

    for _ in 0..10 {
        simulate(
            &grid,
            &mut ghosts,
            &mut collisions,
            &mut occupied_slots,
            u64::MAX,
        );
    }

    collisions.iter().copied().sum()
}

#[register]
pub fn part2(input: &str) -> u64 {
    let (grid, mut ghosts) = parse(input);

    let mut alive = ghosts.len();
    let mut collisions = vec![0; ghosts.len()];
    let mut occupied_slots = HashMap::<Pos, Vec<usize>>::new();

    for i in 1.. {
        alive -= simulate(
            &grid,
            &mut ghosts,
            &mut collisions,
            &mut occupied_slots,
            10000,
        );

        if alive == 0 {
            return i;
        }
    }

    unreachable!()
}

#[register]
pub fn part3(input: &str) -> u64 {
    let (grid, mut ghosts) = parse(input);

    const MAX_COLLISIONS: u64 = 1000000000000;

    let mut alive = ghosts.len();
    let mut collisions = vec![0; ghosts.len()];
    let mut occupied_slots = HashMap::<Pos, Vec<usize>>::new();
    let mut round = 0;

    'outer: loop {
        if alive == 0 {
            return round;
        }

        // map from ghosts => collisions + last_seen
        let mut seen = HashMap::<Vec<(Pos, Dir)>, (Vec<u64>, u64)>::new();

        loop {
            round += 1;

            let dead = simulate(
                &grid,
                &mut ghosts,
                &mut collisions,
                &mut occupied_slots,
                MAX_COLLISIONS,
            );

            if dead != 0 {
                alive -= dead;
                continue 'outer;
            }

            if let Some((prev_collisions, prev_round)) = seen.get(&ghosts) {
                let loop_len = round - prev_round;
                let repetitions = collisions
                    .iter()
                    .zip(prev_collisions)
                    .filter(|&(&c, _)| c < MAX_COLLISIONS)
                    .filter(|&(&c, &prev_c)| c != prev_c)
                    .map(|(&c, &prev_c)| (MAX_COLLISIONS - 1 - c) / (c - prev_c))
                    .min()
                    .unwrap();

                round += loop_len * repetitions;

                for (c, &prev_c) in collisions.iter_mut().zip(prev_collisions) {
                    if *c >= MAX_COLLISIONS {
                        continue;
                    }

                    *c += (*c - prev_c) * repetitions;

                    if *c >= MAX_COLLISIONS {
                        alive -= 1;
                    }
                }

                continue 'outer;
            }

            seen.insert(ghosts.clone(), (collisions.clone(), round));
        }
    }
}
