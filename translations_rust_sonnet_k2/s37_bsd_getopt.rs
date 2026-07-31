use std::io::{self, Read};

struct GetoptState {
    optind: i32,
    optopt: i32,
    optreset: i32,
    opterr: i32,
    optarg: Option<Vec<u8>>,
    place: Vec<u8>,
}

impl GetoptState {
    fn new() -> Self {
        GetoptState {
            optind: 1,
            optopt: 0,
            optreset: 0,
            opterr: 1,
            optarg: None,
            place: Vec::new(),
        }
    }
}

const BADCH: i32 = b'?' as i32;
const BADARG: i32 = b':' as i32;

fn getopt(argv: &[Vec<u8>], ostr: &[u8], st: &mut GetoptState, prog_name: &str) -> i32 {
    let nargc = argv.len() as i32;

    if st.optreset != 0 || st.place.is_empty() {
        st.optreset = 0;
        if st.optind >= nargc {
            st.place = Vec::new();
            return -1;
        }
        let mut p = argv[st.optind as usize].clone();
        let first = if p.is_empty() { 0u8 } else { p.remove(0) };
        if first != b'-' {
            st.place = Vec::new();
            return -1;
        }
        st.optopt = if p.is_empty() { 0 } else { p.remove(0) as i32 };
        if st.optopt == b'-' as i32 && p.is_empty() {
            st.optind += 1;
            st.place = Vec::new();
            return -1;
        }
        if st.optopt == 0 {
            st.place = Vec::new();
            if !ostr.contains(&b'-') {
                return -1;
            }
            st.optopt = b'-' as i32;
        }
        st.place = p;
    } else {
        st.optopt = if st.place.is_empty() {
            0
        } else {
            st.place.remove(0) as i32
        };
    }

    let oli_pos = if st.optopt == b':' as i32 {
        None
    } else {
        ostr.iter().position(|&c| c as i32 == st.optopt)
    };

    if st.optopt == b':' as i32 || oli_pos.is_none() {
        if st.place.is_empty() {
            st.optind += 1;
        }
        if st.opterr != 0 && (ostr.is_empty() || ostr[0] != b':') {
            eprintln!(
                "{}: illegal option -- {}",
                prog_name, st.optopt as u8 as char
            );
        }
        return BADCH;
    }

    let oli = oli_pos.unwrap();
    let needs_arg = oli + 1 < ostr.len() && ostr[oli + 1] == b':';

    if !needs_arg {
        st.optarg = None;
        if st.place.is_empty() {
            st.optind += 1;
        }
    } else {
        if !st.place.is_empty() {
            st.optarg = Some(st.place.clone());
        } else if oli + 2 < ostr.len() && ostr[oli + 2] == b':' {
            st.optarg = None;
        } else {
            st.optind += 1;
            if nargc > st.optind {
                st.optarg = Some(argv[st.optind as usize].clone());
            } else {
                st.place = Vec::new();
                if !ostr.is_empty() && ostr[0] == b':' {
                    return BADARG;
                }
                if st.opterr != 0 {
                    eprintln!(
                        "{}: option requires an argument -- {}",
                        prog_name, st.optopt as u8 as char
                    );
                }
                return BADCH;
            }
        }
        st.place = Vec::new();
        st.optind += 1;
    }

    st.optopt
}

fn is_space_ws(b: u8) -> bool {
    b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' || b == 0x0B || b == 0x0C
}

fn fgets_sim(input: &[u8], pos: &mut usize, cap: usize) -> Option<Vec<u8>> {
    if *pos >= input.len() {
        return None;
    }
    let start = *pos;
    let maxdata = cap - 1;
    let mut count = 0usize;
    let mut end = *pos;
    while end < input.len() && count < maxdata {
        if input[end] == b'\n' {
            end += 1;
            break;
        }
        end += 1;
        count += 1;
    }
    *pos = end;
    Some(input[start..end].to_vec())
}

fn strcspn_trim(s: &mut Vec<u8>) {
    // find first occurrence of '\r' or '\n', truncate there
    let mut cut = s.len();
    for (i, &b) in s.iter().enumerate() {
        if b == b'\r' || b == b'\n' {
            cut = i;
            break;
        }
    }
    s.truncate(cut);
}

fn main() {
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        return;
    }
    let mut pos = 0usize;
    let total_len = input.len();

    let mut ostr = match fgets_sim(&input, &mut pos, 256) {
        Some(l) => l,
        None => return,
    };
    strcspn_trim(&mut ostr);

    // scanf("%d", &n)
    while pos < total_len && is_space_ws(input[pos]) {
        pos += 1;
    }
    let sign_start = pos;
    if pos < total_len && (input[pos] == b'+' || input[pos] == b'-') {
        pos += 1;
    }
    let digits_start = pos;
    while pos < total_len && input[pos].is_ascii_digit() {
        pos += 1;
    }
    if digits_start == pos {
        return;
    }
    let n: i32 = match std::str::from_utf8(&input[sign_start..pos])
        .ok()
        .and_then(|s| s.parse().ok())
    {
        Some(v) => v,
        None => return,
    };

    // getchar(): consume exactly one character
    if pos < total_len {
        pos += 1;
    }

    let mut argv_: Vec<Vec<u8>> = Vec::new();
    argv_.push(b"prog".to_vec());

    for _ in 0..n {
        let mut buf = match fgets_sim(&input, &mut pos, 256) {
            Some(l) => l,
            None => Vec::new(),
        };
        strcspn_trim(&mut buf);
        argv_.push(buf);
    }

    let mut st = GetoptState::new();

    loop {
        let c = getopt(&argv_, &ostr, &mut st, "prog");
        if c == -1 {
            break;
        }
        if c == b'?' as i32 {
            println!("opt=? arg=(null)");
        } else if c == b':' as i32 {
            println!("opt=: arg=(null)");
        } else {
            let arg_str = match &st.optarg {
                Some(a) => String::from_utf8_lossy(a).into_owned(),
                None => "(null)".to_string(),
            };
            println!("opt={} arg={}", c as u8 as char, arg_str);
        }
    }

    let argc_ = argv_.len() as i32;
    let mut i = st.optind;
    while i < argc_ {
        let rest_str = String::from_utf8_lossy(&argv_[i as usize]).into_owned();
        println!("REST:{}", rest_str);
        i += 1;
    }
}
