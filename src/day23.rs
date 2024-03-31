use std::io;
use std::io::Read;

const GRID_SIZE: usize = 141;
const MAX_EDGES: usize = 128;

const ROW: usize = GRID_SIZE + 1;
const START: usize = 1;                  // First few bytes are "# ###"
const END: usize = ROW * GRID_SIZE - 3;  // Last few bytes are "# #\n"

type Node = u16;
const START_NODE: Node = 0;
const END_NODE: Node = 1;
const MAX_NODES: usize = 36;

const UPHILL: u8 = 1;
const DOWNHILL: u8 = 2;
type Edge = (Node, Node, u8, u16);

fn read_input<'a>(edges: &'a mut [Edge]) -> &'a [Edge] {
    // Load the grid into a buffer.
    let mut buffer = [0; (GRID_SIZE + 1) * GRID_SIZE];
    let len = io::stdin().read(&mut buffer).unwrap();
    if len != buffer.len() { panic!("bad input") }
    for line in buffer[0..len-1].split(|b| *b == b'\n') {
        if line.len() != GRID_SIZE { panic!("not a grid") }
    }
    // Close the entrance. This makes the graph exploration simpler since we don't have to check for
    // falling out of the start.
    buffer[1] = b'#';
    // Convert the grid into a list of edges with nodes at each crossroads.
    // nodes[i] is the ID of the node at grid[i].
    let mut nodes = [0; (GRID_SIZE + 1) * GRID_SIZE];
    let mut num_nodes = 2;  // START_NODE and END_NODE are predefined as 0 and 1.
    let mut num_edges = 0;
    graphify(&mut buffer, &mut nodes, &mut num_nodes, edges, &mut num_edges, START, &[START + ROW]);
    return &edges[0..num_edges];
}

fn graphify(
    grid: &mut [u8],
    nodes: &mut [Node],
    num_nodes: &mut usize,
    edges: &mut [Edge],
    num_edges: &mut usize,
    i: usize,
    neighbors: &[usize],
) {
    const ROW: usize = GRID_SIZE + 1;
    let id = nodes[i];
    for n in neighbors {
        if grid[*n] == b'#' { continue }
        // Follow this path until we find another crossroad.
        let mut prev = i;
        let mut pos = *n;
        let mut len = 0;
        let mut hills = 0;
        loop {
            match grid[pos] {
                b'.' => {},
                b'>' if prev + 1 == pos => hills |= DOWNHILL,
                b'v' if prev + ROW == pos => hills |= DOWNHILL,
                b'<' if prev - 1 == pos => hills |= DOWNHILL,
                b'^' if prev - ROW == pos => hills |= DOWNHILL,
                b'>' if prev - 1 == pos => hills |= UPHILL,
                b'v' if prev - ROW == pos => hills |= UPHILL,
                b'<' if prev + 1 == pos => hills |= UPHILL,
                b'^' if prev + ROW == pos => hills |= UPHILL,
                _ => panic!("bad grid"),
            }
            len += 1;
            if pos == END {
                edges[*num_edges] = (id, END_NODE, hills, len);
                *num_edges += 1;
                break;
            }
            // Enumerate the neighbors of the current cell, excluding the one we came from.
            let mut next = [0; 3];
            let mut num_next = 0;
            for n in [pos - ROW, pos - 1, pos + 1, pos + ROW] {
                if n == prev || grid[n] == b'#' { continue }
                next[num_next] = n;
                num_next += 1;
            }
            if num_next == 1 {
                // Not at the end of the path yet.
                prev = pos;
                pos = next[0];
            } else if num_next > 1 {
                // Found a crossroad.
                if nodes[pos] == 0 {
                    nodes[pos] = *num_nodes as Node;
                    *num_nodes += 1;
                    graphify(grid, nodes, num_nodes, edges, num_edges, pos, &next[0..num_next]);
                }
                let end_id = nodes[pos];
                if id < end_id {
                    edges[*num_edges] = (id, end_id, hills, len);
                    *num_edges += 1;
                }
                break;
            } else {
                // Dead end. No edges to add.
                break;
            }
        }
    }
}

