use everybodycodes_helper::register;

fn parse(input: &str) -> (usize, Vec<(usize, usize)>) {
    let input = input.strip_prefix("portals: ").unwrap();
    let (size, rest) = input.split_once("\n\n").unwrap();
    let size = size.parse::<usize>().unwrap();
    let mut edges = Vec::new();
    for line in rest.lines() {
        let (u, v) = line.split_once(" ").unwrap();
        let u = u.parse::<usize>().unwrap() - 1;
        let v = v.parse::<usize>().unwrap() - 1;
        edges.push((u, v));
    }
    (size, edges)
}

fn find_all_distances(edges: &[(usize, usize)], n: usize, neighbours: usize) -> Vec<u64> {
    let mut distances = vec![n as u64; n * n];

    for i in 0..n {
        distances[n * i + i] = 0;
    }

    for i in 0..n - 1 {
        for j in i + 1..n {
            if j - i <= neighbours || n - (j - i) <= neighbours {
                distances[n * i + j] = 1;
                distances[n * j + i] = 1;
            }
        }
    }

    for &(u, v) in edges {
        distances[n * u + v] = 1;
        distances[n * v + u] = 1;
    }

    for m in 0..n {
        propagate_middle(&mut distances, n, m);
    }

    distances
}

fn propagate_middle(distances: &mut [u64], n: usize, m: usize) {
    for s in 0..n - 1 {
        let sm = distances[n * s + m];
        for e in s + 1..n {
            let me = distances[n * m + e];
            let se = distances[n * s + e];
            if sm + me < se {
                distances[n * s + e] = sm + me;
                distances[n * e + s] = sm + me;
            }
        }
    }
}

fn distances_add_edge(distances: &mut [u64], n: usize, u: usize, v: usize) {
    if distances[n * u + v] == 1 {
        return;
    }

    distances[n * u + v] = 1;
    distances[n * v + u] = 1;

    propagate_middle(distances, n, u);
    propagate_middle(distances, n, v);
}

const DISPS: [[(usize, usize); 4]; 105] = {
    let mut disps = [[(0, 0); 4]; 105];
    let mut di = 0;

    let mut took = [false; 8];

    macro_rules! combine_pair {
        (|$p0:ident, $p1:ident| { $($rest:tt)* }) => {
            let mut $p0 = 0;
            while took[$p0] { $p0 += 1; }
            took[$p0] = true;
            let mut $p1 = $p0 + 1;
            while $p1 < 8 {
                if !took[$p1] {
                    took[$p1] = true;
                    $($rest)*
                    took[$p1] = false;
                }
                $p1 += 1;
            }
            took[$p0] = false;
            _ = took;
        };
    }

    combine_pair!(|p0, p1| {
        combine_pair!(|p2, p3| {
            combine_pair!(|p4, p5| {
                combine_pair!(|p6, p7| {
                    disps[di] = [(p0, p1), (p2, p3), (p4, p5), (p6, p7)];
                    di += 1;
                });
            });
        });
    });

    disps
};

#[register]
pub fn part1(input: &str) -> u64 {
    let (n, edges) = parse(input);

    let base = n * (0..n).map(|e| usize::min(e, n - e)).sum::<usize>();

    let tot = find_all_distances(&edges, n, 1).into_iter().sum::<u64>();

    base as u64 - tot
}

#[register]
pub fn part2(input: &str) -> u64 {
    let (n, edges) = parse(input);

    let base = find_all_distances(&[], n, 3).into_iter().sum::<u64>();
    let distances = find_all_distances(&edges, n, 3);
    let mut best = base;

    for u in 0..n - 1 {
        for v in u + 1..n {
            if edges.contains(&(u, v)) || edges.contains(&(v, u)) {
                continue;
            }

            let mut distances = distances.clone();
            distances_add_edge(&mut distances, n, u, v);
            best = best.min(distances.into_iter().sum::<u64>());
        }
    }

    base - best
}

#[register]
pub fn part3(input: &str) -> u64 {
    let (n, edges) = parse(input);

    let base = find_all_distances(&[], n, 3).into_iter().sum::<u64>();

    let distances = find_all_distances(&edges, n, 3);
    let mut best = base;

    for s in 0..n / 8 {
        let mut conns = [0; 8];
        for i in 0..4 {
            conns[i] = s + i * (n / 8);
            conns[4 + i] = conns[i] + n / 2;
        }

        for disp in &DISPS {
            let mut distances = distances.clone();

            for &(srci, dsti) in disp {
                let (u, v) = (conns[srci], conns[dsti]);
                distances_add_edge(&mut distances, n, u, v);
            }

            best = best.min(distances.into_iter().sum::<u64>());
        }
    }

    base - best
}
