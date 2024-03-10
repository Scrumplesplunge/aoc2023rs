use std::io;

fn main() {
    let mut seeds = String::new();
    io::stdin().read_line(&mut seeds).unwrap();
    let mut input: Vec<u64> = seeds
        .split_ascii_whitespace()
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect();
    let mut output = Vec::new();
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        if line.starts_with(|c: char| c.is_lowercase()) {
            input.append(&mut output);
        } else if line == "" {
            continue;
        } else {
            let mut parts = line
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap());
            let destination: u64 = parts.next().unwrap();
            let source: u64 = parts.next().unwrap();
            let range: u64 = parts.next().unwrap();
            if parts.next() != None { panic!("Trailing characters") }
            let mut j = 0;
            for i in 0 .. input.len() {
                if source <= input[i] && input[i] < source + range {
                    output.push(input[i] - source + destination);
                } else {
                    input[j] = input[i];
                    j += 1;
                }
            }
            input.truncate(j);
        }
    }
    input.append(&mut output);
    print!("{}\n", input.iter().min().unwrap());
}
