use std::io::{self, Read};

// IYILESTIRME (Round 2): C'de unsigned tamsayi tasmasi tanimlidir (mod 2^32).
// Ilk ceviri '*' kullandigi icin Rust debug modda tasmada panic veriyordu (RE).
// wrapping_mul ile C semantigi korunur.
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let line = input.lines().next().unwrap_or("");
    let mut hash: u32 = 2166136261;
    for b in line.bytes() {
        hash ^= b as u32;
        hash = hash.wrapping_mul(16777619);
    }
    println!("{}", hash);
}
