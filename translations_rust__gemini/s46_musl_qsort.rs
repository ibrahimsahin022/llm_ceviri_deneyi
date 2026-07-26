type CmpFun = unsafe extern "C" fn(
    *const std::ffi::c_void,
    *const std::ffi::c_void,
    *mut std::ffi::c_void,
) -> i32;

fn ntz_portable(x: usize) -> i32 {
    if x == 0 {
        (8 * std::mem::size_of::<usize>()) as i32
    } else {
        x.trailing_zeros() as i32
    }
}

fn pntz(p: &[usize; 2]) -> i32 {
    if p[0] != 1 {
        ntz_portable(p[0].wrapping_sub(1))
    } else if p[1] != 0 {
        (8 * std::mem::size_of::<usize>()) as i32 + ntz_portable(p[1])
    } else {
        0
    }
}

unsafe fn cycle(width: usize, ar: *mut *mut u8, n: i32) {
    let mut tmp = [0u8; 256];
    let mut l: usize;

    if n < 2 {
        return;
    }

    *ar.add(n as usize) = tmp.as_mut_ptr();
    let mut width = width;
    while width > 0 {
        l = if 256 < width { 256 } else { width };
        std::ptr::copy_nonoverlapping(*ar.add(0), *ar.add(n as usize), l);
        for i in 0..n as usize {
            std::ptr::copy_nonoverlapping(*ar.add(i + 1), *ar.add(i), l);
            *ar.add(i) = (*ar.add(i)).add(l);
        }
        width -= l;
    }
}

