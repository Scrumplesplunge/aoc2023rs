use std::io;
use std::io::Read;
use std::str;

fn parse_int(s: &mut &str) -> u32 {
    let n = s.find(|c: char| !c.is_digit(10)).unwrap_or(s.len());
    let i = s[0..n].parse().unwrap();
    *s = &mut &s[n..];
    return i;
}

fn main() {
    let mut buffer = [0; 10240];
    let size = io::stdin().read(&mut buffer).unwrap();
    let input = str::from_utf8(&buffer[0..size]).unwrap();
    let mut i = input;

    let mut part1 = 0;
    let mut part2 = 0;
    while !i.is_empty() {
        i = i.strip_prefix("Game ").unwrap();
        let id = parse_int(&mut i);
        i = i.strip_prefix(": ").unwrap();

        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        loop {
            let count = parse_int(&mut i);
            if let Some(j) = i.strip_prefix(" red") {
                i = j;
                r = r.max(count);
            } else if let Some(j) = i.strip_prefix(" green") {
                i = j;
                g = g.max(count);
            } else {
                i = i.strip_prefix(" blue").unwrap();
                b = b.max(count);
            }
            if i.chars().nth(0).unwrap() == '\n' { break }
            if let Some(j) = i.strip_prefix("; ") {
                i = j;
            } else {
                i = i.strip_prefix(", ").unwrap();
            }
        }
        i = i.strip_prefix("\n").unwrap();

        if r <= 12 && g <= 13 && b <= 14 { part1 += id }
        part2 += r * g * b;
    }

    print!("{}\n{}\n", part1, part2);
}
