use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut line = Vec::new();
    if handle.read_until(b'\n', &mut line).unwrap_or(0) == 0 {
        return;
    }
    let mut h: u32 = 0;
    for &b in line.iter().take(4095) {
        if b == b'\n' || b == b'\r' {
            break;
        }
        h = h.wrapping_mul(31).wrapping_add(b as u32);
    }
    println!("{}", h);
}
