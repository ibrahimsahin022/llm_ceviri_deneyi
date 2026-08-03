use std::io::{self, Read};

// Seviye C: yalnizca "3/4 test basarisiz" bilgisi var.
// C switch'i hem COKLU ETIKETLI (case 10: case 9:) hem de DUSMELI:
//   case 10/9 -> badges++ (A) -> case 8 badges++ (B) -> case 7 badges++
//   (C) -> case 6 badges++ (D) -> break
// Gercek degerler: tier 10 veya 9 -> 4, 8 -> 3, 7 -> 2, 6 -> 1,
// digerleri 0.
// Round 1 cevirisi her kolu 1 dondurmus; yalnizca tier == 6 dogru
// kaliyor - 4 testten 3'unun basarisiz olmasiyla tutarli.
fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let grade: i32 = match input.split_whitespace().next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let tier = grade / 10; // 0..10
    let mut badges = 0i32;
    match tier {
        10 | 9 => {
            badges += 1; // A
            badges += 1; // B
            badges += 1; // C
            badges += 1; // D
        }
        8 => {
            badges += 1; // B
            badges += 1; // C
            badges += 1; // D
        }
        7 => {
            badges += 1; // C
            badges += 1; // D
        }
        6 => {
            badges += 1; // D
        }
        _ => {
            badges = 0;
        }
    }
    println!("{}", badges);
}
