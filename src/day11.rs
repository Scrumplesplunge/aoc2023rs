use std::io;
use std::io::Read;

struct Input<'a> {
    grid: &'a[u8],
    size: (usize, usize),
}

fn is_line(input: &[u8]) -> bool {
    match input {
        [line @ .., b'\n'] => line.iter().all(|b| *b != b'\n'),
        _ => false,
    }
}

fn read_input<'a>(buffer: &'a mut[u8]) -> Input<'a> {
    let length = io::stdin().read(buffer).unwrap();
    let input = &buffer[0..length];

    // Validate the input.
    if !input.is_ascii() { panic!("Input is not ASCII.") }
    let width = input.iter().position(|b| *b == b'\n').unwrap();
    if !input.chunks(width + 1).all(is_line) {
        panic!("Not all lines are the same length.");
    }
    let height = length / (width + 1);
    return Input{grid: input, size: (width, height)};
}

fn distance((ax, ay): (usize, usize), (bx, by): (usize, usize)) -> usize {
    let dx = if ax < bx { bx - ax } else { ax - bx };
    let dy = if ay < by { by - ay } else { ay - by };
    return dx + dy;
}

fn main() {
    let mut buffer = [0; 141 * 140];
    let input = read_input(&mut buffer);
    let (w, h) = input.size;

    // Identify all empty columns.
    let mut part1_xs = [0; 140];
    let mut part2_xs = [0; 140];
    {
        let mut part1_ox = 0;
        let mut part2_ox = 0;
        for x in 0 .. w {
            let is_empty_column = (0 .. h)
                .map(|y| input.grid[y * (w + 1) + x])
                .all(|c| c == b'.');
            if is_empty_column {
                part1_ox += 2;
                part2_ox += 1000000;
            } else {
                part1_ox += 1;
                part2_ox += 1;
            }
            part1_xs[x] = part1_ox;
            part2_xs[x] = part2_ox;
        }
    }

    // Identify all empty rows.
    let mut part1_ys = [0; 140];
    let mut part2_ys = [0; 140];
    {
        let mut part1_oy = 0;
        let mut part2_oy = 0;
        for y in 0 .. h {
            let is_empty_row = (0 .. w)
                .map(|x| input.grid[y * (w + 1) + x])
                .all(|c| c == b'.');
            if is_empty_row {
                part1_oy += 2;
                part2_oy += 1000000;
            } else {
                part1_oy += 1;
                part2_oy += 1;
            }
            part1_ys[y] = part1_oy;
            part2_ys[y] = part2_oy;
        }
    }

    // Identify all stars, mapping their coordinates.
    let mut star_buffer = [((0, 0), (0, 0)); 512];
    let mut num_stars = 0;
    for y in 0 .. h {
        for x in 0 .. w {
            if input.grid[y * (w + 1) + x] == b'#' {
                let part1 = (part1_xs[x], part1_ys[y]);
                let part2 = (part2_xs[x], part2_ys[y]);
                star_buffer[num_stars] = (part1, part2);
                num_stars += 1;
            }
        }
    }
    let stars = &star_buffer[0..num_stars];

    // Calculate all the distance pairs.
    let mut part1 = 0;
    let mut part2 = 0;
    for i in 0 .. num_stars {
        for j in i + 1 .. num_stars {
            part1 += distance(stars[i].0, stars[j].0);
            part2 += distance(stars[i].1, stars[j].1);
        }
    }
    print!("{}\n{}\n", part1, part2);
}
