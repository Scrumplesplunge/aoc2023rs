use std::io;
use std::io::Read;

const GRID_SIZE: usize = 141;
const MAX_EDGES: usize = 256;

const ROW: usize = GRID_SIZE + 1;
const START: usize = 1;                  // First few bytes are "# ###"
const END: usize = ROW * GRID_SIZE - 3;  // Last few bytes are "# #\n"

type Node = u16;
const START_NODE: Node = 0;
const END_NODE: Node = 1;
const MAX_NODES: usize = 40;

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
                edges[*num_edges] = (id, end_id, hills, len);
                *num_edges += 1;
                break;
            } else {
                // Dead end. No edges to add.
                break;
            }
        }
    }
}

// `m[i][j]` is the distance from node `i` to node `j`.
type AdjacencyMatrix = [[Option<u16>; MAX_NODES]; MAX_NODES];

fn part1(edges: &[Edge]) -> AdjacencyMatrix {
    let mut result = [[None; MAX_NODES]; MAX_NODES];
    for (a, b, hills, n) in edges {
        if hills & UPHILL == 0 { result[*a as usize][*b as usize] = Some(*n) }
        if hills & DOWNHILL == 0 { result[*b as usize][*a as usize] = Some(*n) }
    }
    return result;
}

fn part2(edges: &[Edge]) -> AdjacencyMatrix {
    let mut result = [[None; MAX_NODES]; MAX_NODES];
    for (a, b, _, n) in edges {
        result[*a as usize][*b as usize] = Some(*n);
        result[*b as usize][*a as usize] = Some(*n);
    }
    return result;
}

fn longest_path(m: &AdjacencyMatrix, pos: Node) -> Option<u16> {
    if pos == END_NODE { return Some(0) }
    let mut best = None;
    // Disconnect this node.
    let mut m2 = *m;
    for i in 0..MAX_NODES { m2[i][pos as usize] = None }
    for next in 0..MAX_NODES {
        if let Some(n) = m[pos as usize][next] {
            best = match (best, longest_path(&m2, next as Node)) {
                (None, None) => None,
                (None, Some(x)) => Some(n + x),
                (x, None) => x,
                (Some(a), Some(b)) => Some(a.max(n + b)),
            }
        }
    }
    return best;
}

fn main() {
    let mut edge_buffer = [(0, 0, 0, 0); MAX_EDGES];
    let edges = read_input(&mut edge_buffer);

    print!("{}\n{}\n", longest_path(&part1(&edges), START_NODE).unwrap(),
                       longest_path(&part2(&edges), START_NODE).unwrap());
}
