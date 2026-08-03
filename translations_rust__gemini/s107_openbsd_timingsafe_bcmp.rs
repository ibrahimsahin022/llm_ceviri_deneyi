use std::io::Read;

fn my_timingsafe_bcmp(b1: &[u8], b2: &[u8], n: usize) -> i32 {
    let mut ret: i32 = 0;
    for i in 0..n {
        ret |= (b1[i] ^ b2[i]) as i32;
    }
    if ret != 0 { 1 } else { 0 }
}

fn fgets<R: Read>(reader: &mut R, buf: &mut [u8]) -> bool {
    let mut count = 0;
    while count + 1 < buf.len() {
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

fn strcspn_strip(buf: &mut [u8]) {
    for i in 0..buf.len() {
        if buf[i] == 0 || buf[i] == b'\r' || buf[i] == b'\n' {
            buf[i] = 0;
            break;
        }
    }
}

fn c_strlen(buf: &[u8]) -> usize {
    buf.iter().position(|&c| c == 0).unwrap_or(buf.len())
}

fn main() {
    let mut a = [0u8; 256];
    let mut b = [0u8; 256];
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();

    if !fgets(&mut handle, &mut a) {
        return;
    }
    if !fgets(&mut handle, &mut b) {
        return;
    }

    strcspn_strip(&mut a);
    strcspn_strip(&mut b);

    let mut n = c_strlen(&a);
    let len_b = c_strlen(&b);
    if len_b < n {
        n = len_b;
    }

    let r = my_timingsafe_bcmp(&a, &b, n);
    println!("{}", r);
}
