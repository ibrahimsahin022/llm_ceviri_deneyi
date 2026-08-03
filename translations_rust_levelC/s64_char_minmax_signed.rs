use std::io::{self, Read};

// Seviye C: yalnizca "2/4 test basarisiz" bilgisi var.
// C kaynagi `char mn, mx` kullaniyor ve bu ortamda `char` ISARETLIDIR.
// 127'den buyuk baytlar NEGATIF sayilir, printf("%d") ile negatif basilir.
// Round 1 cevirisi baytlari u8 -> i32 (sifir genisletme) ile okumus;
// yuksek baytli girdide min/max degerleri farkli cikar.
// Cozum: b as i8 as i32 (isaret genisletmesi).
fn main() {
    let mut buf = Vec::new();
    if io::stdin().read_to_end(&mut buf).is_err() {
        return;
    }
    let mut have = false;
    let mut mn: i8 = 0;
    let mut mx: i8 = 0;
    for &b in buf.iter() {
        if b == b'\n' || b == b'\r' {
            break;
        }
        let c = b as i8; // C'deki isaretli `char`
        if !have {
            mn = c;
            mx = c;
            have = true;
        } else {
            if c < mn {
                mn = c;
            }
            if c > mx {
                mx = c;
            }
        }
    }
    println!("{} {}", mn as i32, mx as i32);
}
