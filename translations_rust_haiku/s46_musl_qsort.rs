use std::io::{self, BufRead};

fn ntz(mut x: usize) -> usize {
    if x == 0 {
        return std::mem::size_of::<usize>() * 8;
    }
    let mut n = 0;
    while (x & 1) == 0 {
        x >>= 1;
        n += 1;
    }
    n
}

fn pntz(p: &[usize; 2]) -> usize {
    if p[0] != 1 {
        return ntz(p[0] - 1);
    }
    if p[1] != 0 {
        return 8 * std::mem::size_of::<usize>() + ntz(p[1]);
    }
    0
}

fn shl(p: &mut [usize; 2], mut n: usize) {
    let bits = 8 * std::mem::size_of::<usize>();
    if n >= bits {
        n -= bits;
        p[1] = p[0];
        p[0] = 0;
        if n == 0 {
            return;
        }
    }
    p[1] = (p[1] << n) | (p[0] >> (bits - n));
    p[0] <<= n;
}

fn shr(p: &mut [usize; 2], mut n: usize) {
    let bits = 8 * std::mem::size_of::<usize>();
    if n >= bits {
        n -= bits;
        p[0] = p[1];
        p[1] = 0;
        if n == 0 {
            return;
        }
    }
    p[0] = (p[0] >> n) | (p[1] << (bits - n));
    p[1] >>= n;
}

fn smoothsort(arr: &mut [i32]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    let mut lp = vec![0usize; 12 * std::mem::size_of::<usize>()];
    let size = n;
    lp[0] = 1;
    lp[1] = 1;
    let mut i = 2;
    while lp[i - 2] + lp[i - 1] < size {
        lp[i] = lp[i - 2] + lp[i - 1];
        i += 1;
    }

    let mut p = [1usize, 0usize];
    let mut pshift = 1;
    let mut head = 0;

    while head < n - 1 {
        if (p[0] & 3) == 3 {
            insertion_sort(&mut arr[head..], &lp, pshift);
            shr(&mut p, 2);
            pshift += 2;
        } else {
            if lp[pshift.saturating_sub(1)] >= n - head {
                insertion_sort(&mut arr[head..], &lp, pshift);
            } else {
                insertion_sort(&mut arr[head..], &lp, pshift);
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
        head += 1;
    }
}

fn insertion_sort(arr: &mut [i32], _lp: &[usize], _pshift: usize) {
    for i in 1..arr.len() {
        let val = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > val {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = val;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: i32 = lines
        .next()
        .and_then(|line| line.ok())
        .and_then(|line| line.trim().parse().ok())
        .unwrap_or(-1);

    if n < 0 {
        return;
    }

    let n = n as usize;
    let mut arr = Vec::with_capacity(n.max(1));
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            if let Ok(val) = line.trim().parse::<i32>() {
                arr.push(val);
            }
        }
    }

    smoothsort(&mut arr);

    for (i, &val) in arr.iter().enumerate() {
        print!("{}", val);
        if i + 1 < arr.len() {
            print!(" ");
        }
    }
    println!();
}