fn shl(p: &mut [usize; 2], mut n: i32) {
    let bits = (8 * std::mem::size_of::<usize>()) as i32;
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

fn shr(p: &mut [usize; 2], mut n: i32) {
    let bits = (8 * std::mem::size_of::<usize>()) as i32;
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

const AR_LEN: usize = 16 * std::mem::size_of::<usize>();
const AR_MASK: usize = AR_LEN - 1;

unsafe fn sift(
    mut head: *mut u8,
    width: usize,
    cmp: CmpFun,
    arg: *mut std::ffi::c_void,
    mut pshift: i32,
    lp: *const usize,
) {
    let mut rt: *mut u8;
    let mut lf: *mut u8;
    let mut ar = [*mut u8::null(); AR_LEN];
    let mut i: i32 = 1;

    ar[0] = head;
    while pshift > 1 {
        rt = head.sub(width);
        lf = head.sub(width).sub(*lp.add((pshift - 2) as usize));

        if cmp(ar[0] as *const _, lf as *const _, arg) >= 0
            && cmp(ar[0] as *const _, rt as *const _, arg) >= 0
        {
            break;
        }
        if cmp(lf as *const _, rt as *const _, arg) >= 0 {
            ar[(i as usize) & AR_MASK] = lf;
            i += 1;
            head = lf;
            pshift -= 1;
        } else {
            ar[(i as usize) & AR_MASK] = rt;
            i += 1;
            head = rt;
            pshift -= 2;
        }
    }
    cycle(width, ar.as_mut_ptr(), i & (AR_MASK as i32));
}

unsafe fn trinkle(
    mut head: *mut u8,
    width: usize,
    cmp: CmpFun,
    arg: *mut std::ffi::c_void,
    pp: &[usize; 2],
    mut pshift: i32,
    mut trusty: i32,
    lp: *const usize,
) {
    let mut stepson: *mut u8;
    let mut rt: *mut u8;
    let mut lf: *mut u8;
    let mut p = [pp[0], pp[1]];
    let mut ar = [*mut u8::null(); AR_LEN];
    let mut i: i32 = 1;
    let mut trail: i32;

    ar[0] = head;
    while p[0] != 1 || p[1] != 0 {
        stepson = head.sub(*lp.add(pshift as usize));
        if cmp(stepson as *const _, ar[0] as *const _, arg) <= 0 {
            break;
        }
        if trusty == 0 && pshift > 1 {
            rt = head.sub(width);
            lf = head.sub(width).sub(*lp.add((pshift - 2) as usize));
            if cmp(rt as *const _, stepson as *const _, arg) >= 0
                || cmp(lf as *const _, stepson as *const _, arg) >= 0
            {
                break;
            }
        }

        ar[(i as usize) & AR_MASK] = stepson;
        i += 1;
        head = stepson;
        trail = pntz(&p);
        shr(&mut p, trail);
        pshift += trail;
        trusty = 0;
    }
    if trusty == 0 {
        cycle(width, ar.as_mut_ptr(), i & (AR_MASK as i32));
        sift(head, width, cmp, arg, pshift, lp);
    }
}

pub unsafe fn __qsort_r(
    base: *mut std::ffi::c_void,
    nel: usize,
    width: usize,
    cmp: CmpFun,
    arg: *mut std::ffi::c_void,
) {
    let mut lp = [0usize; 12 * std::mem::size_of::<usize>()];
    let size = width * nel;
    if size == 0 {
        return;
    }

    let mut head = base as *mut u8;
    let high = head.add(size - width);

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

    let mut p = [1usize, 0usize];
    let mut pshift = 1i32;
    let mut trail: i32;

    while head < high {
        if (p[0] & 3) == 3 {
            sift(head, width, cmp, arg, pshift, lp.as_ptr());
            shr(&mut p, 2);
            pshift += 2;
        } else {
            if lp[(pshift - 1) as usize] >= (high as usize - head as usize) {
                trinkle(head, width, cmp, arg, &p, pshift, 0, lp.as_ptr());
            } else {
                sift(head, width, cmp, arg, pshift, lp.as_ptr());
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

    trinkle(head, width, cmp, arg, &p, pshift, 0, lp.as_ptr());

    while pshift != 1 || p[0] != 1 || p[1] != 0 {
        if pshift <= 1 {
            trail = pntz(&p);
            shr(&mut p, trail);
            pshift += trail;
        } else {
            shl(&mut p, 2);
            pshift -= 2;
            p[0] ^= 7;
            shr(&mut p, 1);
            trinkle(
                head.sub(lp[pshift as usize]).sub(width),
                width,
                cmp,
                arg,
                &p,
                pshift + 1,
                1,
                lp.as_ptr(),
            );
            shl(&mut p, 1);
            p[0] |= 1;
            trinkle(
                head.sub(width),
                width,
                cmp,
                arg,
                &p,
                pshift,
                1,
                lp.as_ptr(),
            );
        }
        head = head.sub(width);
    }
}

unsafe extern "C" fn cmp_int(
    a: *const std::ffi::c_void,
    b: *const std::ffi::c_void,
    _arg: *mut std::ffi::c_void,
) -> i32 {
    let x = *(a as *const i32);
    let y = *(b as *const i32);
    (x > y) as i32 - (x < y) as i32
}

fn main() {
    use std::io::Read;

    let mut input = String::new();
    if std::io::stdin().read_to_string(&mut input).is_err() {
        std::process::exit(1);
    }

    let mut tokens = input.split_whitespace();

    let n: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) if val >= 0 => val,
        _ => std::process::exit(1),
    };

    let mut arr = vec![0i32; n as usize];
    for i in 0..n as usize {
        match tokens.next().and_then(|s| s.parse().ok()) {
            Some(val) => arr[i] = val,
            None => std::process::exit(1),
        }
    }

    unsafe {
        __qsort_r(
            arr.as_mut_ptr() as *mut std::ffi::c_void,
            n as usize,
            std::mem::size_of::<i32>(),
            cmp_int,
            std::ptr::null_mut(),
        );
    }

    for i in 0..n as usize {
        print!("{}", arr[i]);
        if i + 1 < n as usize {
            print!(" ");
        }
    }
    println!();
}
