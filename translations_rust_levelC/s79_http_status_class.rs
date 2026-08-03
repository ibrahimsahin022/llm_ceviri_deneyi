use std::io::{self, Read};

// Seviye C: yalnizca "3/4 test basarisiz" bilgisi var.
// C switch'i DUSMELI (fallthrough) ve KUMULATIF:
//   case 5: score += 8; -> case 4: += 4 -> case 3: += 2 -> case 2: += 1
// Yani gercek degerler: 5 -> 15, 4 -> 7, 3 -> 3, 2 -> 1, digerleri 0.
// Round 1 cevirisi Rust match'ini dusmeyen sekilde yazip her kolu tek
// basina (8/4/2/1) dondurmus; yalnizca tier == 2 dogru kaliyor - bu da
// 4 testten 3'unun basarisiz olmasiyla tutarli.
// Cozum: dusmeyi acikca yeniden uretiyorum.
fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let code: i32 = match input.split_whitespace().next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let tier = code / 100; // C'de bolme sifira dogru kirpar, Rust'ta da oyle
    let mut score = 0i32;
    match tier {
        5 => {
            score += 8;
            score += 4;
            score += 2;
            score += 1;
        }
        4 => {
            score += 4;
            score += 2;
            score += 1;
        }
        3 => {
            score += 2;
            score += 1;
        }
        2 => {
            score += 1;
        }
        _ => {
            score = 0;
        }
    }
    println!("{}", score);
}
