use std::io;
use std::io::Read;

const MAX_SIZE: usize = 141;
const BUFFER_SIZE: usize = (MAX_SIZE + 1) * MAX_SIZE;
const QUEUE_SIZE: usize = 64000;

#[derive(Copy, Clone, Default)]
enum Direction {
    #[default]
    Up = 1,
    Down = 2,
    Left = 4,
    Right = 8,
}

impl Direction {
    fn left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
    fn right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
    fn go(&self, n: i16, (x, y): (i16, i16)) -> (i16, i16) {
        match self {
            Direction::Up => (x, y - n),
            Direction::Down => (x, y + n),
            Direction::Left => (x - n, y),
            Direction::Right => (x + n, y),
        }
    }
}

#[derive(Copy, Clone, Default)]
struct QueueEntry {
    heuristic_cost: u16,
    cost: u16,
    position: (u8, u8),
    direction: Direction,
}

type Queue = [QueueEntry; QUEUE_SIZE];

fn pop(queue: &mut Queue, queue_size: &mut usize) -> QueueEntry {
    let result = queue[0];
    *queue_size -= 1;
    let x = queue[*queue_size];
    let mut i = 0;
    loop {
        let l = 2 * i + 1;
        let r = 2 * i + 2;
        if l >= *queue_size { break }
        let mut c = l;
        if r < *queue_size && queue[l].heuristic_cost > queue[r].heuristic_cost { c = r }
        if x.heuristic_cost <= queue[c].heuristic_cost { break }
        queue[i] = queue[c];
        i = c;
    }
    queue[i] = x;
    return result;
}

fn push(queue: &mut Queue, queue_size: &mut usize, entry: QueueEntry) {
    let mut i = *queue_size;
    *queue_size += 1;
    while i != 0 {
        let parent = (i - 1) / 2;
        if queue[parent].heuristic_cost <= entry.heuristic_cost { break }
        queue[i] = queue[parent];
        i = parent;
    }
    queue[i] = entry;
}

fn manhattan_distance((ax, ay): (u8, u8), (bx, by): (u8, u8)) -> u16 {
    let dx = if ax < bx { bx - ax } else { ax - bx };
    let dy = if ay < by { by - ay } else { ay - by };
    return dx as u16 + dy as u16;
}

fn solve(
    grid: &[u8],
    size: usize,
    min_steps: i16,
    max_steps: i16,
) -> u16 {
    let mut seen = [0; BUFFER_SIZE];
    let mut queue: Queue = [Default::default(); QUEUE_SIZE];
    let end = ((size - 1) as u8, (size - 1) as u8);
    let end_index = size * size - 1;
    queue[0] = QueueEntry{
        heuristic_cost: manhattan_distance((0, 0), end),
        cost: 0,
        position: (0, 0),
        direction: Direction::Right,
    };
    queue[1] = queue[0];
    queue[1].direction = Direction::Down;
    let mut queue_size = 2;
    loop {
        // Pop the lowest cost entry from the queue.
        if queue_size == 0 { panic!("not found") }
        let entry = pop(&mut queue, &mut queue_size);
        let (x, y) = (entry.position.0 as i16, entry.position.1 as i16);
        let index = y as usize * size + x as usize;
        if index == end_index { return entry.cost }
        if seen[index] & entry.direction as u8 != 0 { continue }
        seen[index] |= entry.direction as u8;

        // We are at the position indicated by `entry.index`, facing in the direction indicated by
        // `entry.direction`, and we must now make a number of forward steps within the range
        // `steps`.
        let (x2, y2) = entry.direction.go(min_steps, (x, y));
        if x2 < 0 || size as i16 <= x2 || y2 < 0 || size as i16 <= y2 { continue }
        let mut cost = entry.cost;
        // Calculate the unconditionally paid cost from taking the minimum acceptable number of
        // steps.
        for i in 1..min_steps {
            let (x2, y2) = entry.direction.go(i, (x, y));
            cost += grid[y2 as usize * size + x2 as usize] as u16;
        }
        // Enqueue left and right turns at all possible stopping locations.
        for i in min_steps..max_steps+1 {
            let (x2, y2) = entry.direction.go(i, (x, y));
            if x2 < 0 || size as i16 <= x2 || y2 < 0 || size as i16 <= y2 { break }
            let next_index = y2 as usize * size + x2 as usize;
            cost += grid[next_index] as u16;
            let heuristic_cost = cost + manhattan_distance((x2 as u8, y2 as u8), end);
            let mut next = QueueEntry{
                heuristic_cost: heuristic_cost,
                cost: cost,
                position: (x2 as u8, y2 as u8),
                direction: entry.direction.left(),
            };
            if seen[next_index] & next.direction as u8 == 0 {
                push(&mut queue, &mut queue_size, next);
            }
            next.direction = entry.direction.right();
            if seen[next_index] & next.direction as u8 == 0 {
                push(&mut queue, &mut queue_size, next);
            }
        }
    }
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
    // Compact the grid.
    for y in 0..size {
        let from = y * (size + 1);
        buffer.copy_within(from..from + size, y * size);
    }
    // Map all cells to their integer values.
    for c in &mut buffer[0 .. size * size] { *c -= b'0' }

    let part1 = solve(&buffer, size, 1, 3);
    let part2 = solve(&buffer, size, 4, 10);
    print!("{}\n{}\n", part1, part2);
}
