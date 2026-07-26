use std::io::Read;

extern "C" {
    fn malloc(size: usize) -> *mut std::ffi::c_void;
    fn free(ptr: *mut std::ffi::c_void);
    fn realloc(ptr: *mut std::ffi::c_void, size: usize) -> *mut std::ffi::c_void;
    fn sprintf(str: *mut std::os::raw::c_char, format: *const std::os::raw::c_char, ...) -> std::os::raw::c_int;
    fn sscanf(str: *const std::os::raw::c_char, format: *const std::os::raw::c_char, ...) -> std::os::raw::c_int;
    fn strtod(str: *const std::os::raw::c_char, endptr: *mut *mut std::os::raw::c_char) -> f64;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct internal_hooks {
    pub allocate: Option<unsafe extern "C" fn(usize) -> *mut std::ffi::c_void>,
    pub deallocate: Option<unsafe extern "C" fn(*mut std::ffi::c_void)>,
    pub reallocate: Option<unsafe extern "C" fn(*mut std::ffi::c_void, usize) -> *mut std::ffi::c_void>,
}

static GLOBAL_HOOKS: internal_hooks = internal_hooks {
    allocate: Some(malloc),
    deallocate: Some(free),
    reallocate: Some(realloc),
};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cJSON {
    pub type_: std::os::raw::c_int,
    pub valueint: std::os::raw::c_int,
    pub valuedouble: f64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct parse_buffer {
    pub content: *const u8,
    pub length: usize,
    pub offset: usize,
    pub depth: usize,
    pub hooks: internal_hooks,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct printbuffer {
    pub buffer: *mut u8,
    pub length: usize,
    pub offset: usize,
    pub depth: usize,
    pub noalloc: i32,
    pub format: i32,
    pub hooks: internal_hooks,
}

fn get_decimal_point() -> u8 {
    b'.'
}

fn compare_double(a: f64, b: f64) -> i32 {
    let max_val = if a.abs() > b.abs() { a.abs() } else { b.abs() };
    if (a - b).abs() <= max_val * f64::EPSILON {
        1
    } else {
        0
    }
}

unsafe fn can_access_at_index(buffer: *const parse_buffer, index: usize) -> bool {
    !buffer.is_null() && (((*buffer).offset + index) < (*buffer).length)
}

unsafe fn buffer_at_offset(buffer: *const parse_buffer) -> *const u8 {
    (*buffer).content.add((*buffer).offset)
}

unsafe fn ensure(p: *mut printbuffer, mut needed: usize) -> *mut u8 {
    let mut newbuffer: *mut u8;
    let newsize: usize;

    if p.is_null() || (*p).buffer.is_null() {
        return std::ptr::null_mut();
    }

    if (*p).length > 0 && (*p).offset >= (*p).length {
        return std::ptr::null_mut();
    }

    if needed > i32::MAX as usize {
        return std::ptr::null_mut();
    }

    needed += (*p).offset + 1;
    if needed <= (*p).length {
        return (*p).buffer.add((*p).offset);
    }

    if (*p).noalloc != 0 {
        return std::ptr::null_mut();
    }

    if needed > (i32::MAX / 2) as usize {
        if needed <= i32::MAX as usize {
            newsize = i32::MAX as usize;
        } else {
            return std::ptr::null_mut();
        }
    } else {
        newsize = needed * 2;
    }

    if let Some(realloc_fn) = (*p).hooks.reallocate {
        newbuffer = realloc_fn((*p).buffer as *mut _, newsize) as *mut u8;
        if newbuffer.is_null() {
            if let Some(dealloc_fn) = (*p).hooks.deallocate {
                dealloc_fn((*p).buffer as *mut _);
            }
            (*p).length = 0;
            (*p).buffer = std::ptr::null_mut();
            return std::ptr::null_mut();
        }
    } else {
        if let Some(alloc_fn) = (*p).hooks.allocate {
            newbuffer = alloc_fn(newsize) as *mut u8;
        } else {
            newbuffer = std::ptr::null_mut();
        }
        if newbuffer.is_null() {
            if let Some(dealloc_fn) = (*p).hooks.deallocate {
                dealloc_fn((*p).buffer as *mut _);
            }
            (*p).length = 0;
            (*p).buffer = std::ptr::null_mut();
            return std::ptr::null_mut();
        }

        std::ptr::copy_nonoverlapping((*p).buffer, newbuffer, (*p).offset + 1);
        if let Some(dealloc_fn) = (*p).hooks.deallocate {
            dealloc_fn((*p).buffer as *mut _);
        }
    }

    (*p).length = newsize;
    (*p).buffer = newbuffer;

    newbuffer.add((*p).offset)
}

unsafe fn update_offset(buffer: *mut printbuffer) {
    if buffer.is_null() || (*buffer).buffer.is_null() {
        return;
    }
    let buffer_pointer = (*buffer).buffer.add((*buffer).offset);
    let mut len = 0usize;
    while *buffer_pointer.add(len) != 0 {
        len += 1;
    }
    (*buffer).offset += len;
}

unsafe fn parse_number(item: *mut cJSON, input_buffer: *mut parse_buffer) -> i32 {
    let mut number: f64;
    let mut after_end: *mut u8 = std::ptr::null_mut();
    let number_c_string: *mut u8;
    let decimal_point = get_decimal_point();
    let mut i: usize = 0;
    let mut number_string_length: usize = 0;
    let mut has_decimal_point: i32 = 0;

    if input_buffer.is_null() || (*input_buffer).content.is_null() {
        return 0;
    }

    while can_access_at_index(input_buffer, i) {
        let ch = *buffer_at_offset(input_buffer).add(i);
        match ch {
            b'0'..=b'9' | b'+' | b'-' | b'e' | b'E' => {
                number_string_length += 1;
            }
            b'.' => {
                number_string_length += 1;
                has_decimal_point = 1;
            }
            _ => break,
        }
        i += 1;
    }

    let alloc_fn = match (*input_buffer).hooks.allocate {
        Some(f) => f,
        None => return 0,
    };
    number_c_string = alloc_fn(number_string_length + 1) as *mut u8;
    if number_c_string.is_null() {
        return 0;
    }

    std::ptr::copy_nonoverlapping(buffer_at_offset(input_buffer), number_c_string, number_string_length);
    *number_c_string.add(number_string_length) = 0;

    if has_decimal_point != 0 {
        for j in 0..number_string_length {
            if *number_c_string.add(j) == b'.' {
                *number_c_string.add(j) = decimal_point;
            }
        }
    }

    number = strtod(number_c_string as *const _, &mut (after_end as *mut _));
    if number_c_string == after_end {
        if let Some(deallocate) = (*input_buffer).hooks.deallocate {
            deallocate(number_c_string as *mut _);
        }
        return 0;
    }

    (*item).valuedouble = number;

    if number >= i32::MAX as f64 {
        (*item).valueint = i32::MAX;
    } else if number <= i32::MIN as f64 {
        (*item).valueint = i32::MIN;
    } else {
        (*item).valueint = number as i32;
    }

    (*item).type_ = 1 << 3;

    (*input_buffer).offset += after_end.offset_from(number_c_string) as usize;
    if let Some(deallocate) = (*input_buffer).hooks.deallocate {
        deallocate(number_c_string as *mut _);
    }
    1
}

unsafe fn print_number(item: *const cJSON, output_buffer: *mut printbuffer) -> i32 {
    let output_pointer: *mut u8;
    let d = (*item).valuedouble;
    let mut length: i32;
    let mut number_buffer = [0u8; 26];
    let decimal_point = get_decimal_point();
    let mut test: f64 = 0.0;

    if output_buffer.is_null() {
        return 0;
    }

    if d.is_nan() || d.is_infinite() {
        let fmt = b"null\0".as_ptr() as *const _;
        length = sprintf(number_buffer.as_mut_ptr() as *mut _, fmt);
    } else if d == (*item).valueint as f64 {
        let fmt = b"%d\0".as_ptr() as *const _;
        length = sprintf(number_buffer.as_mut_ptr() as *mut _, fmt, (*item).valueint);
    } else {
        let fmt15 = b"%1.15g\0".as_ptr() as *const _;
        length = sprintf(number_buffer.as_mut_ptr() as *mut _, fmt15, d);

        let fmt_lg = b"%lg\0".as_ptr() as *const _;
        if sscanf(number_buffer.as_ptr() as *const _, fmt_lg, &mut test as *mut f64) != 1
            || compare_double(test, d) == 0
        {
            let fmt17 = b"%1.17g\0".as_ptr() as *const _;
            length = sprintf(number_buffer.as_mut_ptr() as *mut _, fmt17, d);
        }
    }

    if length < 0 || length > (number_buffer.len() - 1) as i32 {
        return 0;
    }

    output_pointer = ensure(output_buffer, length as usize + 1);
    if output_pointer.is_null() {
        return 0;
    }

    let mut i: usize = 0;
    while i < length as usize {
        if number_buffer[i] == decimal_point {
            *output_pointer.add(i) = b'.';
        } else {
            *output_pointer.add(i) = number_buffer[i];
        }
        i += 1;
    }
    *output_pointer.add(i) = 0;

    (*output_buffer).offset += length as usize;

    1
}

struct StdinReader<R: Read> {
    inner: R,
    peeked: Option<u8>,
}

impl<R: Read> StdinReader<R> {
    fn new(inner: R) -> Self {
        StdinReader { inner, peeked: None }
    }

    fn getchar(&mut self) -> Option<u8> {
        if let Some(b) = self.peeked.take() {
            return Some(b);
        }
        let mut buf = [0u8; 1];
        if self.inner.read_exact(&mut buf).is_ok() {
            Some(buf[0])
        } else {
            None
        }
    }

    fn peek(&mut self) -> Option<u8> {
        if self.peeked.is_none() {
            self.peeked = self.getchar();
        }
        self.peeked
    }

    fn scanf_d(&mut self) -> Option<i32> {
        while let Some(b) = self.peek() {
            if (b as char).is_ascii_whitespace() {
                self.getchar();
            } else {
                break;
            }
        }
        let mut sign = 1i32;
        if let Some(b) = self.peek() {
            if b == b'-' {
                sign = -1;
                self.getchar();
            } else if b == b'+' {
                self.getchar();
            }
        }
        let mut val = 0i32;
        let mut count = 0;
        while let Some(b) = self.peek() {
            if b.is_ascii_digit() {
                self.getchar();
                val = val.wrapping_mul(10).wrapping_add((b - b'0') as i32);
                count += 1;
            } else {
                break;
            }
        }
        if count > 0 {
            Some(val.wrapping_mul(sign))
        } else {
            None
        }
    }

    fn fgets(&mut self, buf: &mut [u8]) -> bool {
        if buf.len() <= 1 {
            return false;
        }
        let max_len = buf.len() - 1;
        let mut idx = 0;
        while idx < max_len {
            if let Some(b) = self.getchar() {
                buf[idx] = b;
                idx += 1;
                if b == b'\n' {
                    break;
                }
            } else {
                break;
            }
        }
        if idx == 0 {
            return false;
        }
        buf[idx] = 0;
        true
    }
}

fn main() {
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    let mut reader = StdinReader::new(handle);

    let n = match reader.scanf_d() {
        Some(val) => val,
        None => std::process::exit(1),
    };
    reader.getchar();

    for _ in 0..n {
        let mut line = [0u8; 128];
        if !reader.fgets(&mut line) {
            break;
        }
        let mut l = 0usize;
        while line[l] != 0 {
            l += 1;
        }
        while l > 0 && (line[l - 1] == b'\n' || line[l - 1] == b'\r') {
            l -= 1;
            line[l] = 0;
        }

        unsafe {
            let mut pb = parse_buffer {
                content: line.as_ptr(),
                length: l,
                offset: 0,
                depth: 0,
                hooks: GLOBAL_HOOKS,
            };

            let mut item = cJSON {
                type_: 0,
                valueint: 0,
                valuedouble: 0.0,
            };

            if parse_number(&mut item, &mut pb) == 0 {
                println!("PARSE_ERROR");
                continue;
            }

            let mut outbuf = [0u8; 64];
            let mut out = printbuffer {
                buffer: outbuf.as_mut_ptr(),
                length: outbuf.len(),
                offset: 0,
                depth: 0,
                noalloc: 0,
                format: 0,
                hooks: GLOBAL_HOOKS,
            };

            if print_number(&item, &mut out) == 0 {
                println!("PRINT_ERROR");
                continue;
            }

            let len = (0..outbuf.len())
                .position(|i| outbuf[i] == 0)
                .unwrap_or(outbuf.len());
            let out_str = std::str::from_utf8_unchecked(&outbuf[..len]);
            println!("{}", out_str);
        }
    }
}
