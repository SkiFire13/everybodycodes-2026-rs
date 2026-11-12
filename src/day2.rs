use std::collections::VecDeque;

use everybodycodes_helper::register;

fn solve(input: &str) -> u64 {
    let mut tot = 0;
    let mut next = vec![Vec::new(); 256];

    let mut seen = Vec::new();
    let mut prev = vec![0; 256];
    let mut queue = VecDeque::new();

    for line in input.lines() {
        for cmp in line.split(' ') {
            let d1 = cmp.as_bytes()[0] - b'0';
            let d2 = cmp.as_bytes()[2] - b'0';
            let (d1, d2) = match cmp.as_bytes()[1] {
                b'<' => (d1, d2),
                b'>' => (d2, d1),
                _ => unreachable!(),
            };

            next[d1 as usize].push(d2);
            prev[d2 as usize] += 1;

            if next[d1 as usize].len() == 1 && prev[d1 as usize] == 0 {
                seen.push(d1);
            }

            if next[d2 as usize].len() == 0 && prev[d2 as usize] == 1 {
                seen.push(d2);
            }
        }

        queue.extend(seen.drain(..).filter(|&n| prev[n as usize] == 0));

        let mut res = 0;
        let mut fac = 1;

        while let Some(i) = queue.pop_front() {
            res += i as u64 * fac;
            fac *= 10;

            for n in next[i as usize].drain(..) {
                prev[n as usize] -= 1;
                if prev[n as usize] == 0 {
                    queue.push_back(n);
                }
            }
        }

        tot += res;
    }

    tot
}

#[register]
pub fn part1(input: &str) -> u64 {
    solve(input)
}

#[register]
pub fn part2(input: &str) -> u64 {
    solve(input)
}

#[register]
pub fn part3(input: &str) -> u64 {
    solve(input)
}
