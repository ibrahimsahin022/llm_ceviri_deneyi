use std::io::{self, Read};

// IYILESTIRME (Round 2): C kodundaki switch/case KASITLI OLARAK dusuyordu
// (case 4 -> 3 -> 2 -> 1 sirasiyla bonus'a ekleme yapip break yok), yani
// level=4 icin bonus = 8+4+2+1 = 15 olmasi gerekiyordu. Ilk ceviri Rust'in
// match'inin varsayilan olarak dusmedigini goz ardi edip yalnizca eslesen
// kolu calistirdi (level=4 icin yanlislikla yalnizca 8 dondu). Duzeltme:
// her kolun ONCEKI kollarin toplamini da icerecek sekilde acikca yazilmasi.
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let level: i32 = input.trim().parse().unwrap();
    let bonus = match level {
        4 => 8 + 4 + 2 + 1,
        3 => 4 + 2 + 1,
        2 => 2 + 1,
        1 => 1,
        _ => 0,
    };
    println!("{}", bonus);
}
