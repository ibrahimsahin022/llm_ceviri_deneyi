use std::io::{self, Read};

fn ull2string(dst: &mut [u8], dstlen: usize, value: u64) -> usize {
    let mut tmp = [0u8; 20];
    let mut len = 0;
    let mut val = value;
    if val == 0 {
        tmp[len] = b'0';
        len += 1;
    } else {
        while val > 0 {
            tmp[len] = b'0' + (val % 10) as u8;
            len += 1;
            val /= 10;
        }
    }
    if dstlen < len {
        return 0;
    }
    for i in 0..len {
        dst[i] = tmp[len - 1 - i];
    }
    len
}

fn ll2string(dst: &mut [u8], dstlen: usize, svalue: i64) -> usize {
    let value: u64;
    let mut negative = 0;
    let mut offset = 0;
    let mut remaining_len = dstlen;

    if svalue < 0 {
        if svalue != i64::MIN {
            value = (-svalue) as u64;
        } else {
            value = (i64::MAX as u64) + 1;
        }
        if remaining_len < 2 {
            return 0;
        }
        negative = 1;
        dst[offset] = b'-';
        offset += 1;
        remaining_len -= 1;
    } else {
        value = svalue as u64;
    }

    let length = ull2string(&mut dst[offset..], remaining_len, value);
    if length == 0 {
        return 0;
    }
    length + negative
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let token = match input.split_whitespace().next() {
        Some(t) => t,
        None => return,
    };
    let v: i64 = match token.parse() {
        Ok(val) => val,
        Err(_) => return,
    };

    let mut buf = [0u8; 32];
    let n = ll2string(&mut buf, 32, v);
    buf[n] = 0;

    if let Ok(s) = std::str::from_utf8(&buf[..n]) {
        println!("{}", s);
    }
}
