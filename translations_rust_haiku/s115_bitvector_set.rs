use std::io;

const NUM_WORDS: usize = 4;
const BITS_PER_WORD: usize = 64;

fn main() {
    let mut words: [u64; NUM_WORDS] = [0, 0, 0, 0];

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut parts = line.split_whitespace();
        let op = parts.next().unwrap();
        let bit: usize = parts.next().unwrap().parse().unwrap();

        if bit >= NUM_WORDS * BITS_PER_WORD {
            continue;
        }

        let w = bit / BITS_PER_WORD;
        let b = bit % BITS_PER_WORD;

        if op == "S" {
            words[w] |= 1u64 << b;
        } else if op == "C" {
            words[w] &= !(1u64 << b);
        } else {
            let bit_val = (words[w] >> b) & 1u64;
            println!("{}", bit_val);
        }
    }

    let mut count = 0;
    for w in 0..NUM_WORDS {
        let mut v = words[w];
        while v > 0 {
            count += (v & 1) as i32;
            v >>= 1;
        }
    }
    println!("count={}", count);
}
