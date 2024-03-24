use std::io;
use std::io::Read;
use num_integer;

const MAX_EDGES: usize = 256;
const MAX_NODES: usize = 64;

type NodeId = u8;
const NULL_NODE: NodeId = MAX_NODES as NodeId;

#[derive(Copy, Clone)]
struct Node<'a> {
    kind: u8,
    outs: &'a [NodeId],
}

fn get_index(name: &[u8]) -> usize {
    match name {
        b"broadcaster" => 0,
        [a @ b'a'..=b'z', b @ b'a'..=b'z'] => {
            ((a - b'a') as usize + 1) * 26 + ((b - b'a') as usize + 1)
        }
        _ => panic!("bad id"),
    }
}

fn get_or_alloc_id(name: &[u8], node_ids: &mut [NodeId], num_nodes: &mut usize) -> NodeId {
    let index = get_index(name);
    if node_ids[index] == NULL_NODE {
        node_ids[index] = *num_nodes as NodeId;
        *num_nodes += 1;
    }
    return node_ids[index];
}

fn read_input<'a, 'b>(
    nodes: &'a mut [Node<'b>],
    mut edges: &'b mut [NodeId],
) -> (&'a [Node<'b>], NodeId) {
    let mut buffer = [0; 1024];
    let len = io::stdin().read(&mut buffer).unwrap();
    if len == 0 || buffer[len - 1] != b'\n' { panic!("bad input") }
    let input = &buffer[0 .. len - 1];

    let mut node_ids = [NULL_NODE; 27 * 26 + 1];
    let mut num_nodes = 0;
    get_or_alloc_id(b"broadcaster", &mut node_ids, &mut num_nodes);

    for line in input.split(|b| *b == b'\n') {
        let (kind, name, tail): (u8, &[u8], &[u8]) = if line.starts_with(b"broadcaster") {
            (b'b', b"broadcaster", &line[11..])
        } else {
            (line[0], &line[1..3], &line[3..])
        };
        let id = get_or_alloc_id(name, &mut node_ids, &mut num_nodes);
        let mut num_outs = 0;
        for out in tail.strip_prefix(b" ->").unwrap().split(|b| *b == b',') {
            if out[0] != b' ' { panic!("bad line") }
            let out = &out[1..];
            edges[num_outs] = get_or_alloc_id(out, &mut node_ids, &mut num_nodes);
            num_outs += 1;
        }
        let (outs, free) = edges.split_at_mut(num_outs);
        edges = free;
        nodes[id as usize] = Node{kind: kind, outs: outs};
    }

    let rx = node_ids[get_index(b"rx")];
    if rx == NULL_NODE { panic!("no rx node") }
    nodes[rx as usize].kind = b'r';
    return (&nodes[0..num_nodes], rx);
}

fn ham(mut x: u16) -> u64 {
    const M1: u16 = 0b0101010101010101;
    const M2: u16 = 0b0011001100110011;
    const M4: u16 = 0b0000111100001111;
    const M8: u16 = 0b0000000011111111;
    x = (x & M1) + ((x >> 1) & M1);
    x = (x & M2) + ((x >> 2) & M2);
    x = (x & M4) + ((x >> 4) & M4);
    x = (x & M8) + ((x >> 8) & M8);
    return x as u64;
}

