use std::io;
use num_integer;

fn part1(steps: &str, nodes: &[(String, (String, String))]) -> u32 {
    let mut directions = steps.chars().cycle();
    let mut num_steps = 0;
    let mut node: &str = "AAA";
    while node != "ZZZ" {
        num_steps += 1;
        let (_, (l, r)) =
            &nodes[nodes.binary_search_by_key(&node, |(k, _)| k).unwrap()];
        match directions.next().unwrap() {
            'L' => { node = &l },
            'R' => { node = &r },
            c => panic!("bad direction: {}", c),
        }
    }
    return num_steps;
}

fn part2(steps: &str, nodes: &[(String, (String, String))]) -> u64 {
    let mut directions = steps.chars().cycle();
    let mut total = 1;
    for start in nodes.iter().map(|(k, _)| k).filter(|k| k.ends_with("A")) {
        let mut node: &str = start;
        let mut num_steps = 0;
        while !node.ends_with("Z") {
            num_steps += 1;
            let (_, (l, r)) =
                &nodes[nodes.binary_search_by_key(&node, |(k, _)| k).unwrap()];
            match directions.next().unwrap() {
                'L' => { node = &l },
                'R' => { node = &r },
                c => panic!("bad direction: {}", c),
            }
        }
        total = num_integer::lcm(total, num_steps);
    }
    return total;
}

fn main() {
    let mut steps = String::new();
    io::stdin().read_line(&mut steps).unwrap();
    steps.pop();
    let mut nodes: Vec<(String, (String, String))> = Vec::new();
    for line in io::stdin().lines().skip(1).map(|l| l.unwrap()) {
        let (node, branches) = line.split_once(" = ").unwrap();
        let (l, r) = branches
            .strip_prefix("(").unwrap()
            .strip_suffix(")").unwrap()
            .split_once(", ").unwrap();
        nodes.push((node.to_string(), (l.to_string(), r.to_string())));
    }
    nodes.sort();

    print!("{}\n{}\n", part1(&steps, &nodes), part2(&steps, &nodes));
}
