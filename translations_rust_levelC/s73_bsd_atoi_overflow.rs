use std::io::{self, Read};

// Seviye C: yalnizca "2/4 test basarisiz" bilgisi var.
// C kaynagi biriktirici olarak `unsigned long` kullaniyor ve kaynak
// yorumu bu derleme ortaminin Windows/LLP64 oldugunu, yani
// `unsigned long`un 32 BIT (ULONG_MAX = 4294967295) oldugunu belirtiyor.
// Round 1 cevirisi u64 secmis: 2^32 sinirinda gerceklesmesi gereken
// sarma HIC olmuyor. Ayrica C'de unsigned tasma tanimli (mod 2^32),
// Rust'ta * ve + debug'da panik verir.
// Cozum: u32 + wrapping_*.
fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    // scanf("%63s") -> ilk bosluksuz belirtec, en fazla 63 karakter
    let tok = match input.split_whitespace().next() {
        Some(s) => s,
        None => return,
    };
    let bytes = tok.as_bytes();
    let limit = bytes.len().min(63);
    let mut v: u32 = 0;
    for &c in &bytes[..limit] {
        if c < b'0' || c > b'9' {
            break;
        }
        v = v.wrapping_mul(10).wrapping_add((c - b'0') as u32);
    }
    println!("{}", v);
}
