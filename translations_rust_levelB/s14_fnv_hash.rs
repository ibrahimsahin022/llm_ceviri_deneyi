use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let line = input.lines().next().unwrap_or("");
    // Seviye B duzeltme: panik mesaji "attempt to multiply with overflow"
    // goruldu (diff/beklenen-cikti gorulmedi). Hash fonksiyonlarinda tasmanin
    // kasitli/tanimli davranis oldugu bilindiginden wrapping aritmetigine gecildi.
    let mut hash: u32 = 2166136261;
    for b in line.bytes() {
        hash ^= b as u32;
        hash = hash.wrapping_mul(16777619);
    }
    println!("{}", hash);
}
