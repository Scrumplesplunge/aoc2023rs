use std::io;
use std::io::Read;

const MAX_SIZE: usize = 100;
const ROW: usize = 128;

fn load(grid: &[u8], size: usize) -> usize {
    let mut total = 0;
    for y in 0..size {
        let row = &grid[y * ROW..][..size];
        total += (size - y) * row.iter().filter(|b| **b == b'O').count();
    }
    return total;
}

fn roll_n(grid: &mut [u8], size: usize) {
    for x in 0..size {
        let mut o = x;
        for y in 0..size {
            let i = y * ROW + x;
            match grid[i] {
                b'.' => {}
                b'O' => {
                    grid[i] = b'.';
                    grid[o] = b'O';
                    o += ROW;
                }
                b'#' => {
                    o = (y + 1) * ROW + x;
                }
                _ => panic!("bad contents"),
            }
        }
    }
}

fn roll_e(grid: &mut [u8], size: usize) {
    for y in 0..size {
        let mut o = size;
        let row = &mut grid[y * ROW..][..size];
        for x in (0..size).rev() {
            match row[x] {
                b'.' => {}
                b'O' => {
                    o -= 1;
                    row[x] = b'.';
                    row[o] = b'O';
                }
                b'#' => {
                    o = x;
                }
                _ => panic!("bad contents"),
            }
        }
    }
}

fn roll_s(grid: &mut [u8], size: usize) {
    for x in 0..size {
        let mut o = size * ROW + x;
        for y in (0..size).rev() {
            let i = y * ROW + x;
            match grid[i] {
                b'.' => {}
                b'O' => {
                    o -= ROW;
                    grid[i] = b'.';
                    grid[o] = b'O';
                }
                b'#' => {
                    o = y * ROW + x;
                }
                _ => panic!("bad contents"),
            }
        }
    }
}

fn roll_w(grid: &mut [u8], size: usize) {
    for y in 0..size {
        let mut o = 0;
        let row = &mut grid[y * ROW..][..size];
        for x in 0..size {
            match row[x] {
                b'.' => {}
                b'O' => {
                    row[x] = b'.';
                    row[o] = b'O';
                    o += 1;
                }
                b'#' => {
                    o = x + 1;
                }
                _ => panic!("bad contents"),
            }
        }
    }
}

fn cycle(grid: &mut [u8], size: usize) {
    roll_n(grid, size);
    roll_w(grid, size);
    roll_s(grid, size);
    roll_e(grid, size);
}

fn main() {
    let mut buffer = [0; MAX_SIZE * ROW];
    let length = io::stdin().read(&mut buffer).unwrap();
    if length == 0 || buffer[length - 1] != b'\n' {
        panic!("no newline")
    }
    let size = buffer[0..length].iter().position(|b| *b == b'\n').unwrap();
    if size > MAX_SIZE {
        panic!("too wide")
    }
    // Verify that the input is a grid.
    let mut height = 0;
    for line in buffer[0..length - 1].split(|b| *b == b'\n') {
        if line.len() != size {
            panic!("not a grid")
        }
        height += 1;
    }
    if height != size {
        panic!("not square")
    }
    // Align the grid.
    for y in (0..size).rev() {
        let from = y * (size + 1);
        buffer.copy_within(from..from + size, y * ROW);
    }

    // Roll all the stones North.
    roll_n(&mut buffer, size);
    let part1 = load(&buffer, size);
    roll_w(&mut buffer, size);
    roll_s(&mut buffer, size);
    roll_e(&mut buffer, size);

    let mut hare = buffer;
    cycle(&mut hare, size);

    for i in 1..1000000000 {
        if buffer == hare {
            // Cycles i and 2i are the same, so the 1e9'th cycle will look the
            // same as the (i + 1e9 % i)th.
            for _ in 0 .. 1000000000 % i {
                cycle(&mut buffer, size);
            }
            break;
        }

        cycle(&mut buffer, size);
        cycle(&mut hare, size);
        cycle(&mut hare, size);
    }

    let part2 = load(&buffer, size);
    print!("{}\n{}\n", part1, part2);
}
