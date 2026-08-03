use std::io::{self, Read};

// Seviye C: yalnizca "1/5 test basarisiz" bilgisi var.
// C kaynagi: acc ^= (unsigned int)(int)c;  burada c ISARETLI `char`.
// Negatif bir char once int'e ISARET GENISLETMESI ile donusur
// (ust 24 bit 1 ile dolar), sonra unsigned int'e cevrilir.
// Round 1 cevirisi u8 -> u32 (SIFIR genisletme) yapmis; yalnizca yuksek
// baytli tek bir girdide fark cikiyor olmali.
// Cozum: b as i8 as i32 as u32.
fn main() {
    let mut buf = Vec::new();
    if io::stdin().read_to_end(&mut buf).is_err() {
        return;
    }
    let mut acc: u32 = 0;
    for &b in buf.iter() {
        if b == b'\n' || b == b'\r' {
            break;
        }
        let c = b as i8 as i32; // isaret genisletmesi
        acc ^= c as u32;
    }
    println!("{}", acc);
}
