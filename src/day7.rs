use quest_helper::register;

fn parse(input: &str) -> (Vec<u64>, Vec<Vec<usize>>, usize) {
    let mut max = (0, 0);
    let mut energies = Vec::new();
    let mut connections = Vec::new();

    for line in input.lines() {
        let rest = line.strip_prefix("id=").unwrap();
        let (id, rest) = rest.split_once(", energy=").unwrap();
        let (energy, connection) = rest.split_once(", connections=").unwrap();

        let id = id.parse::<usize>().unwrap() - 1;
        let energy = energy.parse::<u64>().unwrap();
        let connection = connection
            .split(',')
            .map(|c| c.parse::<usize>().unwrap() - 1)
            .collect::<Vec<_>>();

        max = max.max((connection.len(), id));
        energies.push(energy);
        connections.push(connection);
    }

    (energies, connections, max.1)
}

fn propagate(
    energies: &mut [u64],
    connections: &[Vec<usize>],
    start: usize,
    strength: u64,
    max_energy: u64,
) {
    let mut curr_impulses = vec![[0; 10]; energies.len()];
    let mut next_impulses = vec![[0; 10]; energies.len()];
    let mut exploded = vec![false; energies.len()];

    curr_impulses[start][strength as usize - 1] = 1;

    loop {
        let mut changed = false;

        for (i, ns) in curr_impulses.iter_mut().enumerate() {
            if exploded[i] {
                continue;
            }

            for strength in 1..11 {
                let n = &mut ns[strength as usize - 1];

                if *n == 0 {
                    continue;
                }

                energies[i] += strength * *n;

                let impulse = strength - 1;
                if impulse != 0 {
                    for &c in &connections[i] {
                        next_impulses[c][impulse as usize - 1] += *n;
                        changed = true;
                    }
                }

                *n = 0;
            }

            if energies[i] >= max_energy {
                exploded[i] = true;
                energies[i] = 0;

                for &c in &connections[i] {
                    next_impulses[c][9] += 1;
                    changed = true;
                }

                continue;
            }
        }

        if !changed {
            break;
        }

        (curr_impulses, next_impulses) = (next_impulses, curr_impulses);
    }
}

#[register]
pub fn part1(input: &str) -> u64 {
    let (mut energies, connections, start) = parse(input);
    propagate(&mut energies, &connections, start, 5, u64::MAX);
    energies.iter().copied().sum()
}

#[register]
pub fn part2(input: &str) -> u64 {
    let (mut energies, connections, start) = parse(input);
    propagate(&mut energies, &connections, start, 10, u64::MAX);
    energies.iter().copied().sum()
}

#[register]
pub fn part3(input: &str) -> u64 {
    let (energies, connections, _) = parse(input);
    let mut max = 0;

    for start in 0..energies.len() {
        let mut energies = energies.clone();
        propagate(&mut energies, &connections, start, 10, 1000000);
        max = max.max(energies.iter().copied().sum());
    }

    max
}
