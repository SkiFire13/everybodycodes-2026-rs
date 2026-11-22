use quest_helper::register;

fn visit_shape(
    grid: &[&[u8]],
    seen: &mut [bool],
    x: usize,
    y: usize,
) -> (Vec<(usize, usize)>, [usize; 4]) {
    let w = grid[0].len();

    seen[w * y + x] = true;

    let mut nodes = vec![(x, y)];
    let mut i = 0;
    let (mut minx, mut maxx, mut miny, mut maxy) = (x, x, y, y);

    while let Some(&(x, y)) = nodes.get(i) {
        minx = minx.min(x);
        maxx = maxx.max(x);
        miny = miny.min(y);
        maxy = maxy.max(y);

        let mut add = |x: usize, y: usize| {
            if y < grid.len() && x < grid[y].len() && grid[y][x] == b'#' && !seen[w * y + x] {
                seen[w * y + x] = true;
                nodes.push((x, y));
            }
        };

        add(x.wrapping_add_signed(-1), y);
        add(x.wrapping_add_signed(1), y);
        add(x, y.wrapping_add_signed(-1));
        add(x, y.wrapping_add_signed(1));

        i += 1;
    }

    (nodes, [minx, maxx, miny, maxy])
}

fn find_l_shape(nodes: &[(usize, usize)], bounds: [usize; 4]) -> (usize, usize, usize, usize) {
    let [minx, maxx, miny, maxy] = bounds;

    let (mut left_top, mut left_bot) = (usize::MAX, 0);
    let (mut right_top, mut right_bot) = (usize::MAX, 0);
    let (mut top_left, mut top_right) = (usize::MAX, 0);
    let (mut bot_left, mut bot_right) = (usize::MAX, 0);
    for &(x, y) in nodes {
        if x == minx {
            left_top = left_top.min(y);
            left_bot = left_bot.max(y);
        }
        if x == maxx {
            right_top = right_top.min(y);
            right_bot = right_bot.max(y);
        }
        if y == miny {
            top_left = top_left.min(x);
            top_right = top_right.max(x);
        }
        if y == maxy {
            bot_left = bot_left.min(x);
            bot_right = bot_right.max(x);
        }
    }

    let w_external = usize::min(top_right - top_left, bot_right - bot_left) + 1;
    let h_external = usize::min(left_bot - left_top, right_bot - right_top) + 1;
    let w_internal = maxx + 1 - minx - w_external;
    let h_internal = maxy + 1 - miny - h_external;

    (w_external, h_external, w_internal, h_internal)
}

fn find_hollow_rectangle(
    grid: &[&[u8]],
    nodes: &[(usize, usize)],
    bounds: [usize; 4],
) -> (usize, usize, usize, usize) {
    let [minx, maxx, miny, maxy] = bounds;

    let (mut x_left, mut x_right, mut y_top, mut y_bot) = (usize::MAX, 0, usize::MAX, 0);

    for &(x, y) in nodes {
        if x < maxx && y < maxy && grid[y + 1][x + 1] != b'#' {
            (x_left, y_top) = (x_left, y_top).min((x, y));
        }
        if x > minx && y > miny && grid[y - 1][x - 1] != b'#' {
            (x_right, y_bot) = (x_right, y_bot).max((x, y));
        }
    }

    (x_left, x_right, y_top, y_bot)
}

#[register]
pub fn part1(input: &str) -> u64 {
    let grid = input
        .lines()
        .map(|l| l.trim().as_bytes())
        .collect::<Vec<_>>();
    let (w, h) = (grid[0].len(), grid.len());
    let mut tot = 0;

    for (y, &l) in grid.iter().enumerate() {
        for (x, &b) in l.iter().enumerate() {
            if b != b'#' || (x != 0 && l[x - 1] == b'#') || (y != 0 && grid[y - 1][x] == b'#') {
                continue;
            }

            let mut rw = 1;
            while x + rw < w && l[x + rw] == b'#' {
                rw += 1;
            }

            let mut rh = 1;
            while y + rh < h && grid[y + rh][x] == b'#' {
                rh += 1;
            }

            tot += usize::min(rw, rh);
        }
    }

    tot as u64
}

#[register]
pub fn part2(input: &str) -> u64 {
    let grid = input
        .lines()
        .map(|l| l.trim().as_bytes())
        .collect::<Vec<_>>();
    let (w, h) = (grid[0].len(), grid.len());
    let mut seen = vec![false; w * h];
    let mut tot = 0;

    for (y, &l) in grid.iter().enumerate() {
        for (x, &b) in l.iter().enumerate() {
            if b != b'#' || seen[w * y + x] {
                continue;
            }

            let (nodes, bounds @ [minx, maxx, miny, maxy]) = visit_shape(&grid, &mut seen, x, y);

            // Full rectangle
            if nodes.len() == (maxx + 1 - minx) * (maxy + 1 - miny) {
                tot += usize::min(maxx + 1 - minx, maxy + 1 - miny);
                continue;
            }

            // L shape
            let (h_external, v_external, h_internal, v_internal) = find_l_shape(&nodes, bounds);
            tot += usize::min(
                h_external + usize::min(v_external, h_internal),
                v_external + usize::min(h_external, v_internal),
            );
        }
    }

    tot as u64
}

#[register]
pub fn part3(input: &str) -> u64 {
    let grid = input
        .lines()
        .map(|l| l.trim().as_bytes())
        .collect::<Vec<_>>();
    let (w, h) = (grid[0].len(), grid.len());
    let mut seen = vec![false; w * h];
    let mut tot = 0;

    for (y, &l) in grid.iter().enumerate() {
        for (x, &b) in l.iter().enumerate() {
            if b != b'#' || seen[w * y + x] {
                continue;
            }

            let (nodes, bounds @ [minx, maxx, miny, maxy]) = visit_shape(&grid, &mut seen, x, y);

            // Full rectangle
            if nodes.len() == (maxx + 1 - minx) * (maxy + 1 - miny) {
                tot += usize::min(maxx + 1 - minx, maxy + 1 - miny);
                continue;
            }

            // L shape
            let (w_external, h_external, w_internal, h_internal) = find_l_shape(&nodes, bounds);
            if w_internal != 0 {
                tot += usize::min(
                    w_external + usize::min(h_external, w_internal),
                    h_external + usize::min(w_external, h_internal),
                );
                continue;
            }

            // Hollow rectangle
            let (x_left, x_right, y_top, y_bot) = find_hollow_rectangle(&grid, &nodes, bounds);
            let (h_top, h_bot, h_mid) = (y_top + 1 - miny, maxy + 1 - y_bot, y_bot - y_top - 1);
            let (w_left, w_right, w_mid) =
                (x_left + 1 - minx, maxx + 1 - x_right, x_right - x_left - 1);
            tot += usize::min(
                h_top + h_bot + w_left.min(h_mid) + w_right.min(h_mid),
                w_left + w_right + h_top.min(w_mid) + h_bot.min(w_mid),
            );
        }
    }

    tot as u64
}
