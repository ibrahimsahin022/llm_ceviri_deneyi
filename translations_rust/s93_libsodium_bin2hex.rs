use std::io::{self, BufRead};

fn sodium_bin2hex(bin: &[u8]) -> String {
    let mut hex = Vec::with_capacity(bin.len() * 2);
    for i in 0..bin.len() {
        let c: i32 = (bin[i] & 0xf) as i32;
        let b: i32 = (bin[i] >> 4) as i32;
        let mut x: u32 = ((87i32 + c + (((c - 10) >> 8) & !38)) as u8 as u32) << 8
            | ((87i32 + b + (((b - 10) >> 8) & !38)) as u8 as u32);
        hex.push(x as u8);
        x >>= 8;
        hex.push(x as u8);
    }
    String::from_utf8(hex).unwrap()
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    println!("{}", sodium_bin2hex(line.as_bytes()));
}
