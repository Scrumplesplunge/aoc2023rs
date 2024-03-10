use std::io;
use std::str;

fn parse_wins(input: &str) -> u32 {
    let input = input.split_once(':').unwrap().1;
    let mut win_buffer = [0; 10];
    let mut winning_numbers = 0;
    let (win_str, values_str) = input.split_once('|').unwrap();
    for n in win_str.split_ascii_whitespace().map(|n| n.parse().unwrap()) {
        if winning_numbers == 10 { panic!("Too many winning numbers") }
        win_buffer[winning_numbers] = n;
        winning_numbers += 1;
    }
    let wins = &win_buffer[0..winning_numbers];
    let mut num_wins = 0;
    for n in values_str.split_ascii_whitespace().map(|n| n.parse().unwrap()) {
        if wins.contains(&n) { num_wins += 1 }
    }
    return num_wins;
}

fn main() {
    // Parse the cards.
    let mut part1 = 0;
    let mut part2 = 0;
    let mut counts = [1; 10];
    let mut i = 0;
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        let num_wins = parse_wins(&line);

        // Part 1: accumulate points based on the number of wins.
        part1 += (1 << num_wins) >> 1;

        // Part 2: accumulate cards.
        let n = counts[i];
        part2 += n;
        counts[i] = 1;
        i = if i < 9 { i + 1 } else { 0 };
        for j in 0 .. num_wins as usize {
            let k = if i + j < 10 { i + j } else { i + j - 10 };
            counts[k] += n;
        }
    }
    print!("{}\n{}\n", part1, part2);
}
