use std::io::{self, Read};

// IYILESTIRME (Round 2): C, karakterleri BAYT bazinda sayar.
// Ilk cevirideki input.chars() Unicode karakter bazinda saydigi icin
// cok baytli (UTF-8) girdilerde karakter sayisi C'den farkli cikiyordu (FE).
// C ile ayni davranis icin baytlar uzerinden sayilir.
fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let mut chars = 0;
    let mut words = 0;
    let mut inword = false;
    for &b in &input {
        chars += 1;
        if b == b' ' || b == b'\n' || b == b'\t' || b == b'\r' {
            inword = false;
        } else if !inword {
            words += 1;
            inword = true;
        }
    }
    println!("{} {}", words, chars);
}
