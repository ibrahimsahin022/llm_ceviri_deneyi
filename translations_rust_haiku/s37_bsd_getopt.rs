use std::io::{self, BufRead};
use std::sync::Mutex;
use std::sync::OnceLock;

const BADCH: i32 = '?' as i32;
const BADARG: i32 = ':' as i32;
const EMSG: &str = "";

struct GetoptState {
    opterr: i32,
    optind: i32,
    optopt: i32,
    optreset: i32,
    optarg: Option<String>,
    place: String,
}

impl GetoptState {
    fn new() -> Self {
        GetoptState {
            opterr: 1,
            optind: 1,
            optopt: 0,
            optreset: 0,
            optarg: None,
            place: String::new(),
        }
    }
}

fn getopt_state() -> &'static Mutex<GetoptState> {
    static STATE: OnceLock<Mutex<GetoptState>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(GetoptState::new()))
}

fn getopt(nargc: i32, nargv: &[&str], ostr: &str) -> i32 {
    let mut state = getopt_state().lock().unwrap();

    if state.optreset != 0 || state.place.is_empty() {
        state.optreset = 0;
        if state.optind >= nargc {
            state.place = String::new();
            return -1;
        }
        let arg = nargv[state.optind as usize];
        if !arg.starts_with('-') {
            state.place = String::new();
            return -1;
        }
        let mut chars = arg.chars();
        chars.next(); // skip '-'
        state.place = chars.collect();

        if state.place.starts_with('-') && state.place.len() == 1 {
            state.optind += 1;
            state.place = String::new();
            return -1;
        }

        if state.place.is_empty() {
            state.place = String::new();
            if !ostr.contains('-') {
                return -1;
            }
            state.optopt = '-' as i32;
        } else {
            let mut chars = state.place.chars();
            if let Some(c) = chars.next() {
                state.optopt = c as i32;
                state.place = chars.collect();
            }
        }
    } else {
        let mut chars = state.place.chars();
        if let Some(c) = chars.next() {
            state.optopt = c as i32;
            state.place = chars.collect();
        }
    }

    if state.optopt == ':' as i32
        || ostr.chars().find(|&c| c as i32 == state.optopt).is_none()
    {
        if state.place.is_empty() {
            state.optind += 1;
        }
        if state.opterr != 0 && !ostr.starts_with(':') {
            eprintln!(
                "prog: illegal option -- {}\n",
                (state.optopt as u8) as char
            );
        }
        return BADCH;
    }

    let opt_char = (state.optopt as u8) as char;
    let ostr_idx = ostr.chars().position(|c| c == opt_char).unwrap_or(0);
    let ostr_chars: Vec<char> = ostr.chars().collect();
    let needs_arg = ostr_idx + 1 < ostr_chars.len() && ostr_chars[ostr_idx + 1] == ':';

    if !needs_arg {
        state.optarg = None;
        if state.place.is_empty() {
            state.optind += 1;
        }
    } else {
        if !state.place.is_empty() {
            state.optarg = Some(state.place.clone());
        } else if ostr_idx + 2 < ostr_chars.len() && ostr_chars[ostr_idx + 2] == ':' {
            state.optarg = None;
        } else if state.optind + 1 < nargc {
            state.optind += 1;
            state.optarg = Some(nargv[state.optind as usize].to_string());
        } else {
            state.place = String::new();
            if ostr.starts_with(':') {
                return BADARG;
            }
            if state.opterr != 0 {
                eprintln!("prog: option requires an argument -- {}\n", opt_char);
            }
            return BADCH;
        }
        state.place = String::new();
        state.optind += 1;
    }

    state.optopt
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let ostr = lines
        .next()
        .and_then(|line| line.ok())
        .unwrap_or_default();
    let ostr = ostr.trim_end_matches('\n').trim_end_matches('\r');

    let n: i32 = lines
        .next()
        .and_then(|line| line.ok())
        .and_then(|line| line.trim().parse().ok())
        .unwrap_or(0);

    let mut argv: Vec<&str> = vec!["prog"];
    let args_vec: Vec<String> = lines
        .take(n as usize)
        .filter_map(|line| line.ok())
        .map(|line| line.trim_end_matches('\n').trim_end_matches('\r').to_string())
        .collect();

    for arg in &args_vec {
        argv.push(arg);
    }

    loop {
        let c = getopt(argv.len() as i32, &argv, ostr);
        if c == -1 {
            break;
        }

        let state = getopt_state().lock().unwrap();
        if c == BADCH {
            println!("opt=? arg=(null)");
        } else if c == BADARG {
            println!("opt=: arg=(null)");
        } else {
            let arg_str = state
                .optarg
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or("(null)");
            println!("opt={} arg={}", c as u8 as char, arg_str);
        }
    }

    let state = getopt_state().lock().unwrap();
    for i in state.optind as usize..argv.len() {
        println!("REST:{}", argv[i]);
    }
}