// `m[a][i]` is a pair `(b, n)` indicating an edge of length `n` between nodes `a` and `b`. If `n`
// is `0`, it means that no such edge exists.
type AdjacencyMatrix = [[(Node, u16); 4]; MAX_NODES];

fn part1(edges: &[Edge]) -> AdjacencyMatrix {
    let mut neighbors = [0; MAX_NODES];
    let mut result = [[(0, 0); 4]; MAX_NODES];
    for (a, b, hills, n) in edges {
        if hills & UPHILL == 0 {
            let i = neighbors[*a as usize];
            neighbors[*a as usize] += 1;
            result[*a as usize][i] = (*b, *n);
        }
        if hills & DOWNHILL == 0 {
            let i = neighbors[*b as usize];
            neighbors[*b as usize] += 1;
            result[*b as usize][i] = (*a, *n);
        }
    }
    return result;
}

fn part2(edges: &[Edge]) -> AdjacencyMatrix {
    let mut neighbors = [0; MAX_NODES];
    let mut result = [[(0, 0); 4]; MAX_NODES];
    for (a, b, _, n) in edges {
        let i = neighbors[*a as usize];
        neighbors[*a as usize] += 1;
        result[*a as usize][i] = (*b, *n);
        let j = neighbors[*b as usize];
        neighbors[*b as usize] += 1;
        result[*b as usize][j] = (*a, *n);
    }
    // Find the unique node connected to the exit.
    let s = result[START_NODE as usize][0].0 as usize;
    let e = result[END_NODE as usize][0].0 as usize;
    // The input forms a perfect grid of    S - N - N - N . .
    // nodes with the corners chopped off.      |   |   | \ .
    // Every perimeter node has exactly 3       N - N - N - N
    // neighbours and all internal nodes        |   |   |   |
    // have four. We never want to go           N - N - N - N
    // backwards around the perimeter,          . \ |   |   |
    // back towards S, because that will        . . N - N - N - E
    // always result in a dead-end.
    for i in 0..2 {
        let mut pos = result[s][i].0 as usize;
        loop {
            // Find the next node around the perimeter.
            let next = result[pos]
                .iter()
                .find(|(next, _)| neighbors[*next as usize] == 3)
                .unwrap().0 as usize;
            // Remove the backwards edge.
            let b = result[next as usize].iter().position(|(x, _)| *x as usize == pos).unwrap();
            result[next as usize].copy_within(b + 1..4, b);
            result[next as usize][3] = (0, 0);
            if next == e { break }
            pos = next;
        }
    }
    return result;
}

fn longest_path(m: &AdjacencyMatrix) -> u16 {
    // `path[i]` is a tuple `(x, n, i)` where `x` is the node at the end of the path, `n` is the
    // length of the path up until that point, and `i` is the index of the next neighbor of `x` to
    // explore when recursing downwards.
    let mut path = [(0, 0, 0); 36];
    path[0] = (START_NODE, 0, 0);
    let mut path_nodes = 1;
    let mut visited: u64 = 1 << START_NODE;
    let mut best = 0;
    while path_nodes > 0 {
        let (pos, path_len, i) = &mut path[path_nodes - 1];
        if *i < 4 && m[*pos as usize][*i].1 != 0 {
            let (next, n) = m[*pos as usize][*i];
            *i += 1;
            if next == END_NODE {
                best = best.max(*path_len + n);
            } else if visited & (1 << next) == 0 {
                visited |= 1 << next;
                path[path_nodes] = (next, *path_len + n, 0);
                path_nodes += 1;
            }
        } else {
            visited ^= 1 << *pos;
            path_nodes -= 1;
        }
    }
    return best;
}

fn main() {
    let mut edge_buffer = [(0, 0, 0, 0); MAX_EDGES];
    let edges = read_input(&mut edge_buffer);

    print!("{}\n{}\n", longest_path(&part1(&edges)), longest_path(&part2(&edges)));
}
