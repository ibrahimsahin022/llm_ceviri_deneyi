use std::io::{self, Read};

// Seviye C: yalnizca "2/4 test basarisiz" bilgisi var.
// C kaynagini yeniden inceledim: `unsigned int h` uzerinde h = h*31 + c.
// C'de unsigned tasma TANIMLIDIR (mod 2^32 sarar), Rust'ta debug
// derlemede + ve * PANIK verir. Bu yuzden wrapping_* kullaniyorum.
// Ayrica girdi ham BAYT olarak okunmali: C fgets bayt okur ve
// (unsigned char) cast'i yapar, UTF-8 dogrulamasi yoktur.
fn main() {
    let mut buf = Vec::new();
    if io::stdin().read_to_end(&mut buf).is_err() {
        return;
    }
    let mut h: u32 = 0;
    for &b in buf.iter() {
        if b == b'\n' || b == b'\r' {
            break;
        }
        h = h.wrapping_mul(31).wrapping_add(b as u32);
    }
    println!("{}", h);
}
