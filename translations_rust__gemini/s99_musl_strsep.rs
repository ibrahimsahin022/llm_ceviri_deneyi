use std::io::{self, Read, Write};

fn fgets(buf: &mut [u8]) -> bool {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut read_count = 0;
    let max_bytes = buf.len() - 1;
    while read_count < max_bytes {
        let mut byte = [0u8; 1];
        match handle.read(&mut byte) {
            Ok(1) => {
                buf[read_count] = byte[0];
                read_count += 1;
                if byte[0] == b'\n' {
                    break;
                }
            }
            _ => break,
        }
    }
    if read_count == 0 {
        return false;
    }
    buf[read_count] = 0;
    true
}

fn truncate_line(buf: &[u8]) -> &[u8] {
    let pos = buf
        .iter()
        .position(|&b| b == b'\0' || b == b'\r' || b == b'\n')
        .unwrap_or(buf.len());
    &buf[..pos]
}

fn my_strsep<'a>(str_ptr: &mut Option<&'a [u8]>, sep: &[u8]) -> Option<&'a [u8]> {
    let s = (*str_ptr)?;
    if let Some(idx) = s.iter().position(|b| sep.contains(b)) {
        let tok = &s[..idx];
        *str_ptr = Some(&s[idx + 1..]);
        Some(tok)
    } else {
        let tok = s;
        *str_ptr = None;
        Some(tok)
    }
}

fn main() {
    let mut sep_buf = [0u8; 64];
    let mut text_buf = [0u8; 512];

    if !fgets(&mut sep_buf) {
        return;
    }
    if !fgets(&mut text_buf) {
        return;
    }

    let sep = truncate_line(&sep_buf);
    let text = truncate_line(&text_buf);

    let mut p = Some(text);
    let mut stdout = io::stdout().lock();
    while let Some(tok) = my_strsep(&mut p, sep) {
        let _ = stdout.write_all(tok);
        let _ = stdout.write_all(b"\n");
    }
}
