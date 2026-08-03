use std::io::{self, BufRead};

fn my_strlcpy(dst: &mut [u8], src: &[u8]) -> usize {
    let src_len = if let Some(pos) = src.iter().position(|&b| b == 0) {
        pos
    } else {
        src.len()
    };

    let copy_len = if dst.len() > 0 {
        std::cmp::min(src_len, dst.len() - 1)
    } else {
        0
    };

    if copy_len > 0 {
        dst[..copy_len].copy_from_slice(&src[..copy_len]);
    }

    if dst.len() > 0 {
        dst[copy_len] = 0;
    }

    src_len
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();

    // Read dsize
    stdin.read_line(&mut line).unwrap_or(0);
    let mut dsize: i32 = line.trim().parse().unwrap_or(0);

    // Read src
    line.clear();
    stdin.read_line(&mut line).unwrap_or(0);
    let src = line.trim_end().to_string();

    // Apply constraints
    if dsize < 0 {
        dsize = 0;
    }
    if dsize > 600 {
        dsize = 600;
    }

    let mut dst = vec![0u8; dsize as usize];
    let ret = my_strlcpy(&mut dst, src.as_bytes());

    let dst_str = if dsize > 0 {
        if let Ok(s) = std::str::from_utf8(&dst[..dst.len().saturating_sub(1)]) {
            s.trim_end_matches('\0')
        } else {
            ""
        }
    } else {
        ""
    };

    println!("dst={} ret={}", dst_str, ret);
}
