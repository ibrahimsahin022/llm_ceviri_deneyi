use std::io::{self, BufRead};

static UPPER_TO_LOWER: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0;
    while i < 256 {
        if i >= b'A' as usize && i <= b'Z' as usize {
            table[i] = (i + 32) as u8;
        } else {
            table[i] = i as u8;
        }
        i += 1;
    }
    table
};

fn sqlite3_str_icmp(z_left: &[u8], z_right: &[u8]) -> i32 {
    let mut i = 0;
    loop {
        let c_byte = *z_left.get(i).unwrap_or(&0);
        let x_byte = *z_right.get(i).unwrap_or(&0);
        let mut c = c_byte as i32;
        let x = x_byte as i32;
        if c == x {
            if c == 0 {
                return 0;
            }
        } else {
            c = UPPER_TO_LOWER[c_byte as usize] as i32 - UPPER_TO_LOWER[x_byte as usize] as i32;
            if c != 0 {
                return c;
            }
        }
        i += 1;
    }
}

fn sqlite3_stricmp(z_left: Option<&[u8]>, z_right: Option<&[u8]>) -> i32 {
    match (z_left, z_right) {
        (None, Some(_)) => -1,
        (None, None) => 0,
        (Some(_), None) => 1,
        (Some(left), Some(right)) => sqlite3_str_icmp(left, right),
    }
}

fn fgets_256<R: BufRead>(reader: &mut R) -> Option<Vec<u8>> {
    let mut buf = Vec::new();
    while buf.len() < 255 {
        let available = match reader.fill_buf() {
            Ok(n) if n.is_empty() => break,
            Ok(n) => n,
            Err(_) => return None,
        };
        let mut consumed = 0;
        let mut stop = false;
        for &b in available {
            if buf.len() >= 255 {
                stop = true;
                break;
            }
            buf.push(b);
            consumed += 1;
            if b == b'\n' {
                stop = true;
                break;
            }
        }
        reader.consume(consumed);
        if stop {
            break;
        }
    }
    if buf.is_empty() {
        None
    } else {
        Some(buf)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let left = match fgets_256(&mut handle) {
        Some(l) => l,
        None => return,
    };
    let right = match fgets_256(&mut handle) {
        Some(r) => r,
        None => return,
    };

    let left_pos = left.iter().position(|&b| b == b'\r' || b == b'\n' || b == b'\0').unwrap_or(left.len());
    let right_pos = right.iter().position(|&b| b == b'\r' || b == b'\n' || b == b'\0').unwrap_or(right.len());

    let left_slice = &left[..left_pos];
    let right_slice = &right[..right_pos];

    println!("{}", sqlite3_stricmp(Some(left_slice), Some(right_slice)));
}
