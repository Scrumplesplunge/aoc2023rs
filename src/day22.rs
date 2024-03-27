use std::io;
use std::io::Read;

const MAX_BRICKS: usize = 2048;
const MAX_CONTACTS: usize = 2048;

fn eat(c: u8, s: &mut &[u8]) {
    if s[0] != c { panic!("expected {}", c as char) }
    *s = &s[1..];
}

fn read_u8(s: &mut &[u8]) -> u8 {
    match s[0] {
        b'0'..=b'9' => {
            let value = s[0] - b'0';
            *s = &s[1..];
            return value;
        }
        _ => panic!("not a digit"),
    }
}

fn read_u16(s: &mut &[u8]) -> u16 {
    let mut value = 0;
    let mut count = 0;
    while let [x @ b'0'..=b'9', ..] = &s[count..] {
        value = 10 * value + (x - b'0') as u16;
        count += 1;
    }
    if count == 0 { panic!("not a number") }
    *s = &s[count..];
    return value;
}

fn read_input<'a>(contacts: &'a mut [(u16, u16)]) -> (usize, &'a [(u16, u16)]) {
    let mut buffer = [0; 24 * 1024];
    let len = io::stdin().read(&mut buffer).unwrap();
    if len == 0 || buffer[len - 1] != b'\n' { panic!("bad input") }
    let input = &buffer[0..len - 1];

    // Parse all the bricks.
    let mut bricks = [(0, 0, 0, 0, 0, 0); MAX_BRICKS];
    let mut num_bricks = 0;
    for mut line in input.split(|b| *b == b'\n') {
        let x1 = read_u8(&mut line);
        eat(b',', &mut line);
        let y1 = read_u8(&mut line);
        eat(b',', &mut line);
        let z1 = read_u16(&mut line);
        eat(b'~', &mut line);
        let x2 = read_u8(&mut line);
        eat(b',', &mut line);
        let y2 = read_u8(&mut line);
        eat(b',', &mut line);
        let z2 = read_u16(&mut line);
        if line != b"" { panic!("trailing characters") }
        bricks[num_bricks] = (x1, y1, z1, x2, y2, z2);
        num_bricks += 1;
    }
    let bricks = &mut bricks[0..num_bricks];

    // Sort them by ascending Z.
    bricks.sort_unstable_by_key(|(_, _, z, _, _, _)| *z);

    // Identify all the points of contact between bricks.
    let mut num_contacts = 0;
    let mut z = [[(0, 0); 10]; 10];  // 2d map of (z, id)
    for (id, (x1, y1, z1, x2, y2, z2)) in (1..).zip(bricks.iter()) {
        // Calculate the height which the brick will rest at.
        let mut support_z = 0;
        for y in *y1 as usize..=*y2 as usize {
            for x in *x1 as usize..=*x2 as usize {
                support_z = support_z.max(z[y][x].0);
            }
        }
        // Identify all bricks which support this brick.
        let first = num_contacts;
        for y in *y1 as usize..=*y2 as usize {
            for x in *x1 as usize..=*x2 as usize {
                if z[y][x].0 != support_z { continue }
                let support_id = z[y][x].1;
                let seen = &contacts[first..num_contacts];
                if let Some(_) = seen.iter().find(|(i, _)| *i == support_id) { continue }
                contacts[num_contacts] = (support_id, id as u16);
                num_contacts += 1;
            }
        }
        // Update the z buffer.
        let h = support_z + z2 + 1 - z1;
        for y in *y1 as usize..=*y2 as usize {
            for x in *x1 as usize..=*x2 as usize {
                z[y][x] = (h, id as u16);
            }
        }
    }

    return (num_bricks, &contacts[0..num_contacts]);
}

fn main() {
    let mut contact_buffer = [(0, 0); MAX_CONTACTS];
    let (num_bricks, contacts) = read_input(&mut contact_buffer);

    // `supports[i]` is the number of bricks directly supporting brick `i`.
    let mut supports = [0; MAX_BRICKS];
    for (_, b) in contacts { supports[*b as usize] += 1 }
    // `removable[i]` is true if it is safe to remove brick `i`.
    let mut removable = [true; MAX_BRICKS];
    // It is safe to remove a brick if every brick supported by it has at least 2 supports.
    for (a, b) in contacts {
        if supports[*b as usize] == 1 { removable[*a as usize] = false }
    }
    let part1 = &removable[1..=num_bricks].iter().filter(|x: &&bool| **x).count();

    // Count the number of other bricks that fall if we remove each brick.
    let mut part2 = 0;
    for brick in 1..=num_bricks {
        // `supported[i]` is true if brick `i` is still supported without `brick`.
        let mut supported = [false; MAX_BRICKS];
        supported[0] = true;
        // `contacts` is sorted by `b`, so by the time we process `(a, b)`, we must have already
        // processed any `(x, a)` and thus would have already populated `supported[a]`. This means
        // that we can calculate `supported` in a single pass.
        for (a, b) in contacts {
            if *b as usize != brick && supported[*a as usize] {
                supported[*b as usize] = true;
            }
        }
        let num_supported = &supported[1..=num_bricks].iter().filter(|x| **x).count();
        part2 += num_bricks - num_supported - 1;
    }

    print!("{}\n{}\n", part1, part2);
}
