use std::io::{self, BufRead};

static DATA_BIN2ASCII: &[u8; 65] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\0";

#[inline]
fn conv_bin2ascii(a: usize, table: &[u8]) -> u8 {
    table[a & 0x3f]
}

fn evp_encodeblock_int(t: &mut [u8], f: &[u8]) -> i32 {
    let mut ret = 0;
    let table = DATA_BIN2ASCII;
    let dlen = f.len();
    let mut i = dlen;
    let mut f_idx = 0;
    let mut t_idx = 0;

    while i > 0 {
        let l: u64;
        if i >= 3 {
            l = ((f[f_idx] as u64) << 16)
                | ((f[f_idx + 1] as u64) << 8)
                | (f[f_idx + 2] as u64);
            t[t_idx] = conv_bin2ascii((l >> 18) as usize, table);
            t[t_idx + 1] = conv_bin2ascii((l >> 12) as usize, table);
            t[t_idx + 2] = conv_bin2ascii((l >> 6) as usize, table);
            t[t_idx + 3] = conv_bin2ascii(l as usize, table);
            i -= 3;
        } else {
            l = (f[f_idx] as u64) << 16
                | if i == 2 {
                    (f[f_idx + 1] as u64) << 8
                } else {
                    0
                };

            t[t_idx] = conv_bin2ascii((l >> 18) as usize, table);
            t[t_idx + 1] = conv_bin2ascii((l >> 12) as usize, table);
            t[t_idx + 2] = if i == 1 {
                b'='
            } else {
                conv_bin2ascii((l >> 6) as usize, table)
            };
            t[t_idx + 3] = b'=';
            i = 0;
        }
        ret += 4;
        t_idx += 4;
        f_idx += 3;
    }

    t[t_idx] = b'\0';
    ret
}

fn main() {
    let mut line_buf = Vec::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    if handle.read_until(b'\n', &mut line_buf).unwrap_or(0) == 0 {
        return;
    }

    if line_buf.len() > 1023 {
        line_buf.truncate(1023);
    }

    let pos = line_buf
        .iter()
        .position(|&b| b == b'\r' || b == b'\n')
        .unwrap_or(line_buf.len());
    let line = &line_buf[..pos];

    let mut out = [0u8; 2048];
    let len = evp_encodeblock_int(&mut out, line) as usize;

    if let Ok(s) = std::str::from_utf8(&out[..len]) {
        println!("{}", s);
    }
}
