use std::array;

use everybodycodes_helper::register;

fn parse(input: &str) -> ([i64; 3], [i64; 3], [i64; 3], [i64; 3]) {
    let rest = input.strip_prefix("hands ").unwrap();
    let (hands, rest) = rest.split_once("\ndirection ").unwrap();
    let (directions, rest) = rest.split_once("\ncurrent ").unwrap();
    let (currents, moves) = rest.split_once("\nmove ").unwrap();

    let mut hands = hands.split(':').map(|s| s.parse::<i64>().unwrap());
    let hands = array::from_fn(|_| hands.next().unwrap());

    let mut directions = directions.split(':').map(|s| match s {
        "CW" => 1,
        "CCW" => -1,
        _ => unreachable!(),
    });
    let directions = array::from_fn(|_| directions.next().unwrap());

    let mut currents = currents.split(':').map(|s| s.parse::<i64>().unwrap());
    let currents = array::from_fn(|_| currents.next().unwrap());

    let mut moves = moves.split(':').map(|s| s.parse::<i64>().unwrap());
    let moves = array::from_fn(|_| moves.next().unwrap());

    (hands, directions, currents, moves)
}

fn lcm(a: i64, b: i64) -> i64 {
    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 { a } else { gcd(b, a % b) }
    }

    a / gcd(a, b) * b
}

fn pos_eq(left: i64, left_hands: i64, right: i64, right_hands: i64) -> bool {
    left * right_hands == right * left_hands
}

fn pos_contain(
    left: i64,
    right: i64,
    mid: i64,
    range_hands: i64,
    mid_hands: i64,
    direction: i64,
) -> bool {
    let left = left * mid_hands;
    let mut right = right * mid_hands;
    let mut mid = mid * range_hands;

    if (right - left) * direction <= 0 {
        right += range_hands * mid_hands * direction;
    }

    if (mid - left) * direction <= 0 {
        mid += range_hands * mid_hands * direction;
    }

    let left_to_mid = (mid - left) * direction;
    let left_to_right = (right - left) * direction;

    left_to_right > left_to_mid
}

fn simulate(
    hands: &[i64; 3],
    directions: &[i64; 3],
    curr: &[i64; 3],
    moves: &[i64; 3],
    m: i64,
) -> ([i64; 3], u64) {
    let mut count = 0;
    let mut next = *curr;

    for i in 0..3 {
        if m >= moves[i] {
            continue;
        }

        next[i] = curr[i] + directions[i];

        if next[i] < 0 {
            next[i] += hands[i]
        } else if next[i] == hands[i] {
            next[i] = 0;
        }
    }

    for i in 0..3 {
        if m >= moves[i] {
            continue;
        }

        for j in 0..3 {
            if i == j {
                continue;
            }

            let diri = directions[i];

            let contain_curr = pos_contain(curr[i], next[i], curr[j], hands[i], hands[j], diri);
            let contain_next = pos_contain(curr[i], next[i], next[j], hands[i], hands[j], diri);
            let next_eq_next = pos_eq(next[i], hands[i], next[j], hands[j]);
            let curr_eq_next = pos_eq(curr[i], hands[i], next[j], hands[j]);
            let next_eq_curr = pos_eq(next[i], hands[i], curr[j], hands[j]);

            if m >= moves[j] {
                // this moves and crosses other still
                if contain_curr || next_eq_next {
                    count += 1;
                }
            } else if diri == directions[j] || m >= moves[j] {
                // CW/CW or CCW/CCW overtake
                if contain_curr && (contain_next || next_eq_next) {
                    count += 1;
                }
            } else if diri == 1 {
                // this CW and other CCW crossing
                if contain_curr || contain_next || next_eq_next || curr_eq_next || next_eq_curr {
                    count += 1;
                }
            } else if diri == -1 {
                if contain_curr && contain_next {
                    count += 1;
                }
            }
        }
    }

    (next, count)
}

fn solve(input: &str) -> u64 {
    let mut tot = 0;

    let (hands, directions, mut curr, moves) = parse(input);
    let max_moves = moves.iter().copied().max().unwrap();

    let mut m = 0;
    while m < max_moves {
        let loop_len = (0..3)
            .filter(|&i| m < moves[i])
            .map(|i| hands[i])
            .fold(1, |acc, h| lcm(acc, h));
        let max_loop_len = (0..3)
            .filter(|&i| m < moves[i])
            .map(|i| moves[i] - m)
            .min()
            .unwrap();

        if 2 * loop_len < max_loop_len {
            let mut loop_tot = 0;

            for _ in 0..loop_len {
                let (next, count) = simulate(&hands, &directions, &curr, &moves, m);
                curr = next;
                loop_tot += count;
            }

            let loops = max_loop_len / loop_len;

            m += loop_len * loops;
            tot += loop_tot * loops as u64;
        } else {
            let next_death = (0..3)
                .filter(|&i| m < moves[i])
                .map(|i| moves[i])
                .min()
                .unwrap();

            while m < next_death {
                let (next, count) = simulate(&hands, &directions, &curr, &moves, m);
                curr = next;
                tot += count;
                m += 1;
            }
        }
    }

    tot
}

#[register]
pub fn part1(input: &str) -> u64 {
    let (hands, directions, mut curr, moves) = parse(input);
    let max_moves = moves.iter().copied().max().unwrap();
    let mut tot = 0;

    for m in 0..max_moves {
        let (next, count) = simulate(&hands, &directions, &curr, &moves, m);
        curr = next;
        tot += count;
    }

    tot
}

#[register]
pub fn part2(input: &str) -> u64 {
    let mut tot = 0;

    for input in input.split("\n\n") {
        tot += solve(input);
    }

    tot
}

#[register]
pub fn part3(input: &str) -> u64 {
    let mut tot = 0;

    for input in input.split("\n\n") {
        tot += solve(input);
    }

    tot
}
