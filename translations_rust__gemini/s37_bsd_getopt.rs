static mut OPTERR: i32 = 1;
static mut OPTIND: i32 = 1;
static mut OPTOPT: i32 = 0;
static mut OPTRESET: i32 = 0;
static mut OPTARG: *mut i8 = std::ptr::null_mut();

static EMSG: [i8; 1] = [0];
static mut PLACE: *const i8 = std::ptr::null();

const BADCH: i32 = '?' as i32;
const BADARG: i32 = ':' as i32;

unsafe fn _getprogname() -> &'static str {
    "prog"
}

unsafe fn strchr(mut s: *const i8, c: i32) -> *const i8 {
    let target = c as u8 as i8;
    while *s != 0 {
        if *s == target {
            return s;
        }
        s = s.add(1);
    }
    if target == 0 {
        return s;
    }
    std::ptr::null()
}

unsafe fn getopt(nargc: i32, nargv: *const *mut i8, ostr: *const i8) -> i32 {
    if PLACE.is_null() {
        PLACE = EMSG.as_ptr();
    }

    let oli: *const i8;

    if OPTRESET != 0 || *PLACE == 0 {
        OPTRESET = 0;
        if OPTIND >= nargc {
            PLACE = EMSG.as_ptr();
            return -1;
        }
        PLACE = *nargv.add(OPTIND as usize);
        if PLACE.is_null() {
            PLACE = EMSG.as_ptr();
            return -1;
        }
        let ch = *PLACE;
        PLACE = PLACE.add(1);
        if ch != b'-' as i8 {
            PLACE = EMSG.as_ptr();
            return -1;
        }
        OPTOPT = *PLACE as i32;
        PLACE = PLACE.add(1);

        if OPTOPT == b'-' as i32 && *PLACE == 0 {
            OPTIND += 1;
            PLACE = EMSG.as_ptr();
            return -1;
        }
        if OPTOPT == 0 {
            PLACE = EMSG.as_ptr();
            if strchr(ostr, b'-' as i32).is_null() {
                return -1;
            }
            OPTOPT = b'-' as i32;
        }
    } else {
        OPTOPT = *PLACE as i32;
        PLACE = PLACE.add(1);
    }

    oli = strchr(ostr, OPTOPT);
    if OPTOPT == b':' as i32 || oli.is_null() {
        if *PLACE == 0 {
            OPTIND += 1;
        }
        if OPTERR != 0 && *ostr != b':' as i8 {
            eprintln!("{}: illegal option -- {}", _getprogname(), OPTOPT as u8 as char);
        }
        return BADCH;
    }

    if *oli.add(1) != b':' as i8 {
        OPTARG = std::ptr::null_mut();
        if *PLACE == 0 {
            OPTIND += 1;
        }
    } else {
        if *PLACE != 0 {
            OPTARG = PLACE as *mut i8;
        } else if *oli.add(2) == b':' as i8 {
            OPTARG = std::ptr::null_mut();
        } else {
            OPTIND += 1;
            if nargc > OPTIND {
                OPTARG = *nargv.add(OPTIND as usize);
            } else {
                PLACE = EMSG.as_ptr();
                if *ostr == b':' as i8 {
                    return BADARG;
                }
                if OPTERR != 0 {
                    eprintln!("{}: option requires an argument -- {}", _getprogname(), OPTOPT as u8 as char);
                }
                return BADCH;
            }
        }
        PLACE = EMSG.as_ptr();
        OPTIND += 1;
    }
    OPTOPT
}

fn main() {
    use std::io::BufRead;
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();

    let mut line = String::new();
    if handle.read_line(&mut line).is_err() || line.is_empty() {
        return;
    }
    let ostr = line.trim_matches(&['\r', '\n'][..]).to_string();
    let ostr_c = match std::ffi::CString::new(ostr) {
        Ok(c) => c,
        Err(_) => return,
    };

    line.clear();
    if handle.read_line(&mut line).is_err() || line.is_empty() {
        return;
    }
    let n: usize = match line.trim().parse() {
        Ok(v) => v,
        Err(_) => return,
    };

    let mut argv_cstrings = Vec::with_capacity(n + 1);
    argv_cstrings.push(std::ffi::CString::new("prog").unwrap());

    for _ in 0..n {
        line.clear();
        if handle.read_line(&mut line).is_err() {
            line.clear();
        }
        let s = line.trim_matches(&['\r', '\n'][..]);
        argv_cstrings.push(std::ffi::CString::new(s).unwrap());
    }

    let argv_: Vec<*mut i8> = argv_cstrings
        .iter()
        .map(|cs| cs.as_ptr() as *mut i8)
        .collect();
    let argc_ = (n + 1) as i32;

    unsafe {
        loop {
            let c = getopt(argc_, argv_.as_ptr(), ostr_c.as_ptr());
            if c == -1 {
                break;
            }
            if c == '?' as i32 {
                println!("opt=? arg=(null)");
            } else if c == ':' as i32 {
                println!("opt=: arg=(null)");
            } else {
                let arg_str = if !OPTARG.is_null() {
                    std::ffi::CStr::from_ptr(OPTARG).to_str().unwrap()
                } else {
                    "(null)"
                };
                println!("opt={} arg={}", c as u8 as char, arg_str);
            }
        }
        for i in OPTIND..argc_ {
            let arg_str = std::ffi::CStr::from_ptr(*argv_.as_ptr().add(i as usize))
                .to_str()
                .unwrap();
            println!("REST:{}", arg_str);
        }
    }
}
