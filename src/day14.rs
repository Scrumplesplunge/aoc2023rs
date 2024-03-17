use std::io;
use std::io::Read;

const MAX_SIZE: usize = 100;
const ROW: usize = 128;

fn load_north(grid: &[u8], size: usize) -> usize {
    let mut total = 0;
    for y in 0..size {
        let row = &grid[y * ROW..][..size];
        total += (size - y) * row.iter().filter(|b| **b == b'O').count();
    }
    return total;
}

fn load_east(grid: &[u8], size: usize) -> usize {
    let mut total = 0;
    for y in 0..size {
        let row = &grid[y * ROW..][..size];
        for x in 0..size {
            if row[x] == b'O' { total += x + 1 }
        }
    }
    return total;
}

fn tumble(input: &[u8], output: &mut [u8], size: usize) {
    for x in 0..size {
        let out = x * ROW + size - 1;
        let mut i = 0;
        for y in 0..size {
            match input[y * ROW + x] {
                b'.' => {
                    output[out - y] = b'.';
                }
                b'O' => {
                    output[out - y] = b'.';
                    output[out - i] = b'O';
                    i += 1;
                }
                b'#' => {
                    output[out - y] = b'#';
                    i = y + 1;
                }
                _ => panic!("bad contents"),
            }
        }
    }
}

fn cycle(a: &mut [u8], b: &mut [u8], size: usize) {
    tumble(a, b, size);
    tumble(b, a, size);
    tumble(a, b, size);
    tumble(b, a, size);
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

    let mut buffer_2 = [0; MAX_SIZE * ROW];

    // Roll all the stones North.
    tumble(&buffer, &mut buffer_2, size);

    let part1 = load_east(&buffer_2, size);
    tumble(&buffer_2, &mut buffer, size);
    tumble(&buffer, &mut buffer_2, size);
    tumble(&buffer_2, &mut buffer, size);

    let mut hare = buffer;
    let mut hare_2 = [0; MAX_SIZE * ROW];
    cycle(&mut hare, &mut hare_2, size);

    for i in 1..1000000000 {
        if buffer == hare {
            // Cycles i and 2i are the same, so the 1e9'th cycle will look the
            // same as the (i + 1e9 % i)th.
            for _ in 0 .. 1000000000 % i {
                cycle(&mut buffer, &mut buffer_2, size);
            }
            break;
        }

        cycle(&mut buffer, &mut buffer_2, size);
        cycle(&mut hare, &mut hare_2, size);
        cycle(&mut hare, &mut hare_2, size);
    }

    let part2 = load_north(&buffer, size);
    print!("{}\n{}\n", part1, part2);
}
