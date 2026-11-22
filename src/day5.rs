use quest_helper::register;

fn parse(input: &str) -> (Vec<bool>, usize, usize) {
    let mut light = vec![false; input.len()];

    let mut w = 0;
    let mut h = 0;

    for (y, line) in input.lines().enumerate() {
        w = line.len();
        h += 1;

        for (x, n) in line.bytes().enumerate() {
            light[w * y + x] = n == b'1';
        }
    }

    (light, w, h)
}

fn around(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    static AROUND: [isize; 3] = [-1, 0, 1];
    AROUND
        .iter()
        .flat_map(|&dy| AROUND.iter().map(move |&dx| (dx, dy)))
        .filter(|&(dx, dy)| dx != 0 || dy != 0)
        .map(move |(dx, dy)| (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy)))
}

fn around25(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    static AROUND2: [isize; 5] = [-2, -1, 0, 1, 2];
    AROUND2
        .iter()
        .flat_map(|&dy| AROUND2.iter().map(move |&dx| (dx, dy)))
        .filter(|&(dx, dy)| dx != 0 || dy != 0)
        .filter(|&(dx, dy)| dx.abs() + dy.abs() != 4)
        .map(move |(dx, dy)| (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy)))
}

fn in_bound(w: usize, h: usize) -> impl Fn(&(usize, usize)) -> bool {
    move |&(x, y)| x < w && y < h
}

fn find_land(light: &[bool], w: usize, h: usize) -> Vec<bool> {
    let (cx, cy) = (w / 2, h / 2);

    let mut land = vec![false; w * h];

    let mut queue = vec![(cx, cy)];

    while let Some((x, y)) = queue.pop() {
        if land[w * y + x] {
            continue;
        }

        let valid = (x == cx && y == cy)
            || around(x, y)
                .filter(in_bound(w, h))
                .filter(|&(ax, ay)| land[w * ay + ax] || light[w * ay + ax])
                .count()
                >= 5;

        if !valid {
            continue;
        }

        land[w * y + x] = true;

        for (ax, ay) in around(x, y).filter(in_bound(w, h)) {
            queue.push((ax, ay));
        }
    }

    land
}

fn find_sand(land: &[bool], w: usize, h: usize) -> Vec<bool> {
    let mut sand = vec![false; land.len()];
    let mut queue = Vec::new();

    let extend = |x: usize, y: usize, sand: &[bool], queue: &mut Vec<(usize, usize)>| {
        for (ax, ay) in around(x, y).filter(in_bound(w, h)) {
            if land[w * ay + ax] && !sand[w * ay + ax] {
                queue.push((ax, ay));
            }
        }
    };

    for y in 0..h {
        for x in 0..w {
            if land[w * y + x] {
                let near_water = around(x, y)
                    .filter(in_bound(w, h))
                    .filter(|&(x, y)| land[w * y + x])
                    .count()
                    != 8;

                if near_water {
                    sand[w * y + x] = true;
                    extend(x, y, &sand, &mut queue);
                }
            }
        }
    }

    while let Some((x, y)) = queue.pop() {
        if sand[w * y + x] {
            continue;
        }

        let near_sand = around(x, y)
            .filter(in_bound(w, h))
            .filter(|&(x, y)| sand[w * y + x])
            .count()
            >= 5;

        if near_sand {
            sand[w * y + x] = true;
            extend(x, y, &sand, &mut queue);
        }
    }

    sand
}

fn find_initial_trees(land: &[bool], sand: &[bool], w: usize, h: usize) -> Vec<bool> {
    let mut trees = vec![false; land.len()];

    for y in 0..h {
        for x in 0..w {
            if !land[w * y + x] || sand[w * y + x] {
                continue;
            }

            let is_tree = around25(x, y)
                .filter(in_bound(w, h))
                .filter(|&(ax, ay)| land[w * ay + ax] && !sand[w * ay + ax])
                .count()
                == 20;

            if is_tree {
                trees[w * y + x] = true;
            }
        }
    }

    trees
}

fn find_more_trees(land: &[bool], sand: &[bool], trees: &[bool], w: usize, h: usize) -> Vec<bool> {
    let mut new_trees = vec![false; land.len()];

    for y in 0..h {
        for x in 0..w {
            if !land[w * y + x] || sand[w * y + x] || trees[w * y + x] {
                continue;
            }

            let near_sand = around(x, y).any(|(x, y)| sand[w * y + x]);

            if near_sand {
                continue;
            }

            let near_trees = around(x, y).filter(|&(x, y)| trees[w * y + x]).count() >= 4;

            if near_trees {
                new_trees[w * y + x] = true;
            }
        }
    }

    new_trees
}

#[register]
pub fn part1(input: &str) -> u64 {
    let (light, w, h) = parse(input);
    let land = find_land(&light, w, h);
    land.iter().filter(|&&b| b).count() as u64
}

#[register]
pub fn part2(input: &str) -> u64 {
    let (light, w, h) = parse(input);
    let land = find_land(&light, w, h);
    let sand = find_sand(&land, w, h);
    sand.iter().filter(|&&b| b).count() as u64
}

#[register]
pub fn part3(input: &str) -> u64 {
    let (light, w, h) = parse(input);
    let land = find_land(&light, w, h);
    let sand = find_sand(&land, w, h);
    let mut trees = find_initial_trees(&land, &sand, w, h);
    let new_trees_1 = find_more_trees(&land, &sand, &trees, w, h);
    (0..trees.len()).for_each(|i| trees[i] |= new_trees_1[i]);
    let new_trees_2 = find_more_trees(&land, &sand, &trees, w, h);
    trees.iter().chain(&new_trees_2).filter(|&&b| b).count() as u64
}
