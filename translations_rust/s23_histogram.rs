use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let line = input.lines().next().unwrap_or("");
    let mut freq = [0i32; 26];
    for c in line.chars() {
        if c >= 'a' && c <= 'z' {
            freq[(c as usize) - ('a' as usize)] += 1;
        }
    }
    let mut out = String::new();
    for i in 0..26 {
        if freq[i] > 0 {
            out.push_str(&format!("{}:{}\n", (b'a' + i as u8) as char, freq[i]));
        }
    }
    print!("{}", out);
}
