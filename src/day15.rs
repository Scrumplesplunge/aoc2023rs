use std::io;
use std::io::Read;

fn hash(a: &[u8]) -> usize {
    let mut value: u8 = 0;
    for x in a {
        value = value.wrapping_add(*x);
        value = value.wrapping_mul(17);
    }
    return value as usize;
}

const BUCKET_SIZE: usize = 8;

type Power = u8;
type Record<'a> = (&'a [u8], Power);

fn set<'a>(bucket: &mut [Record<'a>; BUCKET_SIZE], label: &'a [u8], power: Power) {
    for i in 0..BUCKET_SIZE {
        if bucket[i].0 == label {
            bucket[i].1 = power;
            return;
        } else if bucket[i].0.len() == 0 {
            bucket[i] = (label, power);
            return;
        }
    }
    panic!("bucket full");
}

fn remove(bucket: &mut [Record; BUCKET_SIZE], label: &[u8]) {
    if let Some(i) = bucket.iter().position(|r| r.0 == label) {
        for j in i + 1..BUCKET_SIZE {
            bucket[j - 1] = bucket[j];
        }
        bucket[BUCKET_SIZE - 1] = (&[], 0);
    }
}

fn main() {
    let mut buffer = [0; 24 * 1024];
    let length = io::stdin().read(&mut buffer).unwrap();
    if length == 0 || buffer[length - 1] != b'\n' {
        panic!("bad input")
    }
    let input = &buffer[0..length - 1];

    let mut part1 = 0;
    let mut buckets: [[Record; BUCKET_SIZE]; 256] = [[(&[], 0); BUCKET_SIZE]; 256];
    for entry in input.split(|b| *b == b',') {
        part1 += hash(entry);
        match entry {
            [label @ .., b'=', x] => {
                let h = hash(label);
                set(&mut buckets[h], label, x - b'0');
            }
            [label @ .., b'-'] => {
                let h = hash(label);
                remove(&mut buckets[h], label);
            }
            _ => panic!("bad entry"),
        }
    }
    let mut part2 = 0;
    for b in 0..256 {
        for s in 0..BUCKET_SIZE {
            part2 += (1 + b) * (1 + s) * buckets[b][s].1 as usize;
        }
    }
    print!("{}\n{}\n", part1, part2);
}
