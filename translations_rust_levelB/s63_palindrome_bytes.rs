use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let line = input.lines().next().unwrap_or("");
    let buf = line.as_bytes();
    let len = buf.len();
    let mut ok = true;
    for i in 0..len / 2 {
        if buf[i] != buf[len - 1 - i] {
            ok = false;
            break;
        }
    }
    println!("{}", if ok { "EVET" } else { "HAYIR" });
}
