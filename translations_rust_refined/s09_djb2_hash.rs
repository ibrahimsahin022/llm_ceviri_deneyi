use std::io::{self, Read};

// IYILESTIRME (Round 2): C'deki unsigned tamsayi tasmasi TANIMLIDIR (mod 2^32 sarar).
// Rust'ta '*' ve '+' debug modda tasmada panic verir. Ayni semantigi elde etmek icin
// wrapping_mul / wrapping_add kullanildi.
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let line = input.lines().next().unwrap_or("");
    let mut hash: u32 = 5381;
    for b in line.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(b as u32);
    }
    println!("{}", hash);
}
