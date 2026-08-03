use std::io::{self, BufRead};

fn build_upper_to_lower() -> [u8; 256] {
    let mut t = [0u8; 256];
    for i in 0..256usize {
        if i >= 'A' as usize && i <= 'Z' as usize {
            t[i] = (i + 32) as u8;
        } else {
            t[i] = i as u8;
        }
    }
    t
}

fn sqlite3_str_icmp(left: &[u8], right: &[u8], table: &[u8; 256]) -> i32 {
    let mut ai = 0usize;
    let mut bi = 0usize;
    let mut c: i32;
    loop {
        let cc = if ai < left.len() { left[ai] } else { 0 };
        let xx = if bi < right.len() { right[bi] } else { 0 };
        c = cc as i32;
        let x = xx as i32;
        if c == x {
            if c == 0 {
                break;
            }
        } else {
            c = table[cc as usize] as i32 - table[xx as usize] as i32;
            if c != 0 {
                break;
            }
        }
        ai += 1;
        bi += 1;
    }
    c
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let left = lines.next().unwrap().unwrap();
    let right = lines.next().unwrap().unwrap();
    let table = build_upper_to_lower();
    let result = sqlite3_str_icmp(left.as_bytes(), right.as_bytes(), &table);
    println!("{}", result);
}
