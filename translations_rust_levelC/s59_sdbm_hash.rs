use std::io::{self, Read};

// Seviye C: yalnizca "3/4 test basarisiz" bilgisi var.
// sdbm formulu: hash = c + (hash << 6) + (hash << 16) - hash
// Toplama/cikarma `unsigned int` uzerinde mod 2^32 sarar (C'de tanimli).
// Rust'ta + ve - debug'da panik verir -> wrapping_* kullaniyorum.
// Kaydirmalar (<<6, <<16) u32'de zaten ust bitleri atar, panik vermez.
// Girdi ham bayt olarak okunuyor ((unsigned char) cast'ine karsilik gelir).
fn main() {
    let mut buf = Vec::new();
    if io::stdin().read_to_end(&mut buf).is_err() {
        return;
    }
    let mut hash: u32 = 0;
    for &b in buf.iter() {
        if b == b'\n' || b == b'\r' {
            break;
        }
        let c = b as u32;
        hash = c
            .wrapping_add(hash << 6)
            .wrapping_add(hash << 16)
            .wrapping_sub(hash);
    }
    println!("{}", hash);
}
