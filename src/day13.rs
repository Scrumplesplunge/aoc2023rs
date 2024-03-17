use std::io;
use std::io::Read;
use std::str;

fn mismatches(grid: &[u8], w: usize, h: usize) -> ([u32; 20], [u32; 20]) {
    // `mismatches_x[i]` is the number of reflected positions which do not
    // match for a mirror inbetween `x = i - 1` and `x = i`.
    let mut mismatches_x = [0; 20];
    for mirror in 1 .. w {
        let x_min = mirror - mirror.min(w - mirror);
        let x_max = mirror;
        let mut count = 0;
        for y in 0 .. h {
            let row = &grid[y * (w + 1) ..][..w];
            for x in x_min .. x_max {
                let reflected = x_max + (x_max - x - 1);
                if row[x] != row[reflected] { count += 1 }
            }
        }
        mismatches_x[mirror] = count;
    }
    // `mismatches_y` is the same as `mismatches_x`, but for horizontal
    // mirrors instead of vertical ones.
    let mut mismatches_y = [0; 20];
    for mirror in 1 .. h {
        let y_min = mirror - mirror.min(h - mirror);
        let y_max = mirror;
        let mut count = 0;
        for y in y_min .. y_max {
            let r = y_max + (y_max - y - 1);
            let a = &grid[y * (w + 1) ..][..w];
            let b = &grid[r * (w + 1) ..][..w];
            for x in 0 .. w {
                if a[x] != b[x] { count += 1; }
            }
        }
        mismatches_y[mirror] = count;
    }
    return (mismatches_x, mismatches_y);
}

fn main() {
    let mut buffer = [0; 20480];
    let length = io::stdin().read(&mut buffer).unwrap();
    if buffer[length - 1] != b'\n' { panic!("no newline at end of input") }
    let input = str::from_utf8(&buffer[0..length - 1]).unwrap();
    if !input.is_ascii() { panic!("input is not ascii") }

    let mut part1 = 0;
    let mut part2 = 0;
    for item in input.split("\n\n") {
        // Get the grid dimensions and verify that the input is actually a grid.
        let w = item.find('\n').unwrap();
        let mut h = 0;
        for line in item.split("\n") {
            if line.len() != w { panic!("not a grid") }
            h += 1;
        }
        let (mx, my) = mismatches(item.as_bytes(), w, h);

        // Part 1: The mirror is the single entry with 0 mismatches.
        if let Some(x) = mx[1 .. w].iter().position(|x| *x == 0) {
            part1 += x + 1;
        } else if let Some(y) = my[1 .. h].iter().position(|y| *y == 0) {
            part1 += 100 * (y + 1);
        } else {
            panic!("no mirror");
        }

        // Part 2: The mirror is the single entry with 1 mismatch.
        if let Some(x) = mx[1 .. w].iter().position(|x| *x == 1) {
            part2 += x + 1;
        } else if let Some(y) = my[1 .. h].iter().position(|y| *y == 1) {
            part2 += 100 * (y + 1);
        } else {
            panic!("no mirror");
        }
    }
    print!("{}\n{}\n", part1, part2);
}
