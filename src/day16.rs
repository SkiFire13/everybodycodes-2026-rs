use everybodycodes_helper::register;

fn solve(mut state: Vec<u8>, mut f: impl FnMut(usize)) {
    let mut next_state = state.clone();
    let mut hole = 0;

    let mut i = 0;
    while i < state.len() - 1 {
        let b = state[i];

        if b == state[i + 1] {
            i += 1;
            continue;
        }

        let j = state.iter().rposition(|&c| b == c).unwrap();

        let (mut thisi, mut thisj) = (i + 1, j + 1);
        let (mut nexti, mut nextj) = (thisi, thisj);

        while thisi != thisj {
            f(thisj - thisi);

            while state[thisj - 1] == b {
                next_state[nexti] = b;
                thisj -= 1;
                nexti += 1;
            }

            while state[thisi] != b {
                next_state[nextj - 1] = state[thisi];
                thisi += 1;
                nextj -= 1;
            }

            if thisi == thisj {
                break;
            }

            f(thisj - thisi);

            while state[thisi] == b {
                next_state[nexti] = b;
                thisi += 1;
                nexti += 1;
            }

            while state[thisj - 1] != b {
                next_state[nextj - 1] = state[thisj - 1];
                thisj -= 1;
                nextj -= 1;
            }
        }

        if hole > j {
            next_state[j..hole].copy_from_slice(&state[j..hole]);
        }

        (state, next_state) = (next_state, state);
        hole = j;
        i = nexti;
    }
}

#[register]
pub fn part1(input: &str) -> u64 {
    let input = input.as_bytes().to_vec();

    let mut count = 0;
    solve(input, |_| count += 1);
    count
}

#[register]
pub fn part2(input: &str) -> u64 {
    let input = [input.as_bytes(); 10].concat();

    let mut count = 0;
    solve(input, |len| count += len as u64);
    count
}

#[register]
pub fn part3(input: &str) -> u64 {
    let input = [input.as_bytes(); 100].concat();

    let mut count = 0;
    solve(input, |len| count += len as u64);
    count
}
