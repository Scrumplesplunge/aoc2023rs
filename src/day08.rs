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

const NODE_SIZE: usize = 17;  // Gives the best performance on my machine.
type Node = [(u16, (u16, u16)); NODE_SIZE];

fn read_input() -> (Vec<bool>, Vec<Node>) {
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

    // Sort the list of nodes into a btree.
    let btree_size = (nodes.len() + NODE_SIZE - 1) / NODE_SIZE;
    let mut btree = vec![[(65535, (0, 0)); NODE_SIZE]; btree_size];
    nodes.sort();
    fill_btree(0, &mut btree, &nodes);
    return (steps, btree);
}

fn fill_btree<'a>(
    index: usize,
    tree: &mut[Node],
    values: &'a [(u16, (u16, u16))],
) -> &'a [(u16, (u16, u16))] {
    if index >= tree.len() { return values }
    let children_index = (NODE_SIZE + 1) * index + 1;
    let mut values = fill_btree(children_index, tree, values);
    for j in 0 .. NODE_SIZE {
        if values.is_empty() { break }
        tree[index][j] = values[0];
        values = fill_btree(children_index + j + 1, tree, &values[1..]);
    }
    return values;
}

fn btree_find(index: usize, tree: &[Node], key: u16) -> (u16, u16) {
    if index >= tree.len() { panic!("not found: {}", key) }
    let children_base = (NODE_SIZE + 1) * index + 1;
    for j in 0 .. NODE_SIZE {
        let (k, v) = tree[index][j];
        if k == key { return v }
        if k > key { return btree_find(children_base + j, tree, key) }
    }
    return btree_find(children_base + NODE_SIZE, tree, key);
}

fn part1(steps: &[bool], nodes: &[Node]) -> u32 {
    let mut directions = steps.iter().cycle();
    let mut num_steps = 0;
    let mut node: u16 = read_id("AAA");
    while node != read_id("ZZZ") {
        num_steps += 1;
        let (l, r) = btree_find(0, nodes, node);
        match directions.next().unwrap() {
            true => { node = l },
            false => { node = r },
        }
    }
    return num_steps;
}

fn part2(steps: &[bool], nodes: &[Node]) -> u64 {
    let mut directions = steps.iter().cycle();
    let mut total = 1;
    let start_nodes = nodes
        .iter()
        .flatten()
        .map(|(k, _)| *k)
        .filter(|k| is_start(*k));
    for start in start_nodes {
        let mut node: u16 = start;
        let mut num_steps = 0;
        while !is_end(node) {
            num_steps += 1;
            let (l, r) = btree_find(0, &nodes, node);
            match directions.next().unwrap() {
                true => { node = l },
                false => { node = r },
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
