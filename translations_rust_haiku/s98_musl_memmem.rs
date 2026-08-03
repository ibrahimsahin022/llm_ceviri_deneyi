use std::io::{self, BufRead};

fn twobyte_memmem(h: &[u8], n: &[u8]) -> Option<usize> {
    if h.len() < 2 {
        return None;
    }
    let nw = ((n[0] as u16) << 8) | (n[1] as u16);
    let mut hw = ((h[0] as u16) << 8) | (h[1] as u16);

    if hw == nw {
        return Some(0);
    }

    for i in 2..h.len() {
        hw = ((hw << 8) | (h[i] as u16)) & 0xFFFF;
        if hw == nw {
            return Some(i - 1);
        }
    }
    None
}

fn threebyte_memmem(h: &[u8], n: &[u8]) -> Option<usize> {
    if h.len() < 3 {
        return None;
    }
    let nw = ((n[0] as u32) << 24) | ((n[1] as u32) << 16) | ((n[2] as u32) << 8);
    let mut hw = ((h[0] as u32) << 24) | ((h[1] as u32) << 16) | ((h[2] as u32) << 8);

    if hw == nw {
        return Some(0);
    }

    for i in 3..h.len() {
        hw = (((hw | (h[i] as u32)) << 8)) & 0xFFFFFF00;
        if hw == nw {
            return Some(i - 2);
        }
    }
    None
}

fn fourbyte_memmem(h: &[u8], n: &[u8]) -> Option<usize> {
    if h.len() < 4 {
        return None;
    }
    let nw = ((n[0] as u32) << 24) | ((n[1] as u32) << 16) | ((n[2] as u32) << 8) | (n[3] as u32);
    let mut hw = ((h[0] as u32) << 24) | ((h[1] as u32) << 16) | ((h[2] as u32) << 8) | (h[3] as u32);

    if hw == nw {
        return Some(0);
    }

    for i in 4..h.len() {
        hw = ((hw << 8) | (h[i] as u32)) & 0xFFFFFFFF;
        if hw == nw {
            return Some(i - 3);
        }
    }
    None
}

fn max(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

fn bitop_get(byteset: &[usize], b: u8) -> bool {
    let idx = (b as usize) / (8 * std::mem::size_of::<usize>());
    let shift = (b as usize) % (8 * std::mem::size_of::<usize>());
    idx < byteset.len() && (byteset[idx] & (1usize << shift)) != 0
}

fn bitop_set(byteset: &mut [usize], b: u8) {
    let idx = (b as usize) / (8 * std::mem::size_of::<usize>());
    let shift = (b as usize) % (8 * std::mem::size_of::<usize>());
    if idx < byteset.len() {
        byteset[idx] |= 1usize << shift;
    }
}

fn twoway_memmem(h: &[u8], n: &[u8]) -> Option<usize> {
    let l = n.len();
    let mut byteset = vec![0usize; 32 / std::mem::size_of::<usize>()];
    let mut shift = [0usize; 256];

    for i in 0..l {
        bitop_set(&mut byteset, n[i]);
        shift[n[i] as usize] = i + 1;
    }

    let mut ip = -1isize;
    let mut jp = 0isize;
    let mut k = 1isize;
    let mut p = 1isize;

    while jp + k < l as isize {
        let n_ip_k = n[(ip + k) as usize];
        let n_jp_k = n[(jp + k) as usize];
        if n_ip_k == n_jp_k {
            if k == p {
                jp += p;
                k = 1;
            } else {
                k += 1;
            }
        } else if n_ip_k > n_jp_k {
            jp += k;
            k = 1;
            p = jp - ip;
        } else {
            ip = jp;
            jp += 1;
            k = 1;
            p = 1;
        }
    }
    let ms = ip;
    let p0 = p;

    ip = -1;
    jp = 0;
    k = 1;
    p = 1;

    while jp + k < l as isize {
        let n_ip_k = n[(ip + k) as usize];
        let n_jp_k = n[(jp + k) as usize];
        if n_ip_k == n_jp_k {
            if k == p {
                jp += p;
                k = 1;
            } else {
                k += 1;
            }
        } else if n_ip_k < n_jp_k {
            jp += k;
            k = 1;
            p = jp - ip;
        } else {
            ip = jp;
            jp += 1;
            k = 1;
            p = 1;
        }
    }

    if ip + 1 > ms + 1 {
        let ms_tmp = ip;
        let p = p0;
    } else {
        let ms_tmp = ms;
        let p = p;
    }

    let (mem0, p_use) = if n[..(ms.max(0) as usize + 1)] != n[(p as usize)..(p as usize + ms.max(0) as usize + 1)] {
        (0, max((ms.max(0) as usize + 1), l - (ms.max(0) as usize) - 1) + 1)
    } else {
        (l - p as usize, p as usize)
    };

    let mut mem = 0usize;
    let mut h_idx = 0usize;

    loop {
        if h.len() - h_idx < l {
            return None;
        }

        if bitop_get(&byteset, h[h_idx + l - 1]) {
            let k_val = l - shift[h[h_idx + l - 1] as usize];
            if k_val > 0 {
                let k_adj = if k_val < mem { mem } else { k_val };
                h_idx += k_adj;
                mem = 0;
                continue;
            }
        } else {
            h_idx += l;
            mem = 0;
            continue;
        }

        let mut k_exit = l;
        for k_check in max(ms.max(0) as usize + 1, mem)..l {
            if n[k_check] != h[h_idx + k_check] {
                k_exit = k_check;
                break;
            }
        }

        if k_exit < l {
            h_idx += k_exit - ms.max(0) as usize;
            mem = 0;
            continue;
        }

        let mut k_back = ms.max(0) as usize + 1;
        for k_check in (1..=(ms.max(0) as usize + 1)).rev() {
            if k_check <= mem {
                k_back = k_check;
                break;
            }
            if n[k_check - 1] != h[h_idx + k_check - 1] {
                k_back = k_check;
                break;
            }
        }

        if k_back <= mem {
            return Some(h_idx);
        }

        h_idx += p_use;
        mem = mem0;
    }
}

fn my_memmem(h: &[u8], n: &[u8]) -> Option<usize> {
    if n.is_empty() {
        return Some(0);
    }
    if h.len() < n.len() {
        return None;
    }

    let pos = h.iter().position(|&b| b == n[0])?;

    if n.len() == 1 {
        return Some(pos);
    }

    let h = &h[pos..];
    if h.len() < n.len() {
        return None;
    }

    if n.len() == 2 {
        return twobyte_memmem(h, n).map(|p| pos + p);
    }
    if n.len() == 3 {
        return threebyte_memmem(h, n).map(|p| pos + p);
    }
    if n.len() == 4 {
        return fourbyte_memmem(h, n).map(|p| pos + p);
    }

    twoway_memmem(h, n).map(|p| pos + p)
}

fn main() {
    let stdin = io::stdin();
    let mut hay = String::new();
    let mut needle = String::new();

    if stdin.read_line(&mut hay).is_err() {
        return;
    }
    if stdin.read_line(&mut needle).is_err() {
        return;
    }

    hay = hay.trim_end().to_string();
    needle = needle.trim_end().to_string();

    match my_memmem(hay.as_bytes(), needle.as_bytes()) {
        Some(idx) => println!("{}", idx),
        None => println!("-1"),
    }
}
