use std::io;
use std::io::Read;

struct Input<'a> {
    grid: &'a[u8],
    size: (i32, i32),
    start: (i32, i32),
}

impl<'a> Input<'a> {
    fn index(&self, x: i32, y: i32) -> usize {
        return (y * (self.size.0 + 1) + x) as usize;
    }
    fn cell(&self, x: i32, y: i32) -> u8 {
        return self.grid[self.index(x, y)];
    }
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
    let start_index = input.iter().position(|b| *b == b'S').unwrap();
    let start_x = (start_index % (width + 1)) as i32;
    let start_y = (start_index / (width + 1)) as i32;

    return Input{
        grid: input,
        size: (width as i32, height as i32),
        start: (start_x, start_y),
    };
}

struct PipeIterator<'a, 'b> {
    input: &'a Input<'b>,
    position: (i32, i32),
    direction: (i32, i32),
}

// Iterates over positions in a pipe. Items are (from_direction, position),
// where from_direction is the direction we were going when we entered the
// current position.
impl<'a, 'b> Iterator for PipeIterator<'a, 'b> {
    type Item = ((i32, i32), (i32, i32));

    fn next(&mut self) -> Option<((i32, i32), (i32, i32))> {
        if self.direction == (0, 0) { return None }
        let (w, h) = self.input.size;
        let (px, py) = (self.position.0 + self.direction.0,
                        self.position.1 + self.direction.1);
        if px < 0 || w <= px || py < 0 || h <= py { return None }
        let cell = self.input.cell(px, py);
        let (dx, dy) = match (self.direction, cell) {
            ((1, 0), b'-') => (1, 0),
            ((1, 0), b'J') => (0, -1),
            ((1, 0), b'7') => (0, 1),
            ((-1, 0), b'-') => (-1, 0),
            ((-1, 0), b'L') => (0, -1),
            ((-1, 0), b'F') => (0, 1),
            ((0, 1), b'|') => (0, 1),
            ((0, 1), b'J') => (-1, 0),
            ((0, 1), b'L') => (1, 0),
            ((0, -1), b'|') => (0, -1),
            ((0, -1), b'7') => (-1, 0),
            ((0, -1), b'F') => (1, 0),
            _ => (0, 0),
        };
        let result = Some((self.direction, (px, py)));
        self.position = (px, py);
        self.direction = (dx, dy);
        return result;
    }
}

fn follow_pipe<'a, 'b>(
    input: &'a Input<'b>,
    start_direction: (i32, i32),
) -> PipeIterator<'a, 'b> {
    return PipeIterator{
        input: input,
        position: input.start,
        direction: start_direction,
    };
}

// Finds the looped pipe connected to the start position.
// Returns (steps, from_direction, start_direction), where:
//   * steps is the number of steps taken around the loop.
//   * from_direction is the direction we were going when we re-entered the
//     start position.
//   * start_direction is the direction we went when we first left the start
//     position.
// From from_direction and start_direction, we can infer what piece of pipe is
// in the start position.
fn find_loop(input: &Input) -> (u32, (i32, i32), (i32, i32)) {
    for start_direction in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let mut steps = 0;
        for (from_direction, position) in follow_pipe(input, start_direction) {
            steps += 1;
            if position == input.start {
                return (steps, from_direction, start_direction);
            }
        }
    }
    panic!("not found");
}

// Skip over a piece of pipe in a horizontal line.
// Returns (line', is_toggle), where:
//   * line' is the rest of the line after the bit of skipped pipe
//   * is_toggle is whether we toggled from inside to outside or vice versa
fn skip_pipe<'a>(line: &'a [u8]) -> (&'a [u8], bool) {
    let mut line = line;
    let start = line[0];
    match start {
        b'|' => { return (&line[1..], true) },
        b'L' | b'F' => {
            loop {
                line = &line[1..];
                if line.len() == 0 { panic!("not looped") }
                match line[0] {
                    b'-' => { continue },
                    b'7' => { return (&line[1..], start == b'L') },
                    b'J' => { return (&line[1..], start == b'F') },
                    _ => panic!("not looped"),
                }
            }
        }
        _ => panic!("not looped"),
    }
}

fn main() {
    let mut buffer = [0; 141 * 140];
    let input = read_input(&mut buffer);

    // Part 1: find the loop of pipe and calculate the number of steps required
    // to reach the furthest position (which is just half the steps required to
    // traverse the loop, rounded down).
    let (steps, from_direction, start_direction) = find_loop(&input);
    let part1 = steps / 2;

    // Create a copy of the grid where every bit of pipe except for the loop is
    // replaced with a space.
    let mut copy = [[b' '; 140]; 140];
    for (_, (x, y)) in follow_pipe(&input, start_direction) {
        copy[y as usize][x as usize] = input.cell(x, y);
    }
    // Fill in the start tile with the appropriate bit of pipe.
    let start_value = match (from_direction, start_direction) {
        ((1, 0), (1, 0)) => b'-',
        ((1, 0), (0, -1)) => b'J',
        ((1, 0), (0, 1)) => b'7',
        ((-1, 0), (-1, 0)) => b'-',
        ((-1, 0), (0, -1)) => b'L',
        ((-1, 0), (0, 1)) => b'F',
        ((0, 1), (0, 1)) => b'|',
        ((0, 1), (-1, 0)) => b'J',
        ((0, 1), (1, 0)) => b'L',
        ((0, -1), (0, -1)) => b'|',
        ((0, -1), (-1, 0)) => b'7',
        ((0, -1), (1, 0)) => b'F',
        _ => panic!("can't deduce start pipe"),
    };
    copy[input.start.1 as usize][input.start.0 as usize] = start_value;

    // Count the number of empty cells which are enclosed by the pipe loop. We
    // can calculate this line by line by keeping track of every time we cross
    // over the pipe and thereby toggle from inside to outside or vice versa.
    let mut num_inside = 0;
    for y in 0 .. input.size.1 {
        let mut inside = false;
        let mut line = copy[y as usize].as_slice();
        loop {
            // Skip to the next bit of pipe.
            let mut spaces = 0;
            while line.len() > 0 && line[0] == b' ' {
                spaces += 1;
                line = &line[1..];
            }
            if inside { num_inside += spaces }
            if line.len() == 0 { break }
            // Skip over the bit of pipe.
            let (rest, is_toggle) = skip_pipe(line);
            line = rest;
            if is_toggle { inside = !inside }
        }
        if inside { panic!("not looped") }
    }
    let part2 = num_inside;

    print!("{}\n{}\n", part1, part2);
}
