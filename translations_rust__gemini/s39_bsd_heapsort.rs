use std::alloc::{alloc, dealloc, Layout};
use std::ffi::c_int;
use std::io::{self, Read};
use std::ptr;

unsafe fn heapsort(
    vbase: *mut u8,
    mut nmemb: usize,
    size: usize,
    compar: unsafe extern "C" fn(*const u8, *const u8) -> c_int,
) -> c_int {
    if nmemb <= 1 {
        return 0;
    }

    if size == 0 {
        return -1;
    }

    let layout = match Layout::from_size_align(size, 1) {
        Ok(l) => l,
        Err(_) => return -1,
    };

    let k = alloc(layout);
    if k.is_null() {
        return -1;
    }

    let base = vbase.wrapping_sub(size);

    let create = |initval: usize, nmemb: usize| {
        let mut par_i = initval;
        while par_i * 2 <= nmemb {
            let mut child_i = par_i * 2;
            let mut child = base.wrapping_add(child_i * size);
            if child_i < nmemb && compar(child as *const u8, child.wrapping_add(size) as *const u8) < 0 {
                child = child.wrapping_add(size);
                child_i += 1;
            }
            let par = base.wrapping_add(par_i * size);
            if compar(child as *const u8, par as *const u8) <= 0 {
                break;
            }
            ptr::swap_nonoverlapping(par, child, size);
            par_i = child_i;
        }
    };

    let select = |nmemb: usize| {
        let mut par_i = 1;
        while par_i * 2 <= nmemb {
            let mut child_i = par_i * 2;
            let mut child = base.wrapping_add(child_i * size);
            if child_i < nmemb && compar(child as *const u8, child.wrapping_add(size) as *const u8) < 0 {
                child = child.wrapping_add(size);
                child_i += 1;
            }
            let par = base.wrapping_add(par_i * size);
            ptr::copy_nonoverlapping(child as *const u8, par, size);
            par_i = child_i;
        }
        loop {
            let child_i = par_i;
            par_i = child_i / 2;
            let child = base.wrapping_add(child_i * size);
            let par = base.wrapping_add(par_i * size);
            if child_i == 1 || compar(k as *const u8, par as *const u8) < 0 {
                ptr::copy_nonoverlapping(k as *const u8, child, size);
                break;
            }
            ptr::copy_nonoverlapping(par as *const u8, child, size);
        }
    };

    for l in (1..=nmemb / 2).rev() {
        create(l, nmemb);
    }

    while nmemb > 1 {
        ptr::copy_nonoverlapping(base.wrapping_add(nmemb * size) as *const u8, k, size);
        ptr::copy_nonoverlapping(base.wrapping_add(size) as *const u8, base.wrapping_add(nmemb * size), size);
        nmemb -= 1;
        select(nmemb);
    }

    dealloc(k, layout);
    0
}

unsafe extern "C" fn int_cmp(a: *const u8, b: *const u8) -> c_int {
    let ia = *(a as *const i32);
    let ib = *(b as *const i32);
    (ia > ib) as c_int - (ia < ib) as c_int
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut words = input.split_whitespace();
    let n_str = match words.next() {
        Some(s) => s,
        None => return,
    };
    let n: i32 = match n_str.parse() {
        Ok(val) => val,
        Err(_) => return,
    };

    let count = if n > 0 { n as usize } else { 0 };
    let mut arr: Vec<i32> = Vec::with_capacity(if count > 0 { count } else { 1 });

    for _ in 0..n {
        if let Some(w) = words.next() {
            if let Ok(val) = w.parse::<i32>() {
                arr.push(val);
            }
        }
    }

    if n > 0 {
        unsafe {
            heapsort(
                arr.as_mut_ptr() as *mut u8,
                count,
                std::mem::size_of::<i32>(),
                int_cmp,
            );
        }
    }

    for i in 0..n {
        if i + 1 < n {
            print!("{} ", arr[i as usize]);
        } else {
            println!("{}", arr[i as usize]);
        }
    }
    if n == 0 {
        println!();
    }
}
