use std::io::{self, Read};

const NUM_WORDS: usize = 4;
const BITS_PER_WORD: usize = 64;

fn main() {
    let mut words: [u64; NUM_WORDS] = [0; NUM_WORDS];

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    for _ in 0..n {
        let op = it.next().unwrap();
        let bit: i64 = it.next().unwrap().parse().unwrap();
        if bit < 0 || bit as usize >= NUM_WORDS * BITS_PER_WORD {
            continue;
        }
        let bit = bit as usize;
        let w = bit / BITS_PER_WORD;
        let b = bit % BITS_PER_WORD;
        match op {
            "S" => words[w] |= 1u64 << b,
            "C" => words[w] &= !(1u64 << b),
            _ => println!("{}", (words[w] >> b) & 1),
        }
    }

    let count: u32 = words.iter().map(|w| w.count_ones()).sum();
    println!("count={}", count);
}
