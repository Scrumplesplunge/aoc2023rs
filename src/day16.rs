use std::io;
use std::io::Read;

const MAX_SIZE: usize = 110;
const BUFFER_SIZE: usize = (MAX_SIZE + 1) * MAX_SIZE;

#[derive(Copy, Clone)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

fn next(size: usize, (px, py): (usize, usize), direction: Direction) -> Option<(usize, usize)> {
    match direction {
        Direction::Up => if py > 0 { Some((px, py - 1)) } else { None },
        Direction::Down => if py < size - 1 { Some((px, py + 1)) } else { None },
        Direction::Left => if px > 0 { Some((px - 1, py)) } else { None },
        Direction::Right => if px < size - 1 { Some((px + 1, py)) } else { None },
    }
}

fn energise(
    grid: &[u8],
    size: usize,
    seen: &mut [[bool; 4]],
    (mut px, mut py): (usize, usize),
    mut direction: Direction,
) {
    loop {
        let i = py * size + px;
        if seen[i][direction as usize] { return }
        seen[i][direction as usize] = true;
        match (grid[i], direction) {
            (b'.', _) => {}
            (b'\\', Direction::Up) => direction = Direction::Left,
            (b'\\', Direction::Down) => direction = Direction::Right,
            (b'\\', Direction::Left) => direction = Direction::Up,
            (b'\\', Direction::Right) => direction = Direction::Down,
            (b'/', Direction::Up) => direction = Direction::Right,
            (b'/', Direction::Down) => direction = Direction::Left,
            (b'/', Direction::Left) => direction = Direction::Down,
            (b'/', Direction::Right) => direction = Direction::Up,
            (b'|', Direction::Up | Direction::Down) => {}
            (b'|', Direction::Left | Direction::Right) => {
                if let Some((nx, ny)) = next(size, (px, py), Direction::Up) {
                    energise(grid, size, seen, (nx, ny), Direction::Up);
                }
                direction = Direction::Down;
            }
            (b'-', Direction::Up | Direction::Down) => {
                if let Some((nx, ny)) = next(size, (px, py), Direction::Left) {
                    energise(grid, size, seen, (nx, ny), Direction::Left);
                }
                direction = Direction::Right;
            }
            (b'-', Direction::Left | Direction::Right) => {}
            _ => panic!("bad grid: ({}, {})\n", grid[i], direction as u8),
        }
        if let Some((nx, ny)) = next(size, (px, py), direction) {
            (px, py) = (nx, ny);
        } else {
            return;
        }
    }
}

fn energised(grid: &[u8], size: usize, start: (usize, usize), direction: Direction) -> usize {
    let mut seen = [[false; 4]; BUFFER_SIZE];
    energise(grid, size, &mut seen, start, direction);
    return seen.iter().filter(|d| d.iter().any(|s| *s)).count();
}

fn main() {
    let mut buffer = [0; BUFFER_SIZE];
    let length = io::stdin().read(&mut buffer).unwrap();
    if length == 0 || buffer[length - 1] != b'\n' { panic!("no newline") }
    let size = buffer[0..length].iter().position(|b| *b == b'\n').unwrap();
    if size > MAX_SIZE { panic!("too wide") }
    // Verify that the input is a grid.
    let mut height = 0;
    for line in buffer[0..length - 1].split(|b| *b == b'\n') {
        if line.len() != size { panic!("not a grid") }
        height += 1;
    }
    if height != size { panic!("not square") }
    // Align the grid.
    for y in 0..size {
        let from = y * (size + 1);
        buffer.copy_within(from..from + size, y * size);
    }

    let part1 = energised(&buffer, size, (0, 0), Direction::Right);
    let mut part2 = 0;
    for i in 0..size {
        for (start, direction) in [
            ((0, i), Direction::Right),
            ((i, 0), Direction::Down),
            ((size - 1, i), Direction::Left),
            ((i, size - 1), Direction::Up),
        ] {
            let e = energised(&buffer, size, start, direction);
            if e > part2 { part2 = e }
        }
    }
    print!("{}\n{}\n", part1, part2);
}
