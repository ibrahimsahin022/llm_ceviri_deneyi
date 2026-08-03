use std::io::{self, Read};

const BASE: u64 = 65521;
const NMAX: usize = 5552;

fn adler32_z(mut adler: u64, buf: &[u8]) -> u64 {
    let mut sum2: u64 = (adler >> 16) & 0xffff;
    adler &= 0xffff;

    let mut len = buf.len();
    let mut pos = 0;

    if len == 1 {
        adler += buf[0] as u64;
        if adler >= BASE {
            adler -= BASE;
        }
        sum2 += adler;
        if sum2 >= BASE {
            sum2 -= BASE;
        }
        return adler | (sum2 << 16);
    }

    if len < 16 {
        while len > 0 {
            len -= 1;
            adler += buf[pos] as u64;
            pos += 1;
            sum2 += adler;
        }
        if adler >= BASE {
            adler -= BASE;
        }
        sum2 %= BASE;
        return adler | (sum2 << 16);
    }

    while len >= NMAX {
        len -= NMAX;
        let mut n = NMAX / 16;
        while n > 0 {
            for _ in 0..16 {
                adler += buf[pos] as u64;
                pos += 1;
                sum2 += adler;
            }
            n -= 1;
        }
        adler %= BASE;
        sum2 %= BASE;
    }

    if len > 0 {
        while len >= 16 {
            len -= 16;
            for _ in 0..16 {
                adler += buf[pos] as u64;
                pos += 1;
                sum2 += adler;
            }
        }
        while len > 0 {
            len -= 1;
            adler += buf[pos] as u64;
            pos += 1;
            sum2 += adler;
        }
        adler %= BASE;
        sum2 %= BASE;
    }

    adler | (sum2 << 16)
}

fn main() {
    let mut buffer = [0u8; 4096];
    let mut bytes_read = 0;

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut byte = [0u8; 1];
    while bytes_read < 4095 {
        match handle.read(&mut byte) {
            Ok(1) => {
                buffer[bytes_read] = byte[0];
                bytes_read += 1;
                if byte[0] == b'\n' {
                    break;
                }
            }
            _ => break,
        }
    }

    if bytes_read == 0 {
        return;
    }

    let line = &buffer[..bytes_read];
    let len = line
        .iter()
        .position(|&b| b == b'\r' || b == b'\n')
        .unwrap_or(line.len());

    let result = adler32_z(1, &line[..len]);
    println!("{}", result);
}
