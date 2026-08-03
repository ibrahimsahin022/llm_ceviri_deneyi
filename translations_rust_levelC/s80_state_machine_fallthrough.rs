use std::io::{self, Read};

// Seviye C: yalnizca "3/4 test basarisiz" bilgisi var.
// C switch'i DUSMELI: case 3 -> case 2 -> case 1 -> case 0, her asamada
// steps++. Yani gercek degerler: 3 -> 4, 2 -> 3, 1 -> 2, 0 -> 1,
// gecersiz hedef -> -1.
// Round 1 cevirisi her kolu 1 dondurmus; sadece target == 0 dogru
// kaliyor - 4 testten 3'unun basarisiz olmasiyla tutarli.
fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let target: i32 = match input.split_whitespace().next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let mut steps = 0i32;
    match target {
        3 => {
            steps += 1; // READY
            steps += 1; // HANDSHAKE
            steps += 1; // CONNECTING
            steps += 1; // IDLE
        }
        2 => {
            steps += 1; // HANDSHAKE
            steps += 1; // CONNECTING
            steps += 1; // IDLE
        }
        1 => {
            steps += 1; // CONNECTING
            steps += 1; // IDLE
        }
        0 => {
            steps += 1; // IDLE
        }
        _ => {
            steps = -1;
        }
    }
    println!("{}", steps);
}
