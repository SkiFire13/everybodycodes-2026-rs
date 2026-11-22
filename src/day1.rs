use quest_helper::register;

#[register]
pub fn part1(input: &str) -> u64 {
    let mut tot = 0;

    for n in input.split(",") {
        let n = n.parse::<u64>().unwrap();
        let r = (n - 1).isqrt() + 1;
        tot += r * r - n;
    }

    tot
}

#[register]
pub fn part2(input: &str) -> u64 {
    let mut tot = 0;

    for n in input.split(",") {
        let n = n.parse::<u64>().unwrap();
        let m = n.max(8).next_multiple_of(4);
        tot += m - n;
    }

    tot
}

#[register]
pub fn part3(input: &str) -> u64 {
    let mut tot = 0;

    for n in input.split(",") {
        let n = n.parse::<u64>().unwrap();
        let r = n.isqrt().next_multiple_of(2) + 1;
        tot += r * r - 1 - n;
    }

    tot
}
