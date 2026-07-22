use std::io::{self, Read};

// IYILESTIRME (Round 2): C'nin switch'i break olmadan bir sonraki case'e
// kasitli olarak duser (bracket N icin N,N-1,...,1 katkilari birikir).
// Ilk ceviri her dilimi yalnizca kendi (kumulatif olmayan) katkisiyla
// eslestirmisti. Duzeltme: her match kolu, karsilik geldigi dusme
// zincirinin toplam katkisini acikca icerir.
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bracket: i32 = input.trim().parse().unwrap();

    let tax = match bracket {
        4 => 800 + 400 + 200 + 100,
        3 => 400 + 200 + 100,
        2 => 200 + 100,
        1 => 100,
        _ => 0,
    };
    println!("{}", tax);
}
