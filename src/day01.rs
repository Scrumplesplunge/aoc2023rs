use std::io;

// Returns an iterator over all suffixes of a string.
fn suffixes(s: &str) -> impl Iterator<Item = &str> + DoubleEndedIterator {
    return s.char_indices().map(|(i, _)| &s[i..]);
}

// Returns the numerical digit represented by a suffix of the given string, if
// any. The representation can be a digit, or a word naming a digit.
fn digit_prefix(s: &str) -> Option<u32> {
    if let Some(i) = s.chars().nth(0).unwrap().to_digit(10) { return Some(i); }
    if s.starts_with("zero") { return Some(0); }
    if s.starts_with("one") { return Some(1); }
    if s.starts_with("two") { return Some(2); }
    if s.starts_with("three") { return Some(3); }
    if s.starts_with("four") { return Some(4); }
    if s.starts_with("five") { return Some(5); }
    if s.starts_with("six") { return Some(6); }
    if s.starts_with("seven") { return Some(7); }
    if s.starts_with("eight") { return Some(8); }
    if s.starts_with("nine") { return Some(9); }
    return None;
}

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        // Part 1
        let first = line.chars().find_map(|c| c.to_digit(10)).unwrap();
        let last = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
        part1 += 10 * first + last;

        // Part 2
        let first = suffixes(&line).find_map(digit_prefix).unwrap();
        let last = suffixes(&line).rev().find_map(digit_prefix).unwrap();
        part2 += 10 * first + last;
    }
    print!("{}\n{}\n", part1, part2);
}