fn main() {
    let mut node_buffer = [Node{kind: b'?', outs: &[]}; MAX_NODES];
    let mut edge_buffer = [0; MAX_EDGES];
    let (nodes, rx) = read_input(&mut node_buffer, &mut edge_buffer);

    // The input graph follows a very strict format:
    //
    //   * Nothing sends pulses to the broadcaster (except the button).
    for node in nodes {
        for out in node.outs {
            if *out == 0 { panic!("something sends pulses to the broadcaster") }
        }
    }
    //   * The broadcaster sends pulses to four "root" flip-flop module.
    if nodes[0].outs.len() != 4 { panic!("broadcaster does not have 4 outputs") }
    for out in nodes[0].outs {
        if nodes[*out as usize].kind != b'%' { panic!("broadcaster recipient is not a flip-flop") }
    }
    //   * Each "root" flip-flop is the start of a chain of 12 consecutive flip-flops, each one
    //     feeding into the next.
    //   * Each chain is paired with a single "comparator" conjunction module, which:
    //     * Always has a mutual connection with the root flip-flop (i.e. they both send pulses to
    //       each other).
    //     * Either sends pulses to, or receives pulses from, every other flip-flop in the chain.
    let mut chains = [(0, [0; 12]); 4];
    for (root, (comparator, chain)) in nodes[0].outs.iter().zip(chains.iter_mut()) {
        chain[0] = *root;
        let mut i = 1;
        let mut node = &nodes[*root as usize];
        if let Some(c) = node.outs.iter().find(|i| nodes[**i as usize].kind == b'&') {
            *comparator = *c;
        } else {
            panic!("chain root not connected to a comparator");
        }
        while i < 12 {
            let next_id = match node.outs {
                [a] => *a,
                [a, b] if *a == *comparator => *b,
                [a, b] if *b == *comparator => *a,
                _ => panic!(
                    "chain node should output to next flip-flop and optionally to the comparator"
                ),
            };
            chain[i] = next_id;
            i += 1;
            node = &nodes[next_id as usize];
            if node.kind != b'%' { panic!("chain node is not a flip-flop") }
        }
    }
    //   * Each "comparator" module feeds into a separate inverter (a conjunction module with only
    //     one input).
    let mut inverters = [0; 4];
    for ((comparator, _), inverter) in chains.iter().zip(inverters.iter_mut()) {
        let comparator = &nodes[*comparator as usize];
        if let Some(i) = comparator.outs.iter().find(|i| nodes[**i as usize].kind == b'&') {
            *inverter = *i;
        } else {
            panic!("comparator not connected to an inverter");
        }
    }
    for i in 1..4 {
        for j in 0..i {
            if inverters[i] == inverters[j] { panic!("two comparators connected to one inverter") }
        }
    }
    //   * The four inverters are the inputs for a final conjunction module, which feeds into `rx`.
    if nodes[inverters[0] as usize].outs.len() != 1 { panic!("inverter should have one output") }
    let fc = nodes[inverters[0] as usize].outs[0];
    if nodes[fc as usize].kind != b'&' {
        panic!("inverter not connected to final conjunction module");
    }
    match nodes[fc as usize].outs {
        [x] => if *x != rx { panic!("final conjunction module not connected to rx") },
        _ => panic!("final conjunction module should have exactly one output"),
    }
    for inverter in inverters {
        match nodes[inverter as usize].outs {
            [x] => if *x != fc { panic!("inverter not connected to final conjunction module") },
            _ => panic!("inverter should have one output"),
        }
    }

    // This structure gives us some guarantees:
    //
    //   * Each chain of flip-flops form a binary counter that increments for each button press.
    //   * Each comparator is connected such that when we read a specific binary number from the
    //     flip-flop chain, we get a low pulse and the entire counter is reset to 0. In the binary
    //     representation of the target number, every node which sends pulses to the comparator is
    //     a 1, and every other node is a 0.
    let mut targets: [u64; 4] = [0; 4];
    for ((comparator, chain), target) in chains.iter().zip(targets.iter_mut()) {
        for i in 0..12 {
            if let Some(_) = nodes[chain[i] as usize].outs.iter().find(|x| **x == *comparator) {
                *target |= 1 << i;
            }
        }
    }
    //   * The number of pulses created for each increment is fairly predictable:
    //     * a different number if we're at the target number, but that never happens in part 1.
    let mut high = 0;
    let mut low = 0;
    for i in 0..1000 {
        // print!("low: {}, high: {}\n", low, high);
        let changed_bits = i ^ (i + 1);
        let carries = ham(changed_bits >> 1);
        let low_changes = changed_bits >> 1;
        let high_change = low_changes + 1;
        // * one low pulse from the button
        // * four low pulses from the broadcaster
        low += 1 + 4;
        for target in targets {
            // * one low pulse for each carry in the increment, plus one high pulse
            low += carries;
            high += 1;
            // * for each flipped bit where the target requires a 1:
            //   * one pulse to the comparator for each change.
            let compare_low = ham(low_changes & target as u16);
            let compare_high = ham(high_change & target as u16);
            let compares = compare_low + compare_high;
            low += compare_low;
            high += compare_high;
            //   * one high pulse from the comparator to each target=0 bit, plus bit 0.
            high += compares * (1 + ham(target as u16 ^ 0xFFF));
            //   * one high pulse from the comparator to the inverter.
            high += compares;
            //   * one low pulse from the inverter to the final conjunction module.
            low += compares;
            //   * one high pulse to RX.
            high += compares;
        }
    }
    let part1 = high * low;

    //   * The inverters and final conjunction module ensure that `rx` only gets a pulse when all
    //     four counters reset to 0 at the same time, so we know that we need the least common
    //     multiple of the reset value for each counter to make this happen.
    let part2 = targets.iter().fold(1, |x, t| num_integer::lcm(x, *t));

    print!("{}\n{}\n", part1, part2);
}
