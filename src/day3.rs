use quest_helper::register;

fn compute(base: &[u8], digits: &[u8]) -> u64 {
    let base = base.iter().map(|&b| (b - b'0') as u64).rev();
    let digits = digits.iter().map(|&b| (b - b'0') as u64).rev();
    base.cycle()
        .zip(digits)
        .enumerate()
        .map(|(i, (b, d))| d * b.pow(i as u32))
        .sum()
}

#[register]
pub fn part1(input: &str) -> u64 {
    let mut tot = 0;

    for line in input.lines() {
        let line = line.strip_prefix("ISO-").unwrap();
        let (base, digits) = line.split_once(" ").unwrap();
        tot += compute(base.as_bytes(), digits.as_bytes());
    }

    tot
}

#[register]
pub fn part2(input: &str) -> u64 {
    let mut tot = 0;

    for line in input.lines() {
        let line = line.strip_prefix("ISO-").unwrap();
        let (base, digits) = line.split_once(" ").unwrap();
        tot += compute(base.as_bytes(), digits.as_bytes());
    }

    tot
}

#[register]
pub fn part3(input: &str) -> u64 {
    let mut tot = 0;
    let mut buf = Vec::new();

    for line in input.lines() {
        let line = line.strip_prefix("ISO2-").unwrap();
        let (base, digits) = line.split_once(" ").unwrap();
        let num = compute(base.as_bytes(), digits.as_bytes());
        buf.clear();
        buf.resize(digits.len(), b'0');
        let buflen = buf.len();
        let mut len = 1;

        loop {
            let num2 = compute(base.as_bytes(), &buf[buflen - len..]);

            if num == num2 {
                break;
            }

            let mut i = 0;
            loop {
                if i == len {
                    buf[buflen - 1 - i] = b'1';
                    len += 1;
                    break;
                }

                if buf[buflen - 1 - i] != b'9' {
                    buf[buflen - 1 - i] += 1;
                    break;
                }

                buf[buflen - 1 - i] = b'0';
                i += 1;
            }
        }

        if buf == digits.as_bytes() {
            tot += num;
        }
    }

    tot
}
