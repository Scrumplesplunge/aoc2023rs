use std::io;
use std::io::Read;
use std::str;

type Grid = [[char; 140]; 140];

fn read_input() -> Grid {
    let mut buffer = [0; 20480];
    let size = io::stdin().read(&mut buffer).unwrap();
    let input = str::from_utf8(&buffer[0..size]).unwrap();
    let width = input.find('\n').unwrap();
    if width > 140 { panic!("grid too wide") }
    let mut height = 0;
    let mut grid = [['.'; 140]; 140];
    for line in input.lines() {
        if line.len() != width { panic!("not a grid") }
        for (i, c) in line.char_indices() { grid[height][i] = c }
        height += 1;
    }
    return grid;
}

// NumberIterator iterates over numbers that appear in a sequence of characters.
struct NumberIterator<'a, T: Iterator<Item = &'a char>> {
    data: T,
}

impl<'a, T: Iterator<Item = &'a char>> Iterator for NumberIterator<'a, T> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip any non-digit characters.
        let i = &mut self.data;
        if let Some(c) = i.find(|c| c.is_digit(10)) {
            let mut n = c.to_digit(10).unwrap();
            while let Some(c) = i.next() {
                if let Some(d) = c.to_digit(10) {
                    n = 10 * n + d;
                } else { break }
            }
            return Some(n);
        } else {
            return None;
        }
    }
}

fn numbers<'a, T: Iterator<Item = &'a char>>(i: T) -> NumberIterator<'a, T> {
    return NumberIterator{data: i};
}

struct Part<'a> {
    grid: &'a Grid,
    x: usize,
    y: usize,
}

fn to_part<'a>(grid: &'a Grid, x: usize, y: usize) -> Option<Part<'a>> {
    let c = grid[y][x];
    if c == '.' || c.is_digit(10) { return None }
    return Some(Part{grid: &grid, x: x, y: y});
}

// Iterate over all parts in the grid.
fn parts(grid: &Grid) -> impl Iterator<Item = Part> {
    return (0..140).flat_map(move |x| {
        (0..140).filter_map(move |y| {
            to_part(grid, x, y)
        })
    });
}

// Iterate over all the numbers adjacent to a part.
fn neighbours<'a>(part: &'a Part) -> impl Iterator<Item = u32> + 'a {
    let min_y = if part.y == 0 { 0 } else { part.y - 1 };
    let max_y = (part.y + 2).min(140);
    return (min_y .. max_y).flat_map(move |iy| {
        // Scan left/right as far as there are digits. The individual numbers
        // are picked out by the loop below.
        let row = &part.grid[iy];
        let min_x = row[..part.x]
            .iter()
            .rposition(|c| !c.is_digit(10))
            .unwrap_or(0);
        let max_x = row[part.x + 1..]
            .iter()
            .position(|c| !c.is_digit(10))
            .map(|i| part.x + 1 + i)
            .unwrap_or(140);
        numbers(part.grid[iy][min_x .. max_x].iter())
    });
}

fn main() {
    let grid = read_input();
    let mut part1 = 0;
    let mut part2 = 0;
    // Assumption #1: No number is adjacent to two symbols.
    // Assumption #2: Only '*' symbols are adjacent to exactly two numbers.
    for part in parts(&grid) {
        let mut count = 0;
        let mut product = 1;
        for number in neighbours(&part) {
            count += 1;
            part1 += number;
            product *= number;
        }
        if count == 2 { part2 += product }
    }
    print!("{}\n{}\n", part1, part2);
}
