use std::io;

fn main() {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return;
    }

    let buf = input.trim();
    let mut v: u32 = 0;

    for c in buf.chars() {
        if c < '0' || c > '9' {
            break;
        }
        let digit = (c as u32) - ('0' as u32);
        v = v.wrapping_mul(10).wrapping_add(digit);
    }

    println!("{}", v);
}
