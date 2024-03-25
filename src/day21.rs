use std::io;
use std::io::Read;

const SIZE: usize = 131;
type Grid<T> = [[T; SIZE]; SIZE];

fn read_input() -> Grid<bool> {
    let mut buffer = [0; (SIZE + 1) * SIZE];
    let len = io::stdin().read(&mut buffer).unwrap();

    // The input should be a perfect grid occupying the whole buffer.
    if len != buffer.len() { panic!("truncated") }
    let mut result = [[false; SIZE]; SIZE];
    const CENTER: usize = SIZE / 2 * (SIZE + 2);
    if buffer[CENTER] != b'S' { panic!("center is not S") }
    buffer[CENTER] = b'.';
    for (line, row) in buffer.split(|b| *b == b'\n').zip(result.iter_mut()) {
        if line.len() != row.len() { panic!("not a grid") }
        for (c, cell) in line.iter().zip(row.iter_mut()) {
            *cell = match c {
                b'.' => false,
                b'#' => true,
                _ => panic!("bad grid"),
            };
        }
    }
    return result;
}

fn step(grid: &Grid<bool>, from: &Grid<bool>, to: &mut Grid<bool>) {
    for y in 0..SIZE {
        for x in 0..SIZE {
            if !grid[y][x] {
                let up = if y > 0 { from[y - 1][x] } else { false };
                let down = if y < SIZE - 1 { from[y + 1][x] } else { false };
                let left = if x > 0 { from[y][x - 1] } else { false };
                let right = if x < SIZE - 1 { from[y][x + 1] } else { false };
                to[y][x] = up | down | left | right;
            }
        }
    }
}

fn part2(grid: &Grid<bool>, at130: &Grid<bool>, at131: &Grid<bool>) -> usize {
    // The input grid has some specific properties which ensure that there is symmetry in how the
    // visited set expands outwards in the tiled grid:
    //
    //   * There's an unobstructed straight line path from `S` in all four cardinal directions, all
    //     the way to the edge, plus an empty border all the way around the outside edge. This
    //     guarantees that we can always reach any part of the edge in the minimum possible number
    //     of steps.
    //   * There's a big diamond of uncluttered space connecting the centers of each edge. This,
    //     combined with the sparsity of obstacles in the grid, seems to be sufficient to ensure
    //     that the exploration frontier always forms a perfectly straight line down this channel.
    //   * The required step count is 26501365, which is `100 * 2023 * SIZE + SIZE / 2`, so the
    //     perimeter of the explored space will be in the diamond-shaped channel.
    //
    //                                                             +--+--+--+
    // From this, we can infer that the final explored             |  |/\|  |
    // space will form a perfect diamond of tiles:                 | /|##|\ |
    //                                                          +--+--+--+--+--+
    //    * Two kinds of fully-explored tiles                   |  |/#|##|#\|  |
    //      (it alternates, and the width is odd).              | /|##|##|##|\ |
    //    * For each corner entry point, two kinds              +--+--+--+--+--+
    //      of diagonal tile (out of phase by 132               |/#|##|##|##|#\|
    //      steps).                                             |\#|##|##|##|#/|
    //    * For each edge-center entry point, a                 +--+--+--+--+--+
    //      pointy tile.                                        | \|##|##|##|/ |
    //                                                          |  |\#|##|#/|  |
    // The positions which can be reached in 26501365           +--+--+--+--+--+
    // steps will be all positions within this diamond             | \|##|/ |
    // which are an odd distance away from the origin.             |  |\/|  |
    // These can be calculated directly from the grid:             +--+--+--+
    //
    //   * Count spaces `(x, y)` in the four corners (split along the diagonals) which have an odd
    //     value for `x + y` and are not walls from the grid.
    //   * Construct the counts for each of the aforementioned tile combinations using combinations
    //     of these counts.
    let mut tl = [0, 0];
    let mut tr = [0, 0];
    let mut bl = [0, 0];
    let mut br = [0, 0];
    let mut total = [0, 0];
    for y in 0..SIZE {
        for x in 0..SIZE {
            if grid[y][x] { continue }
            let ix = SIZE - x - 1;
            let iy = SIZE - y - 1;
            let is_tl = x + y < SIZE / 2;
            let is_tr = ix + y < SIZE / 2;
            let is_bl = x + iy < SIZE / 2;
            let is_br = ix + iy < SIZE / 2;
            if at130[y][x] {
                total[0] += 1;
                if is_tl { tl[0] += 1 }
                if is_tr { tr[0] += 1 }
                if is_bl { bl[0] += 1 }
                if is_br { br[0] += 1 }
            }
            if at131[y][x] {
                total[1] += 1;
                if is_tl { tl[1] += 1 }
                if is_tr { tr[1] += 1 }
                if is_bl { bl[1] += 1 }
                if is_br { br[1] += 1 }
            }
        }
    }
    const N: usize = 26501365;
    const INDEX: usize = (N - SIZE) / SIZE;
    // The number of full tiles grows according to this series (derived by inspection):
    //
    //   INDEX  0  1  2  3  4  5
    //   A      0  4  4 16 16 36  NUM_FULL_A(INDEX)
    //   B      1  1  9  9 25 25  NUM_FULL_B(INDEX)
    const NUM_FULL_A: usize = ((INDEX + 1) / 2 * 2) * ((INDEX + 1) / 2 * 2);
    const NUM_FULL_B: usize = (INDEX / 2 * 2 + 1) * (INDEX / 2 * 2 + 1);
    let full = NUM_FULL_A * total[0] + NUM_FULL_B * total[1];
    // Each pointy tile consists of odd positions, excluding the two corners on the opposite side.
    let left = total[1] - tr[1] - br[1];
    let right = total[1] - tl[1] - bl[1];
    let top = total[1] - bl[1] - br[1];
    let bottom = total[1] - tl[1] - tr[1];
    let points = left + right + top + bottom;
    // Each diagonal has two types of tile. The number of each scales linearly with the iteration.
    const NUM_SLOPE_A: usize = (N - 1) / (2 * SIZE) * 2;
    const NUM_SLOPE_B: usize = 1 + (N - SIZE - 1) / (2 * SIZE) * 2;
    let tla = tl[0];
    let tra = tr[0];
    let bla = bl[0];
    let bra = br[0];
    let tlb = total[1] - br[1];
    let trb = total[1] - bl[1];
    let blb = total[1] - tr[1];
    let brb = total[1] - tl[1];
    let slopes = NUM_SLOPE_A * (tla + tra + bla + bra) + NUM_SLOPE_B * (tlb + trb + blb + brb);
    return full + points + slopes;
}

fn main() {
    let grid = read_input();

    // Part 1: Iterate until 64 steps and then count the cells.
    let mut a = [[false; SIZE]; SIZE];
    let mut b = a;
    a[65][65] = true;
    for i in 0..64 {
        let (from, to) = if i % 2 == 0 {
            (&a, &mut b)
        } else {
            (&b, &mut a)
        };
        step(&grid, from, to);
    }
    let part1 = a.iter().flatten().filter(|x| **x).count();

    // Part 2: Infer the result after `100 * 2023 * SIZE + SIZE / 2` steps. To do this, we need the
    // state of the grid after 130 steps and 131 steps. We can construct the final state by
    // inspection.
    for i in 64..132 {
        let (from, to) = if i % 2 == 0 {
            (&a, &mut b)
        } else {
            (&b, &mut a)
        };
        step(&grid, from, to);
    }
    print!("{}\n{}\n", part1, part2(&grid, &a, &b));
}
