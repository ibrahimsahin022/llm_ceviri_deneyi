use std::io::{self, Read};

fn my_strlcpy(dst: &mut [u8], src: &[u8], dsize: usize) -> usize {
    let src_len = src.iter().position(|&b| b == 0).unwrap_or(src.len());
    if dsize > 0 {
        let copy_len = src_len.min(dsize - 1);
        dst[..copy_len].copy_from_slice(&src[..copy_len]);
        dst[copy_len] = 0;
    }
    src_len
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut b = [0u8; 1];
    let mut c: u8;

    loop {
        if handle.read_exact(&mut b).is_err() {
            return;
        }
        c = b[0];
        if !matches!(c, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c) {
            break;
        }
    }

    let mut neg = false;
    if c == b'-' {
        neg = true;
        if handle.read_exact(&mut b).is_err() {
            return;
        }
        c = b[0];
    } else if c == b'+' {
        if handle.read_exact(&mut b).is_err() {
            return;
        }
        c = b[0];
    }

    if !c.is_ascii_digit() {
        return;
    }

    let mut dsize: i32 = 0;
    while c.is_ascii_digit() {
        dsize = dsize.wrapping_mul(10).wrapping_add((c - b'0') as i32);
        if handle.read_exact(&mut b).is_err() {
            c = 0;
            break;
        }
        c = b[0];
    }

    if neg {
        dsize = dsize.wrapping_neg();
    }

    if c == 0 {
        return;
    }

    let mut src = [0u8; 512];
    let mut src_len = 0;

    while src_len < 511 {
        if handle.read_exact(&mut b).is_err() {
            break;
        }
        let byte = b[0];
        src[src_len] = byte;
        src_len += 1;
        if byte == b'\n' {
            break;
        }
    }

    if src_len == 0 {
        return;
    }

    let mut len = src_len;
    for i in 0..src_len {
        if src[i] == b'\r' || src[i] == b'\n' {
            len = i;
            break;
        }
    }
    src[len] = 0;
    let src_bytes = &src[..len];

    let mut dst = [0u8; 600];
    let mut effective_dsize = dsize;
    if effective_dsize < 0 {
        effective_dsize = 0;
    }
    if effective_dsize > 600 {
        effective_dsize = 600;
    }

    let ret = my_strlcpy(&mut dst, src_bytes, effective_dsize as usize);

    let dst_str = if effective_dsize > 0 {
        let copy_len = dst.iter().position(|&x| x == 0).unwrap_or(effective_dsize as usize);
        std::str::from_utf8(&dst[..copy_len]).unwrap_or("")
    } else {
        ""
    };

    println!("dst={} ret={}", dst_str, ret);
}
