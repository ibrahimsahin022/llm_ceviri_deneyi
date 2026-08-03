use std::io::{self, BufRead};

const BASE: u32 = 65521;
const NMAX: usize = 5552;

fn adler32_z(adler: u32, buf: &[u8]) -> u32 {
    let mut sum2 = (adler >> 16) & 0xffff;
    let mut adler = adler & 0xffff;
    let mut len = buf.len();

    if len == 1 {
        adler += buf[0] as u32;
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
        for &byte in &buf[..len] {
            adler += byte as u32;
            sum2 += adler;
        }
        if adler >= BASE {
            adler %= BASE;
        }
        sum2 %= BASE;
        return adler | (sum2 << 16);
    }

    let mut pos = 0;
    while len >= NMAX {
        len -= NMAX;
        let mut n = NMAX / 16;
        loop {
            for _ in 0..16 {
                adler += buf[pos] as u32;
                sum2 += adler;
                pos += 1;
            }
            n -= 1;
            if n == 0 {
                break;
            }
        }
        adler %= BASE;
        sum2 %= BASE;
    }

    if len > 0 {
        while len >= 16 {
            len -= 16;
            for _ in 0..16 {
                adler += buf[pos] as u32;
                sum2 += adler;
                pos += 1;
            }
        }
        while len > 0 {
            len -= 1;
            adler += buf[pos] as u32;
            sum2 += adler;
            pos += 1;
        }
        adler %= BASE;
        sum2 %= BASE;
    }

    adler | (sum2 << 16)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        let result = adler32_z(1, line.as_bytes());
        println!("{}", result);
    }
}
