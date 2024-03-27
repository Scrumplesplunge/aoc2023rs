use std::io;
use std::io::Read;
use num_integer;

fn id(x: &[u8; 3]) -> u16 {
    let mut out = 0;
    for b in x {
        match b {
            b'A'..=b'Z' => out = 32 * out + (b - b'A' + 1) as u16,
            _ => panic!("bad id"),
        }
    }
    return out;
}

fn is_end(id: u16) -> bool { id % 32 == 26 }

const MAX_NODES: usize = 26 * 32 * 32 + 26 * 32 + 26 + 1;
type Nodes = [[u16; 2]; MAX_NODES];

fn read_input<'a>(
    step_buffer: &'a mut [bool],
    nodes: &mut Nodes,
) -> &'a [bool] {
    let mut buffer = [0; 14 * 1024];
    let len = io::stdin().read(&mut buffer).unwrap();
    if len == 0 || buffer[len - 1] != b'\n' { panic!("bad input") }
    let input = &buffer[0..len - 1];
    let num_steps = input.iter().position(|b| *b == b'\n').unwrap();
    for i in 0..num_steps {
        match input[i] {
            b'L' => step_buffer[i] = false,
            b'R' => step_buffer[i] = true,
            _ => panic!("bad step"),
        }
    }
    let body = &input[num_steps + 2..];
    for line in body.split(|b| *b == b'\n') {
        match line {
            [a1, a2, a3, b' ', b'=', b' ', b'(', b1, b2, b3, b',', b' ', c1, c2, c3, b')'] => {
                nodes[id(&[*a1, *a2, *a3]) as usize] = [id(&[*b1, *b2, *b3]), id(&[*c1, *c2, *c3])];
            }
            _ => panic!("bad node"),
        }
    }

    return &step_buffer[0..num_steps];
}

fn part1(steps: &[bool], nodes: &Nodes) -> u32 {
    let mut next_step = 0;
    let mut num_steps = 0;
    let mut node: u16 = id(b"AAA");
    let end = id(b"ZZZ");
    while node != end {
        let d = steps[next_step] as usize;
        next_step += 1;
        if next_step >= steps.len() { next_step = 0 }
        node = nodes[node as usize][d];
        num_steps += 1;
    }
    return num_steps;
}

fn part2(steps: &[bool], nodes: &Nodes) -> u64 {
    let mut total = 1;
    for a in b'A'..=b'Z' {
        for b in b'A'..=b'Z' {
            let start = id(&[a, b, b'A']);
            if nodes[start as usize][0] == 0 { continue }
            let mut node: u16 = start;
            let mut num_steps = 0;
            let mut next_step = 0;
            while !is_end(node) {
                let d = steps[next_step] as usize;
                next_step += 1;
                if next_step >= steps.len() { next_step = 0 }
                node = nodes[node as usize][d];
                num_steps += 1;
            }
            total = num_integer::lcm(total, num_steps);
        }
    }
    return total;
}

fn main() {
    let mut step_buffer = [false; 300];
    let mut nodes = [[0, 0]; MAX_NODES];
    let steps = read_input(&mut step_buffer, &mut nodes);
    print!("{}\n{}\n", part1(&steps, &nodes), part2(&steps, &nodes));
}
