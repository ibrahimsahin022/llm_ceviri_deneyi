use std::io::{self, Read, Write};

type CmpFun = extern "C" fn(*const u8, *const u8, *mut std::ffi::c_void) -> i32;

const AR_LEN: usize = 16 * std::mem::size_of::<usize>();
const AR_MASK: usize = AR_LEN - 1;

fn ntz_portable(x: usize) -> i32 {
    if x == 0 {
        return 8 * std::mem::size_of::<usize>() as i32;
    }
    let mut x = x;
    let mut n = 0;
    while x & 1 == 0 {
        x >>= 1;
        n += 1;
    }
    n
}

fn pntz(p: &[usize; 2]) -> i32 {
    if p[0] != 1 {
        return ntz_portable(p[0] - 1);
    }
    if p[1] != 0 {
        return 8 * std::mem::size_of::<usize>() as i32 + ntz_portable(p[1]);
    }
    0
}

unsafe fn cycle(width: usize, ar: &mut [*mut u8; AR_LEN], n: usize) {
    let mut tmp = [0u8; 256];
    if n < 2 {
        return;
    }
    ar[n] = tmp.as_mut_ptr();
    let mut w = width;
    while w > 0 {
        let l = if 256 < w { 256 } else { w };
        std::ptr::copy_nonoverlapping(ar[0], ar[n], l);
        for i in 0..n {
            std::ptr::copy_nonoverlapping(ar[i + 1], ar[i], l);
            ar[i] = ar[i].add(l);
        }
        w -= l;
    }
}

fn shl(p: &mut [usize; 2], n: i32) {
    let bits = 8 * std::mem::size_of::<usize>() as i32;
    let mut n = n;
    if n >= bits {
        n -= bits;
        p[1] = p[0];
        p[0] = 0;
        if n == 0 {
            return;
        }
    }
    p[1] <<= n;
    p[1] |= p[0] >> (bits - n);
    p[0] <<= n;
}

fn shr(p: &mut [usize; 2], n: i32) {
    let bits = 8 * std::mem::size_of::<usize>() as i32;
    let mut n = n;
    if n >= bits {
        n -= bits;
        p[0] = p[1];
        p[1] = 0;
        if n == 0 {
            return;
        }
    }
    p[0] >>= n;
    p[0] |= p[1] << (bits - n);
    p[1] >>= n;
}

unsafe fn sift(head_in: *mut u8, width: usize, cmp: CmpFun, arg: *mut std::ffi::c_void, pshift_in: i32, lp: &[usize]) {
    let mut ar = [std::ptr::null_mut::<u8>(); AR_LEN];
    let mut i: usize = 1;
    let mut head = head_in;
    let mut pshift = pshift_in;

    ar[0] = head;
    while pshift > 1 {
        let rt = head.sub(width);
        let lf = head.sub(width).sub(lp[(pshift - 2) as usize]);

        if cmp(ar[0], lf, arg) >= 0 && cmp(ar[0], rt, arg) >= 0 {
            break;
        }
        if cmp(lf, rt, arg) >= 0 {
            ar[i & AR_MASK] = lf;
            i += 1;
            head = lf;
            pshift -= 1;
        } else {
            ar[i & AR_MASK] = rt;
            i += 1;
            head = rt;
            pshift -= 2;
        }
    }
    cycle(width, &mut ar, i & AR_MASK);
}

