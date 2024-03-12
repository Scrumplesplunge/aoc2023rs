use std::io;

fn extrapolate(values: &[i32]) -> (i32, i32) {
    if values.iter().all(|x| *x == 0) { return (0, 0) }
    let mut deltas = [0; 20];
    for i in 1 .. values.len() {
        deltas[i - 1] = values[i] - values[i - 1];
    }
    let (a, b) = extrapolate(&deltas[0 .. values.len() - 1]);
    return (values[0] - a, values[values.len() - 1] + b)
}

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        let mut n = 0;
        let mut values = [0; 21];
        for x in line.split_ascii_whitespace().map(|x| x.parse().unwrap()) {
            if n >= values.len() { panic!("too many") }
            values[n] = x;
            n += 1;
        }
        let (a, b) = extrapolate(&values[0..n]);
        part1 += b;
        part2 += a;
    }
    print!("{}\n{}\n", part1, part2);
}
