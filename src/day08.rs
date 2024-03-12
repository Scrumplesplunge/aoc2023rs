use std::io;
use num_integer;

fn read_id(id: &str) -> u16 {
    let mut n: u16 = 0;
    for c in id.chars().take(3) {
        n = 32 * n + (c as u16 - 'A' as u16);
    }
    return n;
}

fn is_start(id: u16) -> bool { id % 32 == 0 }
fn is_end(id: u16) -> bool { id % 32 == 25 }

fn read_input() -> (Vec<bool>, Vec<(u16, (u16, u16))>) {
    let mut step_line = String::new();
    io::stdin().read_line(&mut step_line).unwrap();
    step_line.pop();
    let steps = step_line.chars().map(|c| c == 'L').collect();
    let mut nodes: Vec<(u16, (u16, u16))> = Vec::new();
    for line in io::stdin().lines().skip(1).map(|l| l.unwrap()) {
        let (node, branches) = line.split_once(" = ").unwrap();
        let (l, r) = branches
            .strip_prefix("(").unwrap()
            .strip_suffix(")").unwrap()
            .split_once(", ").unwrap();
        nodes.push((read_id(node), (read_id(l), read_id(r))));
    }
    nodes.sort();
    return (steps, nodes);
}

fn part1(steps: &[bool], nodes: &[(u16, (u16, u16))]) -> u32 {
    let mut directions = steps.iter().cycle();
    let mut num_steps = 0;
    let mut node: u16 = read_id("AAA");
    while node != read_id("ZZZ") {
        num_steps += 1;
        let (_, (l, r)) =
            &nodes[nodes.binary_search_by_key(&node, |(k, _)| *k).unwrap()];
        match directions.next().unwrap() {
            true => { node = *l },
            false => { node = *r },
        }
    }
    return num_steps;
}

fn part2(steps: &[bool], nodes: &[(u16, (u16, u16))]) -> u64 {
    let mut directions = steps.iter().cycle();
    let mut total = 1;
    for start in nodes.iter().map(|(k, _)| *k).filter(|k| is_start(*k)) {
        let mut node: u16 = start;
        let mut num_steps = 0;
        while !is_end(node) {
            num_steps += 1;
            let (_, (l, r)) =
                &nodes[nodes.binary_search_by_key(&node, |(k, _)| *k).unwrap()];
            match directions.next().unwrap() {
                true => { node = *l },
                false => { node = *r },
            }
        }
        total = num_integer::lcm(total, num_steps);
    }
    return total;
}

fn main() {
    let (steps, nodes) = read_input();
    print!("{}\n{}\n", part1(&steps, &nodes), part2(&steps, &nodes));
}
