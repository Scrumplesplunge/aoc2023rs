use std::io;

fn ways(time: u64, distance: u64) -> u64 {
    // Part 1: find the number of values of t where
    //                      t * (time - t) > distance
    let max = time / 2;
    // Binary search for the min.
    let mut a = 1;
    let mut b = max;
    while a != b {
        let mid = (a + b) / 2;
        if mid * (time - mid) <= distance {
            a = mid + 1
        } else {
            b = mid
        }
    }
    let low = a;
    // Probe for the max.
    let mut high = 2 * max - a - 1;
    while high * (time - high) > distance { high += 1 }
    return high - low;
}

fn main() {
    let mut time_line = String::new();
    io::stdin().read_line(&mut time_line).unwrap();
    let mut distance_line = String::new();
    io::stdin().read_line(&mut distance_line).unwrap();
    let times = time_line
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap());
    let distances = distance_line
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap());
    let part1: u64 = times.zip(distances).map(|(t, d)| ways(t, d)).product();
    let time: u64 = time_line
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.chars())
        .flatten()
        .collect::<String>()
        .parse()
        .unwrap();
    let distance: u64 = distance_line
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.chars())
        .flatten()
        .collect::<String>()
        .parse()
        .unwrap();
    let part2 = ways(time, distance);
    print!("{}\n{}\n", part1, part2);
}
