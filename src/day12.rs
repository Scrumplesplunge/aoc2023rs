use std::io;

const MAX_INPUT_PATTERN_LENGTH: usize = 20;
const MAX_INPUT_GROUPS: usize = 6;

const MAX_PATTERN_LENGTH: usize = MAX_INPUT_PATTERN_LENGTH * 5 + 4;
const MAX_GROUPS: usize = MAX_INPUT_GROUPS * 5;

type Ascii = [u8];
type Table = [[u64; MAX_GROUPS + 1]; MAX_PATTERN_LENGTH + 1];

fn arrangements(pattern: &Ascii, groups: &[u8]) -> Table {
    let n = pattern.len();
    let m = groups.len();
    // `bad_run[i]` is the number of consecutive positions, starting at `i`,
    // which are either bad ('#') or unknown ('?').
    let mut bad_run = [0; MAX_PATTERN_LENGTH + 1];
    for i in (0 .. n).rev() {
        if pattern[i] != b'.' {
            bad_run[i] = bad_run[i + 1] + 1;
        }
    }
    // `a[i][j]` is the number of arrangements for `&pattern[i..]` and
    // `&groups[j..]`.
    let mut a = [[0; MAX_GROUPS + 1]; MAX_PATTERN_LENGTH + 1];
    // The empty pattern has one matching arrangement for zero groups, and no
    // matching arrangements for any nonzero number of groups (which are covered
    // by the default initialization to 0).
    a[n][m] = 1;
    for i in (0..n).rev() {
        a[i][m] = if pattern[i] == b'#' { 0 } else { a[i + 1][m] };
    }
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            if pattern[i] == b'.' {
                // Empty space: skip to the next group.
                a[i][j] = a[i + 1][j];
            } else {
                let c = groups[j] as usize;
                let end = i + c;
                if end <= n {
                    // True if a sequence of `c` bad entries can fit before the
                    // next empty space.
                    let can_fit = bad_run[i] >= c;
                    // True if a sequence starting at `i` could stop at `end`.
                    let can_stop = end == n || pattern[end] != b'#';
                    let mut count = 0;
                    // Calculate the number of arrangements where the `j`th
                    // sequence starts at position `i`.
                    if can_fit && can_stop {
                        count += a[n.min(end + 1)][j + 1]
                    }
                    // Calculate the number of arrangements where the `j`th
                    // sequence starts strictly after position `i`.
                    if pattern[i] == b'?' {
                        count += a[i + 1][j]
                    }
                    a[i][j] = count;
                }
            }
        }
    }
    return a;
}

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        // Parse the input line.
        let (p, list) = line.split_once(" ").unwrap();
        if !p.is_ascii() || p.len() > MAX_INPUT_PATTERN_LENGTH {
            panic!("bad pattern");
        }
        let mut pattern_buffer = [0; MAX_PATTERN_LENGTH];
        let pattern_len = p.len();
        pattern_buffer[0..pattern_len].copy_from_slice(p.as_bytes());
        let mut num_groups = 0;
        let mut group_buffer = [0; MAX_GROUPS];
        for n in list.split(",").map(|x| x.parse().unwrap()) {
            if num_groups == MAX_INPUT_GROUPS {
                panic!("too many groups")
            }
            group_buffer[num_groups] = n;
            num_groups += 1;
        }
        // Unfold the list into five copies (with patterns separated by '?').
        for i in 1..5 {
            group_buffer.copy_within(0..num_groups, i * num_groups);
            pattern_buffer[i * (pattern_len + 1) - 1] = b'?';
            pattern_buffer.copy_within(0..pattern_len, i * (pattern_len + 1));
        }
        let pattern = &pattern_buffer[0..5 * pattern_len + 4];
        let groups = &group_buffer[0..5 * num_groups];

        // Compute the table of arrangement counts for different suffixes of the
        // pattern and the list of groups. `table[i][j]` is the number of
        // arrangements for `&pattern[i..]` and `&groups[j..]`.
        let table = arrangements(pattern, groups);
        part1 += table[4 * pattern_len + 4][4 * num_groups];
        part2 += table[0][0];
    }
    print!("{}\n{}\n", part1, part2);
}
