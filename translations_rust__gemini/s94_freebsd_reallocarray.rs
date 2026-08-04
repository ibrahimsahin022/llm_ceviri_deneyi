use std::ffi::c_void;
use std::os::raw::{c_char, c_ulonglong};

extern "C" {
    fn scanf(fmt: *const c_char, ...) -> i32;
    fn printf(fmt: *const c_char, ...) -> i32;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

const MUL_NO_OVERFLOW: usize = 1 << (std::mem::size_of::<usize>() * 4);

unsafe fn reallocarray(optr: *mut c_void, nmemb: usize, size: usize) -> *mut c_void {
    if (nmemb >= MUL_NO_OVERFLOW || size >= MUL_NO_OVERFLOW)
        && nmemb > 0
        && usize::MAX / nmemb < size
    {
        return std::ptr::null_mut();
    }
    realloc(optr, size.wrapping_mul(nmemb))
}

fn main() {
    unsafe {
        let mut nmemb: c_ulonglong = 0;
        let mut size: c_ulonglong = 0;
        if scanf(b"%llu %llu\0".as_ptr() as *const c_char, &mut nmemb, &mut size) != 2 {
            return;
        }
        let p = reallocarray(std::ptr::null_mut(), nmemb as usize, size as usize);
        if p.is_null() {
            printf(b"OVERFLOW\n\0".as_ptr() as *const c_char);
        } else {
            printf(
                b"OK size=%llu\n\0".as_ptr() as *const c_char,
                nmemb.wrapping_mul(size),
            );
            free(p);
        }
    }
}
