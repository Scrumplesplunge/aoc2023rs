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
    return (next_id, &edges[0..num_edges]);
}

fn karger(rng: &mut ThreadRng, mut num_nodes: u16, input: &[(u16, u16)]) -> (usize, u32, u32) {
    let mut edge_buffer = [(0, 0); MAX_EDGES];
    let mut edges = &mut edge_buffer[0..input.len()];
    let mut size = [1; MAX_NODES];
    edges.copy_from_slice(input);
    while num_nodes > 2 {
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
    let cut_size = edges.len();
    let (a, b) = edges[0];
    return (cut_size, size[a as usize], size[b as usize]);
}

fn main() {
    let mut edge_buffer = [(0, 0); MAX_EDGES];
    let (num_nodes, edges) = read_input(&mut edge_buffer);
    let mut rng = rand::thread_rng();
    let mut runs = 0;
    loop {
        runs += 1;
        let (n, a, b) = karger(&mut rng, num_nodes, edges);
        if n == 3 {
            print!("{}\n", a * b);
            eprint!("Took {} runs\n", runs);
            break;
        }
    }
}
