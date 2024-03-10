use std::io;

fn map(destination: u64, source: u64, range: u64,
       input: &mut Vec<(u64, u64)>, output: &mut Vec<(u64, u64)>) {
    for i in (0 .. input.len()).rev() {
        let (a, b) = input[i];
        if b <= source || source + range <= a {
            // No overlap
            continue;
        } else {
            // Overlap in [a2, b2)
            let a2 = a.max(source);
            let b2 = b.min(source + range);
            if a2 == b2 { continue }
            output.push((a2 - source + destination, b2 - source + destination));
            if a < a2 && b2 < b {
                input[i] = (a, a2);
                input.push((b2, b));
            } else if a < a2 {
                input[i] = (a, a2);
            } else if b2 < b {
                input[i] = (b2, b);
            } else {
                input[i] = input[input.len() - 1];
                input.pop();
            }
        }
    }
}

fn main() {
    let mut seed_line = String::new();
    io::stdin().read_line(&mut seed_line).unwrap();
    let seeds: Vec<u64> = seed_line
        .split_ascii_whitespace()
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect();
    let mut part1_input: Vec<(u64, u64)> = seeds
        .iter()
        .map(|x| (*x, *x + 1))
        .collect();
    let mut part2_input: Vec<(u64, u64)> = seeds
        .chunks_exact(2)
        .map(|ab| (ab[0], ab[0] + ab[1]))
        .collect();
    let mut part2_output = Vec::new();
    let mut part1_output = Vec::new();
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        if line.starts_with(|c: char| c.is_lowercase()) {
            part1_input.append(&mut part1_output);
            part2_input.append(&mut part2_output);
        } else if line == "" {
            continue;
        } else {
            let mut parts = line
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap());
            let destination: u64 = parts.next().unwrap();
            let source: u64 = parts.next().unwrap();
            let range: u64 = parts.next().unwrap();
            map(destination, source, range, &mut part1_input, &mut part1_output);
            map(destination, source, range, &mut part2_input, &mut part2_output);
        }
    }
    part1_input.append(&mut part1_output);
    part2_input.append(&mut part2_output);
    let part1 = part1_input.iter().map(|(a, _)| a).min().unwrap();
    let part2 = part2_input.iter().map(|(a, _)| a).min().unwrap();
    print!("{}\n{}\n", part1, part2);
}
