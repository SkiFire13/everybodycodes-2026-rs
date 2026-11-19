use everybodycodes_helper::register;

fn solve(input: &str) -> u64 {
    let mut stacks = input
        .lines()
        .map(|line| {
            let line = line.strip_prefix("x=").unwrap();
            let (x, rest) = line.split_once(" y=").unwrap();
            let (y, w) = rest.split_once(" width=").unwrap_or((rest, "1"));
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();
            let w = w.parse::<usize>().unwrap();
            (x, y, w)
        })
        .collect::<Vec<_>>();

    stacks.sort_unstable();

    let mut tot = 0;
    let (mut cx, mut cy, mut cw) = stacks[0];
    let mut ci = 0;

    for i in 1..stacks.len() {
        let (nx, ny, nw) = stacks[i];
        if ny > cy {
            tot += (nx - cx) * cy;
            (cx, cy, cw, ci) = (nx, ny, nw, i);
        }
    }

    tot += cy * cw;

    let (mut cx, mut cy, mut cw) = stacks[stacks.len() - 1];

    for i in (ci..stacks.len() - 1).rev() {
        let (nx, ny, nw) = stacks[i];
        if ny >= cy {
            tot += (cx + cw - nx - nw) * cy;
            (cx, cy, cw) = (nx, ny, nw);
        }
    }

    tot as u64
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
