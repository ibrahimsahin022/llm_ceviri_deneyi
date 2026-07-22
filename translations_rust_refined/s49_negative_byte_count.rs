use std::io::{self, Read};

// IYILESTIRME (Round 2): C'nin `char`'i bu platformda isaretlidir; 127'den
// buyuk baytlar (b as i8) ile isaretli yeniden yorumlandiginda negatif olur.
// Ilk ceviri `b as i32` (u8 -> i32 sifir-genisletme) kullandigi icin hicbir
// zaman negatif uretmiyordu. Duzeltme: once `as i8` ile isaretli donusum,
// sonra i32'ye genislet.
fn main() {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf).unwrap();

    let mut negative_count = 0;
    let mut total = 0;
    for &b in &buf {
        let ch: i32 = (b as i8) as i32;
        if ch < 0 {
            negative_count += 1;
        }
        total += 1;
    }
    println!("{} {}", total, negative_count);
}
