use std::io;
use std::io::Read;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

const MAX_NODES: usize = 4096;
const MAX_EDGES: usize = 8192;
const NO_ID: u16 = MAX_NODES as u16;

fn read_id(input: &mut &[u8], id_buf: &mut[u16], next_id: &mut u16) -> u16 {
    match *input {
        [a @ b'a'..=b'z', b @ b'a'..=b'z', c @ b'a'..=b'z', tail @ ..] => {
            *input = tail;
            let index = ((a - b'a') as usize * 26 + (b - b'a') as usize) * 26 + (c - b'a') as usize;
            if id_buf[index] == NO_ID {
                id_buf[index] = *next_id;
                *next_id += 1;
            }
            return id_buf[index];
        }
        _ => panic!("bad input"),
    }
}

fn read_input<'a>(edges: &'a mut[(u16, u16)]) -> (u16, &'a [(u16, u16)]) {
    let mut buffer = [0; 20 * 1024];
    let len = io::stdin().read(&mut buffer).unwrap();
    let mut input = &buffer[0..len];
    let mut id_buf = [NO_ID; 26 * 26 * 26];
    let mut next_id = 0;
    let mut num_edges = 0;
    while input.len() > 0 {
        let a = read_id(&mut input, &mut id_buf, &mut next_id);
        input = input.strip_prefix(b": ").unwrap();
        loop {
            let b = read_id(&mut input, &mut id_buf, &mut next_id);
            edges[num_edges] = (a, b);
            num_edges += 1;
            match input {
                [b' ', tail @ ..] => input = tail,
                [b'\n', tail @ ..] => {
                    input = tail;
                    break;
                }
                _ => panic!("bad input"),
            }
        }
    }
    return (next_id, &mut edges[0..num_edges]);
}

fn contract<'a>(
    rng: &mut ThreadRng,
    size: &mut [u32],
    mut edges: &'a mut [(u16, u16)],
    mut num_nodes: u16,
    target_num_nodes: u16,
) -> &'a mut [(u16, u16)] {
    while num_nodes > target_num_nodes {
        // Pick a random edge.
        let i = Uniform::from(0..edges.len()).sample(rng);
        let (a, b) = edges[i];
        // Merge the nodes at either end of the edge.
        size[a as usize] += size[b as usize];
        num_nodes -= 1;
        // Update all edges to only use `a`, and remove any `(a, a)` edges.
        let mut j = 0;
        for i in 0..edges.len() {
            let (mut p, mut q) = edges[i];
            if p == b { p = a }
            if q == b { q = a }
            if p != q {
                edges[j] = (p, q);
                j += 1;
            }
        }
        edges = &mut edges[0..j];
    }
    return edges;
}

fn karger_stein(
    rng: &mut ThreadRng,
    num_nodes: u16,
    size: &mut [u32],
    edges: &mut [(u16, u16)],
) -> (usize, u32, u32) {
    if num_nodes < 24 {
        let edges = contract(rng, size, edges, num_nodes, 2);
        let cut_size = edges.len();
        let (a, b) = edges[0];
        return (cut_size, size[a as usize], size[b as usize]);
    } else {
        let t = num_nodes * 2 / 3;

        let (n1, a1, b1) = {
            let mut size_copy_buffer = [0; MAX_NODES];
            let size_copy = &mut size_copy_buffer[0..size.len()];
            size_copy.copy_from_slice(size);
            let mut edge_copy_buffer = [(0, 0); MAX_EDGES];
            let edges_copy = &mut edge_copy_buffer[0..edges.len()];
            edges_copy.copy_from_slice(edges);
            let edges_copy = contract(rng, size_copy, edges_copy, num_nodes, t);
            karger_stein(rng, t, size_copy, edges_copy)
        };

        let edges = contract(rng, size, edges, num_nodes, t);
        let (n2, a2, b2) = karger_stein(rng, t, size, edges);

        return if n1 < n2 { (n1, a1, b1) } else { (n2, a2, b2) };
    }
}

fn main() {
    let mut edge_buffer = [(0, 0); MAX_EDGES];
    let (num_nodes, edges) = read_input(&mut edge_buffer);
    let mut rng = rand::thread_rng();
    loop {
        let mut size = [1; MAX_NODES];
        let mut edge_copy_buffer = [(0, 0); MAX_EDGES];
        let edges_copy = &mut edge_copy_buffer[0..edges.len()];
        edges_copy.copy_from_slice(edges);
        let (n, a, b) = karger_stein(&mut rng, num_nodes, &mut size, edges_copy);
        if n == 3 {
            print!("{}\n", a * b);
            break;
        }
    }
}
