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

fn part1(grid: &Grid) -> u32 {
    // Create a second grid which expands influence outwards from symbols.
    let mut influence = [[false; 140]; 140];
    for y in 0..140 {
        for x in 0..140 {
            let c = grid[y as usize][x as usize];
            if c != '.' && !c.is_digit(10) {
                for iy in y - 1..y + 2 {
                    if iy < 0 || 140 <= iy { continue }
                    for ix in x - 1..x + 2 {
                        if ix < 0 || 140 <= ix { continue }
                        influence[iy as usize][ix as usize] = true;
                    }
                }
            }
        }
    }
    // Parse and sum all part numbers.
    let mut total = 0;
    for y in 0..140 {
        let mut is_part = false;
        let mut number = 0;
        for x in 0..140 {
            if let Some(d) = grid[y][x].to_digit(10) {
                number = 10 * number + d;
                is_part |= influence[y][x];
            } else {
                if is_part { total += number }
                is_part = false;
                number = 0;
            }
        }
        if is_part { total += number }
    }
    return total;
}

fn gear_ratio(grid: &Grid, x: i32, y: i32) -> Option<u32> {
    let mut count = 0;
    let mut product = 1;
    // Iterate over all positions that can contain adjacent numbers.
    for iy in y - 1..y + 2 {
        if iy < 0 || 140 <= iy { continue }
        let row = &grid[iy as usize];
        // Scan left/right as far as there are digits. The individual numbers
        // are picked out by the loop below.
        let min_x = row[..x as usize]
            .iter()
            .rposition(|c| !c.is_digit(10))
            .unwrap_or(0);
        let max_x = row[x as usize + 1..]
            .iter()
            .position(|c| !c.is_digit(10))
            .map(|i| x as usize + 1 + i)
            .unwrap_or(140);
        let mut number = None;
        for ix in min_x..max_x {
            if let Some(d) = grid[iy as usize][ix as usize].to_digit(10) {
                number = Some(10 * number.unwrap_or(0) + d);
            } else {
                if let Some(n) = number {
                    count += 1;
                    product *= n;
                    number = None;
                    if count > 2 { return None }
                }
            }
        }
        if let Some(n) = number {
            count += 1;
            product *= n;
            if count > 2 { return None }
        }
    }
    if count == 2 { return Some(product) }
    return None;
}

fn part2(grid: &Grid) -> u32 {
    let mut total = 0;
    for y in 0..140 {
        for x in 0..140 {
            if grid[y as usize][x as usize] != '*' { continue }
            if let Some(x) = gear_ratio(grid, x, y) { total += x }
        }
    }
    return total;
}

fn main() {
    let grid = read_input();
    print!("{}\n{}\n", part1(&grid), part2(&grid));
}
