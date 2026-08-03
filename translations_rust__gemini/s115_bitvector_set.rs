use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut tokens = input.split_whitespace();

    let n: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    const NUM_WORDS: usize = 4;
    const BITS_PER_WORD: usize = 64;

    let mut words = [0u64; NUM_WORDS];

    for _ in 0..n {
        let op = match tokens.next() {
            Some(s) => s,
            None => return,
        };
        let bit: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
            Some(b) => b,
            None => return,
        };

        if bit < 0 || bit >= (NUM_WORDS * BITS_PER_WORD) as i32 {
            continue;
        }

        let w = (bit as usize) / BITS_PER_WORD;
        let b = (bit as usize) % BITS_PER_WORD;

        let first_char = op.bytes().next().unwrap_or(0);
        if first_char == b'S' {
            words[w] |= 1u64 << b;
        } else if first_char == b'C' {
            words[w] &= !(1u64 << b);
        } else {
            println!("{}", (words[w] >> b) & 1);
        }
    }

    let mut count = 0;
    for w in 0..NUM_WORDS {
        let mut v = words[w];
        while v != 0 {
            count += (v & 1) as i32;
            v >>= 1;
        }
    }
    println!("count={}", count);
}
