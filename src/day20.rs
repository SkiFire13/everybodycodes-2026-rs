use everybodycodes_helper::register;

#[register]
pub fn part1(input: &str) -> u64 {
    let mut tot = 0;

    for line in input.lines() {
        let (left, right) = line.split_once(" = ").unwrap();

        let (ll, rest) = left.split_once(' ').unwrap();
        let (lo, lr) = rest.split_once(' ').unwrap();
        let (ll, lo, lr) = (ll.as_bytes()[0], lo.as_bytes()[0], lr.as_bytes()[0]);
        let (ll, lr) = ((ll - b'0') as i32, (lr - b'0') as i32);
        let l = match lo {
            b'-' => ll - lr,
            b'+' => ll + lr,
            _ => unreachable!(),
        };

        let (rl, rest) = right.split_once(' ').unwrap();
        let (ro, rr) = rest.split_once(' ').unwrap();
        let (rl, ro, rr) = (rl.as_bytes()[0], ro.as_bytes()[0], rr.as_bytes()[0]);
        let (rl, rr) = ((rl - b'0') as i32, (rr - b'0') as i32);
        let r = match ro {
            b'-' => rl - rr,
            b'+' => rl + rr,
            _ => unreachable!(),
        };

        if l == r {
            let counts = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];
            let op = [1, 2];
            tot += counts[ll as usize]
                + counts[lr as usize]
                + counts[rl as usize]
                + counts[rr as usize];
            tot += op[(lo == b'+') as usize] + op[(ro == b'+') as usize];
            tot += 2;
        }
    }

    tot
}

#[register]
pub fn part2(input: &str) -> u64 {
    let mut tot = 0;

    fn parse(s: &str) -> (i32, u64) {
        let mut n = 0;
        let mut count = 0;

        for b in s.bytes() {
            let i = match b {
                b'0'..=b'9' => (b - b'0') as i32,
                b'A' => 10,
                _ => unreachable!(),
            };
            n = 11 * n + i;

            let counts = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6, 6];
            count += counts[i as usize];
        }

        (n, count)
    }

    for line in input.lines() {
        let (left, right) = line.split_once(" = ").unwrap();

        let (ll, rest) = left.split_once(' ').unwrap();
        let (lo, lr) = rest.split_once(' ').unwrap();
        let ((ll, llc), lo, (lr, lrc)) = (parse(ll), lo.as_bytes()[0], parse(lr));
        let l = match lo {
            b'-' => ll - lr,
            b'+' => ll + lr,
            _ => unreachable!(),
        };

        let (rl, rest) = right.split_once(' ').unwrap();
        let (ro, rr) = rest.split_once(' ').unwrap();
        let ((rl, rlc), ro, (rr, rrc)) = (parse(rl), ro.as_bytes()[0], parse(rr));
        let r = match ro {
            b'-' => rl - rr,
            b'+' => rl + rr,
            _ => unreachable!(),
        };

        if l == r {
            let op = [1, 2];
            tot += llc + lrc + rlc + rrc;
            tot += op[(lo == b'+') as usize] + op[(ro == b'+') as usize];
            tot += 2;
        }
    }

    let mut tot11 = 0;
    let mut mul = 1;
    while tot != 0 {
        tot11 += mul * (tot % 11);
        mul *= 10;
        tot /= 11;
    }
    tot11
}

#[register]
pub fn part3(input: &str) -> u64 {
    const WITHIN: [&[i32]; 13] = [
        &[6, 9, 10],
        &[],
        &[3],
        &[2, 5],
        &[],
        &[3],
        &[0, 9, 10],
        &[],
        &[],
        &[0, 6, 10],
        &[0, 6, 9],
        &[],
        &[],
    ];

    const ADD: [&[i32]; 13] = [
        &[8],
        &[7],
        &[],
        &[9],
        &[],
        &[6, 9],
        &[8],
        &[],
        &[],
        &[8],
        &[8],
        &[12],
        &[],
    ];

    const REMOVE: [&[i32]; 13] = [
        &[],
        &[],
        &[],
        &[],
        &[],
        &[],
        &[5],
        &[1],
        &[0, 6, 9, 10],
        &[3, 5],
        &[],
        &[],
        &[11],
    ];

    const COUNT: [u64; 13] = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6, 6, 1, 2];

    let mut tot = 0;

    for line in input.lines() {
        fn parse(s: &str, buf: &mut [i32]) {
            for (b, d) in s.bytes().zip(buf) {
                *d = match b {
                    b'0'..=b'9' => (b - b'0') as i32,
                    b'A' => 10,
                    _ => unreachable!(),
                }
            }
        }

        fn check(buf: &[i32; 18]) -> bool {
            let ll = buf[0..4].iter().fold(0, |acc, &n| 11 * acc + n);
            let lr = buf[4..8].iter().fold(0, |acc, &n| 11 * acc + n);
            let rl = buf[8..12].iter().fold(0, |acc, &n| 11 * acc + n);
            let rr = buf[12..16].iter().fold(0, |acc, &n| 11 * acc + n);
            let ol = 2 * (buf[16] - 11) - 1;
            let or = 2 * (buf[17] - 11) - 1;
            ll + ol * lr == rl + or * rr
        }

        let mut buf = [0; 18];

        let (left, right) = line.split_once(" = ").unwrap();
        let (ll, rest) = left.split_once(' ').unwrap();
        let (lo, lr) = rest.split_once(' ').unwrap();
        let (rl, rest) = right.split_once(' ').unwrap();
        let (ro, rr) = rest.split_once(' ').unwrap();

        parse(ll, &mut buf[0..4]);
        parse(lr, &mut buf[4..8]);
        parse(rl, &mut buf[8..12]);
        parse(rr, &mut buf[12..16]);
        buf[16] = if lo == "-" { 11 } else { 12 };
        buf[17] = if ro == "-" { 11 } else { 12 };

        let any_valid = 'valid: {
            if check(&buf) {
                break 'valid true;
            }

            for i in 0..18 {
                for &b in WITHIN[buf[i] as usize] {
                    let mut buf = buf;
                    buf[i] = b;
                    if check(&buf) {
                        break 'valid true;
                    }
                }
            }

            for i in 0..18 {
                for &b1 in REMOVE[buf[i] as usize] {
                    for j in (0..18).filter(|&j| j != i) {
                        for &b2 in ADD[buf[j] as usize] {
                            let mut buf = buf;
                            buf[i] = b1;
                            buf[j] = b2;
                            if check(&buf) {
                                break 'valid true;
                            }
                        }
                    }
                }
            }

            false
        };

        if any_valid {
            tot += 2 + buf.iter().map(|&b| COUNT[b as usize]).sum::<u64>();
        }
    }

    let mut tot11 = 0;
    let mut mul = 1;
    while tot != 0 {
        tot11 += mul * (tot % 11);
        mul *= 10;
        tot /= 11;
    }
    tot11
}
