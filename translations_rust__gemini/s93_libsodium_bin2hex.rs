use std::io::{self, Read};

fn sodium_bin2hex(
    hex: &mut [u8],
    hex_maxlen: usize,
    bin: &[u8],
    bin_len: usize,
) -> Option<()> {
    if hex_maxlen <= bin_len.wrapping_mul(2) {
        return None;
    }
    let mut i = 0usize;
    while i < bin_len {
        let c = (bin[i] & 0xf) as u32;
        let b = (bin[i] >> 4) as u32;
        let val_c = (87u32
            .wrapping_add(c)
            .wrapping_add(((c.wrapping_sub(10)) >> 8) & !38u32)) as u8;
        let val_b = (87u32
            .wrapping_add(b)
            .wrapping_add(((b.wrapping_sub(10)) >> 8) & !38u32)) as u8;
        let mut x = ((val_c as u32) << 8) | (val_b as u32);
        hex[i * 2] = x as u8;
        x >>= 8;
        hex[i * 2 + 1] = x as u8;
        i += 1;
    }
    hex[i * 2] = 0;

    Some(())
}

fn main() {
    let mut line = [0u8; 512];
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut len = 0;
    while len < 511 {
        let mut b = [0u8; 1];
        match handle.read(&mut b) {
            Ok(1) => {
                line[len] = b[0];
                len += 1;
                if b[0] == b'\n' {
                    break;
                }
            }
            _ => break,
        }
    }

    if len == 0 {
        return;
    }

    let mut blen = len;
    for i in 0..len {
        if line[i] == b'\r' || line[i] == b'\n' {
            blen = i;
            break;
        }
    }

    let mut hex = [0u8; 1024];
    if sodium_bin2hex(&mut hex, 1024, &line, blen).is_some() {
        if let Ok(s) = std::str::from_utf8(&hex[..blen * 2]) {
            println!("{}", s);
        }
    }
}
