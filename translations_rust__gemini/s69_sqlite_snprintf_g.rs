extern "C" {
    fn scanf(fmt: *const u8, ...) -> i32;
    fn printf(fmt: *const u8, ...) -> i32;
}

fn main() {
    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;
    unsafe {
        if scanf(b"%lf %lf\0".as_ptr(), &mut a as *mut f64, &mut b as *mut f64) != 2 || b == 0.0 {
            return;
        }
        let ratio = a / b;
        printf(b"%g\n\0".as_ptr(), ratio);
    }
}
