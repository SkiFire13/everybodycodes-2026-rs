use quest_helper::register;

#[register]
pub fn part1(input: &str) -> u64 {
    let (square1, square2) = input.split_once("\n\n").unwrap();
    let grid1 = square1
        .lines()
        .map(|l| l.trim().as_bytes())
        .collect::<Vec<_>>();
    let grid2 = square2
        .lines()
        .map(|l| l.trim().as_bytes())
        .collect::<Vec<_>>();

    let (mut c, mut m, mut y, mut k, mut r, mut g, mut b) = (0, 0, 0, 0, 0, 0, 0);

    let (w, h) = (grid1[0].len(), grid1.len());

    for iy in 0..h {
        for ix in 0..w {
            let b1 = grid1[iy][ix];
            let b2 = grid2[iy][ix];

            let b3 = match (b1.min(b2), b1.max(b2)) {
                (b'K', _) | (_, b'K') => b'K',
                (b'C', b'M') => b'B',
                (b'C', b'Y') => b'G',
                (b'M', b'Y') => b'R',
                (b1, b2) if b1 == b2 => b1,
                _ => unreachable!(),
            };

            match b3 {
                b'C' => c += 1,
                b'M' => m += 1,
                b'Y' => y += 1,
                b'K' => k += 1,
                b'R' => r += 1,
                b'G' => g += 1,
                b'B' => b += 1,
                _ => unreachable!(),
            }
        }
    }

    let mut counts = [c, m, y, k, r, g, b];
    counts.sort_unstable();

    counts[6] * counts[5] * counts[4]
}

#[register]
pub fn part2(input: &str) -> u64 {
    let (square1, square2) = input.split_once("\n\n").unwrap();
    let grid1 = square1
        .lines()
        .map(|l| l.trim().as_bytes())
        .collect::<Vec<_>>();
    let grid2 = square2
        .lines()
        .map(|l| l.trim().as_bytes())
        .collect::<Vec<_>>();

    let count_with_assignment = |map: [u8; 4]| {
        let (mut c, mut m, mut y, mut k, mut r, mut g, mut b) = (0, 0, 0, 0, 0, 0, 0);

        let (w, h) = (grid1[0].len(), grid1.len());

        for iy in 0..h {
            for ix in 0..w {
                let b1 = grid1[iy][ix];
                let i2 = grid2[iy][ix];
                let b2 = map[(i2 - b'1') as usize];

                let b3 = match (b1.min(b2), b1.max(b2)) {
                    (b'K', _) | (_, b'K') => b'K',
                    (b'C', b'M') => b'B',
                    (b'C', b'Y') => b'G',
                    (b'M', b'Y') => b'R',
                    (b1, b2) if b1 == b2 => b1,
                    _ => unreachable!(),
                };

                match b3 {
                    b'C' => c += 1,
                    b'M' => m += 1,
                    b'Y' => y += 1,
                    b'K' => k += 1,
                    b'R' => r += 1,
                    b'G' => g += 1,
                    b'B' => b += 1,
                    _ => unreachable!(),
                }
            }
        }

        let mut counts = [c, m, y, k, r, g, b];
        counts.sort_unstable();

        counts[6] * counts[5] * counts[4]
    };

    let mut best = 0;
    for b1 in [b'C', b'M', b'Y', b'K'] {
        for b2 in [b'C', b'M', b'Y', b'K'] {
            for b3 in [b'C', b'M', b'Y', b'K'] {
                for b4 in [b'C', b'M', b'Y', b'K'] {
                    let mut map = [b1, b2, b3, b4];
                    map.sort_unstable();
                    if map != *b"CKMY" {
                        continue;
                    }
                    best = best.max(count_with_assignment([b1, b2, b3, b4]));
                }
            }
        }
    }
    best
}

#[register]
pub fn part3(input: &str) -> u64 {
    let (mut c, mut m, mut y, mut k) = (0, 0, 0, 0);
    for b in input.bytes() {
        match b {
            b'C' => c += 1,
            b'M' => m += 1,
            b'Y' => y += 1,
            b'K' => k += 1,
            b'\n' => {}
            _ => unreachable!(),
        }
    }

    let mut best = c * m * y * k / c.min(m).min(y).min(k);

    // Merge c1+c2 into RGB and keep c3. Plus k.
    best = best.max((c + m) * y * k);
    best = best.max((c + y) * m * k);
    best = best.max((m + y) * c * k);

    // Split c3 and merge c1+c3 nd c2+c3 into RGB. Plus k.
    let merge_2_rgb = |mut c1: u64, mut c2: u64, mut c3: u64| -> u64 {
        let give_to_c1 = c2.saturating_sub(c1).min(c3);
        let give_to_c2 = c1.saturating_sub(c2).min(c3);

        c1 += give_to_c1;
        c2 += give_to_c2;
        c3 -= give_to_c1 + give_to_c2;

        c1 += c3 / 2;
        c2 += (c3 + 1) / 2;

        c1 * c2 * k
    };
    best = best.max(merge_2_rgb(c, m, y));
    best = best.max(merge_2_rgb(c, y, m));
    best = best.max(merge_2_rgb(m, y, c));

    // Split c1, c2, c3 and merge c1+c2, c1+c3 and c2+c3 into RGB. Ignore k.
    let merge_3_rgb = |c1: u64, c2: u64, c3: u64| -> u64 {
        let max = c1.max(c2).max(c3);
        let rem = c1 + c2 + c3 - max;

        if max > 2 * rem {
            let m1 = max / 2;
            let m2 = (max + 1) / 2;
            m1 * m2 * rem
        } else {
            let third = (c1 + c2 + c3) / 3;
            let r1 = (c1 + c2 + c3) % 3 > 0;
            let r2 = (c1 + c2 + c3) % 3 > 1;
            third * (third + r1 as u64) * (third + r2 as u64)
        }
    };
    best = best.max(merge_3_rgb(c, m, y));
    best = best.max(merge_3_rgb(c, y, m));
    best = best.max(merge_3_rgb(m, y, c));

    best
}
