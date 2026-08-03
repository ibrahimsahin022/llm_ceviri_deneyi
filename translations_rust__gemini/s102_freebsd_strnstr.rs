use std::io::{self, Read};

unsafe fn my_strnstr(mut s: *const u8, mut find: *const u8, mut slen: usize) -> *const u8 {
    let c = *find;
    find = find.add(1);
    if c != 0 {
        let mut p = find;
        while *p != 0 {
            p = p.add(1);
        }
        let len = p as usize - find as usize;

        loop {
            let mut sc: u8;
            loop {
                if slen == 0 {
                    return std::ptr::null();
                }
                slen -= 1;
                sc = *s;
                s = s.add(1);
                if sc == 0 {
                    return std::ptr::null();
                }
                if sc == c {
                    break;
                }
            }
            if len > slen {
                return std::ptr::null();
            }
            let mut match_ok = true;
            for i in 0..len {
                if *s.add(i) != *find.add(i) {
                    match_ok = false;
                    break;
                }
            }
            if match_ok {
                break;
            }
        }
        s = s.sub(1);
    }
    s
}

fn fgets<R: Read>(reader: &mut R, buf: &mut [u8]) -> bool {
    let max_read = buf.len() - 1;
    let mut count = 0;
    while count < max_read {
        let mut b = [0u8; 1];
        match reader.read(&mut b) {
            Ok(1) => {
                buf[count] = b[0];
                count += 1;
                if b[0] == b'\n' {
                    break;
                }
            }
            _ => break,
        }
    }
    if count == 0 {
        return false;
    }
    buf[count] = 0;
    true
}

fn scanf_d<R: Read>(reader: &mut R) -> Option<i32> {
    let mut byte = [0u8; 1];
    let mut b;
    loop {
        if reader.read(&mut byte).ok()? != 1 {
            return None;
        }
        b = byte[0];
        if !b.is_ascii_whitespace() {
            break;
        }
    }
    let mut neg = false;
    if b == b'-' {
        neg = true;
        if reader.read(&mut byte).ok()? != 1 {
            return None;
        }
        b = byte[0];
    } else if b == b'+' {
        if reader.read(&mut byte).ok()? != 1 {
            return None;
        }
        b = byte[0];
    }

    if !b.is_ascii_digit() {
        return None;
    }

    let mut val: i32 = 0;
    while b.is_ascii_digit() {
        let digit = (b - b'0') as i32;
        val = val.wrapping_mul(10).wrapping_add(digit);
        let res = reader.read(&mut byte);
        if let Ok(1) = res {
            b = byte[0];
        } else {
            break;
        }
    }

    if neg {
        val = val.wrapping_neg();
    }
    Some(val)
}

fn truncate_at_newline(buf: &mut [u8]) {
    for i in 0..buf.len() {
        if buf[i] == 0 || buf[i] == b'\r' || buf[i] == b'\n' {
            buf[i] = 0;
            break;
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();

    let mut s = [0u8; 512];
    let mut find = [0u8; 256];
    let mut slen: i32;

    if !fgets(&mut stdin_lock, &mut s) {
        return;
    }
    if !fgets(&mut stdin_lock, &mut find) {
        return;
    }
    if let Some(val) = scanf_d(&mut stdin_lock) {
        slen = val;
    } else {
        return;
    }

    truncate_at_newline(&mut s);
    truncate_at_newline(&mut find);

    if slen < 0 {
        slen = 0;
    }

    unsafe {
        let r = my_strnstr(s.as_ptr(), find.as_ptr(), slen as usize);
        if !r.is_null() {
            let offset = r.offset_from(s.as_ptr());
            println!("{}", offset);
        } else {
            println!("-1");
        }
    }
}
