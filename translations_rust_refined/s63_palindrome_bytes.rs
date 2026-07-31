use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let line = input.lines().next().unwrap_or("");
    let bytes = line.as_bytes();
    let n = bytes.len();
    let mut ok = true;
    for i in 0..n / 2 {
        if bytes[i] != bytes[n - 1 - i] {
            ok = false;
            break;
        }
    }
    println!("{}", if ok { "EVET" } else { "HAYIR" });
}
