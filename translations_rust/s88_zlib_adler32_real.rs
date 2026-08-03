use std::io::{self, BufRead};

const BASE: u32 = 65521;
const NMAX: usize = 5552;

fn adler32_z(adler_in: u32, buf: &[u8]) -> u32 {
    let mut adler = adler_in & 0xffff;
    let mut sum2 = (adler_in >> 16) & 0xffff;
    let mut len = buf.len();
    let mut idx = 0usize;

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
        while len > 0 {
            adler += buf[idx] as u32;
            sum2 += adler;
            idx += 1;
            len -= 1;
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
                adler += buf[idx] as u32;
                sum2 += adler;
                idx += 1;
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
                adler += buf[idx] as u32;
                sum2 += adler;
                idx += 1;
            }
        }
        while len > 0 {
            adler += buf[idx] as u32;
            sum2 += adler;
            idx += 1;
            len -= 1;
        }
        adler %= BASE;
        sum2 %= BASE;
    }

    adler | (sum2 << 16)
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let result = adler32_z(1, line.as_bytes());
    println!("{}", result);
}
