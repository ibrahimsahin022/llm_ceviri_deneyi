use std::os::raw::{c_char, c_double, c_int};

extern "C" {
    fn scanf(fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
}

fn main() {
    let mut price: c_double = 0.0;
    let mut qty: c_int = 0;

    unsafe {
        if scanf(
            b"%lf %d\0".as_ptr() as *const c_char,
            &mut price as *mut c_double,
            &mut qty as *mut c_int,
        ) != 2
        {
            return;
        }
        let total = price * (qty as c_double);
        printf(b"%g\n\0".as_ptr() as *const c_char, total);
    }
}
