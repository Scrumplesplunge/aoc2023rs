use std::io;
use std::io::Read;

enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

fn parse(line: &[u8]) -> (Direction, i64, Direction, i64) {
    match line {
        [direction, b' ', n @ .., b' ', b'(', b'#', a, b, c, d, e, f, b')'] => {
            let part1_direction = match direction {
                b'R' => Direction::Right,
                b'D' => Direction::Down,
                b'L' => Direction::Left,
                b'U' => Direction::Up,
                _ => panic!("bad direction"),
            };
            let mut part1_amount = 0;
            for x in n {
                if !(b'0' <= *x && *x <= b'9') { panic!("bad amount") }
                part1_amount = 10 * part1_amount + (x - b'0') as i64;
            }
            let part2_direction = match f {
                b'0' => Direction::Right,
                b'1' => Direction::Down,
                b'2' => Direction::Left,
                b'3' => Direction::Up,
                _ => panic!("bad color"),
            };
            let mut part2_amount = 0;
            for x in [a, b, c, d, e] {
                match x {
                    b'0'..=b'9' => part2_amount = 16 * part2_amount + (x - b'0') as i64,
                    b'a'..=b'f' => part2_amount = 16 * part2_amount + 10 + (x - b'a') as i64,
                    _ => panic!("bad color"),
                }
            }
            return (part1_direction, part1_amount, part2_direction, part2_amount);
        },
        _ => panic!("bad line"),
    }
}

#[derive(Default)]
struct Shoelace {
    y: i64,
    integral: i64,
    perimeter: i64,
}

impl Shoelace {
    fn go(&mut self, direction: Direction, amount: i64) {
        self.perimeter += amount;
        match direction {
            Direction::Up => self.y -= amount,
            Direction::Down => self.y += amount,
            Direction::Left => self.integral -= self.y * amount,
            Direction::Right => self.integral += self.y * amount,
        }
    }

    fn area(&self) -> i64 {
        return self.integral.abs() + self.perimeter / 2 + 1;
    }
}


fn main() {
    let mut buffer = [0; 10 * 1024];
    let length = io::stdin().read(&mut buffer).unwrap();
    if length == 0 || buffer[length - 1] != b'\n' { panic!("bad input") }
    let input = &buffer[0 .. length - 1];

    let mut part1: Shoelace = Default::default();
    let mut part2: Shoelace = Default::default();
    for (p1d, p1a, p2d, p2a) in input.split(|b| *b == b'\n').map(parse) {
        part1.go(p1d, p1a);
        part2.go(p2d, p2a);
    }
    print!("{}\n{}\n", part1.area(), part2.area());
}
