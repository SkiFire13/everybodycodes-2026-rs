use std::{cmp::Reverse, collections::BinaryHeap};

use everybodycodes_helper::register;

const NEIGHBOURS_1: &[isize] = &[-1, 1];
const NEIGHBOURS_3: &[isize] = &[-3, -2, -1, 1, 2, 3];

fn parse(input: &str) -> Vec<Vec<usize>> {
    let input = input.strip_prefix("portals: ").unwrap();
    let (size, rest) = input.split_once("\n\n").unwrap();
    let size = size.parse::<usize>().unwrap();
    let mut edges = vec![Vec::new(); size];
    for line in rest.lines() {
        let (u, v) = line.split_once(" ").unwrap();
        let u = u.parse::<usize>().unwrap() - 1;
        let v = v.parse::<usize>().unwrap() - 1;
        edges[u].push(v);
        edges[v].push(u);
    }
    edges
}

fn find_distances(edges: &[Vec<usize>], src: usize, neighbours: &[isize]) -> Vec<u64> {
    let mut queue = BinaryHeap::from([(Reverse(0), src)]);
    let mut distances = vec![u64::MAX; edges.len()];
    distances[src] = 0;

    while let Some((Reverse(d), src)) = queue.pop() {
        if distances[src] != d {
            continue;
        }

        let neighbours = neighbours
            .iter()
            .map(|&dn| (src + edges.len()).wrapping_add_signed(dn) % edges.len());

        for dst in edges[src].iter().copied().chain(neighbours) {
            let nd = d + 1;
            if distances[dst] > nd {
                distances[dst] = nd;
                queue.push((Reverse(nd), dst));
            }
        }
    }

    distances
}

fn find_tot_distance(edges: &[Vec<usize>], neighbours: &[isize]) -> u64 {
    let mut tot = 0;

    for src in 0..edges.len() {
        let distances = find_distances(edges, src, neighbours);
        tot += distances.into_iter().sum::<u64>();
    }

    tot
}

const DISPS: [[(usize, usize); 4]; 105] = {
    let mut disps = [[(0, 0); 4]; 105];
    let mut di = 0;

    let mut took = [false; 8];

    macro_rules! combine_pair {
        ($i0:literal = $p0:ident, $i1:literal = $p1:ident, || { $($rest:tt)* }) => {
            #[allow(unused)]
            {
                let mut $p0 = 0;
                while took[$p0] { $p0 += 1; }
                took[$p0] = true;
                let mut $p1 = $p0 + 1;
                while $p1 < 8 {
                    if !took[$p1] {
                        took[$p1] = true;
                        $($rest)*;
                        took[$p1] = false;
                    }
                    $p1 += 1;
                }
                took[$p0] = false;
            }
        };
    }

    combine_pair!(0 = p0, 1 = p1, || {
        combine_pair!(2 = p2, 3 = p3, || {
            combine_pair!(4 = p4, 5 = p5, || {
                combine_pair!(6 = p6, 7 = p7, || {
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
    let edges = parse(input);

    let n = edges.len() as u64;
    let base = n
        * (0..edges.len() as u64)
            .map(|e| u64::min(e, n - e))
            .sum::<u64>();

    let tot = find_tot_distance(&edges, NEIGHBOURS_1);

    base - tot
}

#[register]
pub fn part2(input: &str) -> u64 {
    let mut edges = parse(input);

    let no_edges = vec![Vec::new(); edges.len()];
    let base = find_tot_distance(&no_edges, NEIGHBOURS_3);

    let mut best = base;

    for u in 0..edges.len() - 1 {
        for v in u + 1..edges.len() {
            if edges[u].contains(&v) {
                continue;
            }

            edges[u].push(v);
            edges[v].push(u);

            let tot = find_tot_distance(&edges, NEIGHBOURS_3);
            best = best.min(tot);

            edges[u].pop();
            edges[v].pop();
        }
    }

    base - best
}

#[register]
pub fn part3(input: &str) -> u64 {
    let mut edges = parse(input);

    let no_edges = vec![Vec::new(); edges.len()];
    let base = find_tot_distance(&no_edges, NEIGHBOURS_3);

    let mut best = base;

    let n = edges.len();
    for s in 0..n / 8 {
        let mut conns = [0; 8];
        for i in 0..4 {
            conns[i] = s + i * (n / 8);
            conns[4 + i] = conns[i] + n / 2;
        }

        for disp in &DISPS {
            for &(srci, dsti) in disp {
                let (u, v) = (conns[srci], conns[dsti]);

                edges[u].push(v);
                edges[v].push(u);
            }

            let tot = find_tot_distance(&edges, NEIGHBOURS_3);
            best = best.min(tot);

            for u in conns {
                edges[u].pop();
            }
        }
    }

    base - best
}
