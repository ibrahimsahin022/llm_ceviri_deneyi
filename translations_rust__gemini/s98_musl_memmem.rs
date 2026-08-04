use std::io::{self, BufRead};

fn twobyte_memmem(h: &[u8], mut k: usize, n: &[u8]) -> Option<usize> {
    let nw = ((n[0] as u16) << 8) | (n[1] as u16);
    let mut hw = ((h[0] as u16) << 8) | (h[1] as u16);
    let mut idx = 2;
    k -= 2;
    while k > 0 {
        if hw == nw {
            return Some(idx - 2);
        }
        hw = (hw.wrapping_shl(8)) | (h[idx] as u16);
        idx += 1;
        k -= 1;
    }
    if hw == nw {
        Some(idx - 2)
    } else {
        None
    }
}

fn threebyte_memmem(h: &[u8], mut k: usize, n: &[u8]) -> Option<usize> {
    let nw = ((n[0] as u32) << 24) | ((n[1] as u32) << 16) | ((n[2] as u32) << 8);
    let mut hw = ((h[0] as u32) << 24) | ((h[1] as u32) << 16) | ((h[2] as u32) << 8);
    let mut idx = 3;
    k -= 3;
    while k > 0 {
        if hw == nw {
            return Some(idx - 3);
        }
        hw = (hw | (h[idx] as u32)).wrapping_shl(8);
        idx += 1;
        k -= 1;
    }
    if hw == nw {
        Some(idx - 3)
    } else {
        None
    }
}

fn fourbyte_memmem(h: &[u8], mut k: usize, n: &[u8]) -> Option<usize> {
    let nw = ((n[0] as u32) << 24) | ((n[1] as u32) << 16) | ((n[2] as u32) << 8) | (n[3] as u32);
    let mut hw = ((h[0] as u32) << 24) | ((h[1] as u32) << 16) | ((h[2] as u32) << 8) | (h[3] as u32);
    let mut idx = 4;
    k -= 4;
    while k > 0 {
        if hw == nw {
            return Some(idx - 4);
        }
        hw = (hw.wrapping_shl(8)) | (h[idx] as u32);
        idx += 1;
        k -= 1;
    }
    if hw == nw {
        Some(idx - 4)
    } else {
        None
    }
}

