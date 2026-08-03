use std::io::{self, BufRead};

fn init_upper_to_lower() -> [u8; 256] {
    let mut table = [0u8; 256];
    for i in 0..256 {
        if i >= 'A' as usize && i <= 'Z' as usize {
            table[i] = (i as u8).wrapping_add(32);
        } else {
            table[i] = i as u8;
        }
    }
    table
}

fn sqlite3_str_icmp(left: &str, right: &str, table: &[u8; 256]) -> i32 {
    let left_bytes = left.as_bytes();
    let right_bytes = right.as_bytes();
    let mut a_idx = 0;
    let mut b_idx = 0;

    loop {
        let c = if a_idx < left_bytes.len() {
            left_bytes[a_idx] as i32
        } else {
            0
        };
        let x = if b_idx < right_bytes.len() {
            right_bytes[b_idx] as i32
        } else {
            0
        };

        if c == x {
            if c == 0 {
                break;
            }
        } else {
            let c_mapped = table[c as usize] as i32;
            let x_mapped = table[x as usize] as i32;
            let diff = c_mapped - x_mapped;
            if diff != 0 {
                return diff;
            }
        }
        a_idx += 1;
        b_idx += 1;
    }
    0
}

fn sqlite3_stricmp(left: Option<&str>, right: Option<&str>, table: &[u8; 256]) -> i32 {
    match (left, right) {
        (None, None) => 0,
        (None, Some(_)) => -1,
        (Some(_), None) => 1,
        (Some(l), Some(r)) => sqlite3_str_icmp(l, r, table),
    }
}

fn main() {
    let table = init_upper_to_lower();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let left = lines.next().and_then(|r| r.ok());
    let right = lines.next().and_then(|r| r.ok());

    if left.is_none() || right.is_none() {
        return;
    }

    let result = sqlite3_stricmp(left.as_deref(), right.as_deref(), &table);
    println!("{}", result);
}