unsafe fn trinkle(
    head_in: *mut u8,
    width: usize,
    cmp: CmpFun,
    arg: *mut std::ffi::c_void,
    pp: &[usize; 2],
    pshift_in: i32,
    trusty_in: i32,
    lp: &[usize],
) {
    let mut p = [pp[0], pp[1]];
    let mut ar = [std::ptr::null_mut::<u8>(); AR_LEN];
    let mut i: usize = 1;
    let mut head = head_in;
    let mut pshift = pshift_in;
    let mut trusty = trusty_in;

    ar[0] = head;
    while p[0] != 1 || p[1] != 0 {
        let stepson = head.sub(lp[pshift as usize]);
        if cmp(stepson, ar[0], arg) <= 0 {
            break;
        }
        if trusty == 0 && pshift > 1 {
            let rt = head.sub(width);
            let lf = head.sub(width).sub(lp[(pshift - 2) as usize]);
            if cmp(rt, stepson, arg) >= 0 || cmp(lf, stepson, arg) >= 0 {
                break;
            }
        }

        ar[i & AR_MASK] = stepson;
        i += 1;
        head = stepson;
        let trail = pntz(&p);
        shr(&mut p, trail);
        pshift += trail;
        trusty = 0;
    }
    if trusty == 0 {
        cycle(width, &mut ar, i & AR_MASK);
        sift(head, width, cmp, arg, pshift, lp);
    }
}

unsafe fn qsort_r(base: *mut u8, nel: usize, width: usize, cmp: CmpFun, arg: *mut std::ffi::c_void) {
    let mut lp = vec![0usize; 12 * std::mem::size_of::<usize>()];
    let size = width * nel;
    if size == 0 {
        return;
    }

    let mut head = base;
    let high = base.add(size - width);
    let mut p = [1usize, 0usize];
    let mut pshift: i32 = 1;

    lp[0] = width;
    lp[1] = width;
    let mut i = 2;
    loop {
        lp[i] = lp[i - 2] + lp[i - 1] + width;
        if lp[i] >= size {
            break;
        }
        i += 1;
    }

    while (head as usize) < (high as usize) {
        if p[0] & 3 == 3 {
            sift(head, width, cmp, arg, pshift, &lp);
            shr(&mut p, 2);
            pshift += 2;
        } else {
            if lp[(pshift - 1) as usize] >= (high as usize - head as usize) {
                trinkle(head, width, cmp, arg, &p, pshift, 0, &lp);
            } else {
                sift(head, width, cmp, arg, pshift, &lp);
            }

            if pshift == 1 {
                shl(&mut p, 1);
                pshift = 0;
            } else {
                shl(&mut p, pshift - 1);
                pshift = 1;
            }
        }

        p[0] |= 1;
        head = head.add(width);
    }

    trinkle(head, width, cmp, arg, &p, pshift, 0, &lp);

    while pshift != 1 || p[0] != 1 || p[1] != 0 {
        if pshift <= 1 {
            let trail = pntz(&p);
            shr(&mut p, trail);
            pshift += trail;
        } else {
            shl(&mut p, 2);
            pshift -= 2;
            p[0] ^= 7;
            shr(&mut p, 1);
            trinkle(head.sub(lp[pshift as usize]).sub(width), width, cmp, arg, &p, pshift + 1, 1, &lp);
            shl(&mut p, 1);
            p[0] |= 1;
            trinkle(head.sub(width), width, cmp, arg, &p, pshift, 1, &lp);
        }
        head = head.sub(width);
    }
}

extern "C" fn cmp_int(a: *const u8, b: *const u8, _arg: *mut std::ffi::c_void) -> i32 {
    unsafe {
        let x = *(a as *const i32);
        let y = *(b as *const i32);
        (x > y) as i32 - (x < y) as i32
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace().map(|s| s.parse::<i32>().unwrap());

    let n = it.next().unwrap_or(0) as usize;
    let mut arr: Vec<i32> = (0..n).map(|_| it.next().unwrap()).collect();

    if n > 0 {
        unsafe {
            qsort_r(
                arr.as_mut_ptr() as *mut u8,
                n,
                std::mem::size_of::<i32>(),
                cmp_int,
                std::ptr::null_mut(),
            );
        }
    }

    let stdout = io::stdout();
    let mut out = stdout.lock();
    let strs: Vec<String> = arr.iter().map(|x| x.to_string()).collect();
    writeln!(out, "{}", strs.join(" ")).unwrap();
}
