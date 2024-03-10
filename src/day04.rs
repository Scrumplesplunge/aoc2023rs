use std::io;
use std::str;

fn skip_whitespace(input: &mut &str) {
    *input = input.trim_start_matches(|c: char| c == ' ');
}

fn consume_prefix(input: &mut &str, prefix: &str) {
    if !input.starts_with(prefix) {
        panic!("expected \"{}\"", prefix);
    }
    *input = &input[prefix.len()..];
}

fn parse_int(input: &mut &str) -> u32 {
    let l = input.find(|c: char| !c.is_digit(10))
                 .unwrap_or(input.len());
    if l == 0 { panic!("expected number") }
    let mut n = 0;
    let digits = &input[0 .. l];
    for c in digits.chars().map(|c| c.to_digit(10).unwrap()) {
        n = 10 * n + c;
    }
    *input = &input[l..];
    return n;
}

fn parse_wins(input: &mut &str) -> u32 {
    consume_prefix(&mut *input, "Card ");
    skip_whitespace(&mut *input);
    let _: u32 = parse_int(&mut *input);
    consume_prefix(&mut *input, ": ");
    let mut win_buffer = [0; 10];
    let mut winning_numbers = 0;
    skip_whitespace(&mut *input);
    while input.starts_with(|c: char| c.is_digit(10)) {
        if winning_numbers == 10 { panic!("Too many winning numbers") }
        win_buffer[winning_numbers] = parse_int(&mut *input);
        winning_numbers += 1;
        skip_whitespace(&mut *input);
    }
    let wins = &win_buffer[0..winning_numbers];
    consume_prefix(&mut *input, "|");
    let mut num_wins = 0;
    skip_whitespace(&mut *input);
    while input.starts_with(|c: char| c.is_digit(10)) {
        if wins.contains(&parse_int(&mut *input)) { num_wins += 1 }
        skip_whitespace(&mut *input);
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
        let mut input = line.as_str();
        let num_wins = parse_wins(&mut input);
        if input != "" { panic!("Trailing characters: {}", input) }

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