fn twoway_memmem(haystack: &[u8], n: &[u8]) -> Option<usize> {
    let l = n.len();
    let mut byteset = [0usize; 32 / std::mem::size_of::<usize>()];
    let mut shift = [0usize; 256];

    for i in 0..l {
        let byte = n[i] as usize;
        let idx = byte / (8 * std::mem::size_of::<usize>());
        let bit = byte % (8 * std::mem::size_of::<usize>());
        byteset[idx] |= 1usize << bit;
        shift[byte] = i + 1;
    }

    let bit_test = |b: u8| -> bool {
        let byte = b as usize;
        let idx = byte / (8 * std::mem::size_of::<usize>());
        let bit = byte % (8 * std::mem::size_of::<usize>());
        (byteset[idx] & (1usize << bit)) != 0
    };

    let mut ip: usize = usize::MAX;
    let mut jp: usize = 0;
    let mut k: usize = 1;
    let mut p: usize = 1;

    while jp.wrapping_add(k) < l {
        let n_ip_k = n[ip.wrapping_add(k)];
        let n_jp_k = n[jp.wrapping_add(k)];
        if n_ip_k == n_jp_k {
            if k == p {
                jp = jp.wrapping_add(p);
                k = 1;
            } else {
                k += 1;
            }
        } else if n_ip_k > n_jp_k {
            jp = jp.wrapping_add(k);
            k = 1;
            p = jp.wrapping_sub(ip);
        } else {
            ip = jp;
            jp += 1;
            k = 1;
            p = 1;
        }
    }

    let mut ms = ip;
    let p0 = p;

    ip = usize::MAX;
    jp = 0;
    k = 1;
    p = 1;

    while jp.wrapping_add(k) < l {
        let n_ip_k = n[ip.wrapping_add(k)];
        let n_jp_k = n[jp.wrapping_add(k)];
        if n_ip_k == n_jp_k {
            if k == p {
                jp = jp.wrapping_add(p);
                k = 1;
            } else {
                k += 1;
            }
        } else if n_ip_k < n_jp_k {
            jp = jp.wrapping_add(k);
            k = 1;
            p = jp.wrapping_sub(ip);
        } else {
            ip = jp;
            jp += 1;
            k = 1;
            p = 1;
        }
    }

    if ip.wrapping_add(1) > ms.wrapping_add(1) {
        ms = ip;
    } else {
        p = p0;
    }

    let ms_plus_1 = ms.wrapping_add(1);

    let mem0 = if p + ms_plus_1 > l || n[..ms_plus_1] != n[p..p + ms_plus_1] {
        p = ms.max(l - ms - 1) + 1;
        0
    } else {
        l - p
    };

    let mut mem = 0;
    let mut h_idx = 0;

    loop {
        if haystack.len() - h_idx < l {
            return None;
        }

        let hl_1 = haystack[h_idx + l - 1];
        if bit_test(hl_1) {
            k = l - shift[hl_1 as usize];
            if k != 0 {
                if k < mem {
                    k = mem;
                }
                h_idx += k;
                mem = 0;
                continue;
            }
        } else {
            h_idx += l;
            mem = 0;
            continue;
        }

        k = ms_plus_1.max(mem);
        while k < l && n[k] == haystack[h_idx + k] {
            k += 1;
        }
        if k < l {
            h_idx += k.wrapping_sub(ms);
            mem = 0;
            continue;
        }

        k = ms_plus_1;
        while k > mem && n[k - 1] == haystack[h_idx + k - 1] {
            k -= 1;
        }
        if k <= mem {
            return Some(h_idx);
        }

        h_idx += p;
        mem = mem0;
    }
}

fn my_memmem(h0: &[u8], n0: &[u8]) -> Option<usize> {
    let k = h0.len();
    let l = n0.len();

    if l == 0 {
        return Some(0);
    }
    if k < l {
        return None;
    }

    let first_byte = n0[0];
    let offset = h0.iter().position(|&b| b == first_byte)?;

    let h = &h0[offset..];
    let k = h.len();

    if l == 1 {
        return Some(offset);
    }
    if k < l {
        return None;
    }
    if l == 2 {
        return twobyte_memmem(h, k, n0).map(|rel| offset + rel);
    }
    if l == 3 {
        return threebyte_memmem(h, k, n0).map(|rel| offset + rel);
    }
    if l == 4 {
        return fourbyte_memmem(h, k, n0).map(|rel| offset + rel);
    }

    twoway_memmem(h, n0).map(|rel| offset + rel)
}

fn read_line_c(handle: &mut impl BufRead) -> Option<Vec<u8>> {
    let mut buf = Vec::new();
    let mut count = 0;
    while count < 511 {
        let available = match handle.fill_buf() {
            Ok(n) if n.is_empty() => break,
            Ok(n) => n,
            Err(_) => return None,
        };
        let mut consumed = 0;
        let mut done = false;
        for &b in available {
            consumed += 1;
            count += 1;
            buf.push(b);
            if b == b'\n' || count == 511 {
                done = true;
                break;
            }
        }
        handle.consume(consumed);
        if done {
            break;
        }
    }
    if buf.is_empty() && count == 0 {
        return None;
    }
    if let Some(pos) = buf.iter().position(|&b| b == b'\r' || b == b'\n') {
        buf.truncate(pos);
    }
    Some(buf)
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let hay = match read_line_c(&mut handle) {
        Some(h) => h,
        None => return,
    };
    let needle = match read_line_c(&mut handle) {
        Some(n) => n,
        None => return,
    };

    match my_memmem(&hay, &needle) {
        Some(idx) => println!("{}", idx),
        None => println!("-1"),
    }
}
