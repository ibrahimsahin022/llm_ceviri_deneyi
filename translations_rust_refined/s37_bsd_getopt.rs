use std::io::{self, Read};

static mut OPTARG: Option<String> = None;
static mut OPTIND: i32 = 1;
static mut OPTOPT: i32 = 0;
static mut OPTERR: i32 = 1;
static mut OPTRESET: i32 = 0;

static mut PLACE: Vec<u8> = Vec::new();
static mut PLACE_POS: usize = 0;

fn getprogname() -> &'static str { "prog" }

unsafe fn place_at_end() -> bool { PLACE_POS >= PLACE.len() }

unsafe fn getopt(nargc: i32, nargv: &[String], ostr: &str) -> i32 {
    let ostr_b = ostr.as_bytes();

    if OPTRESET != 0 || place_at_end() {
        OPTRESET = 0;
        if OPTIND >= nargc {
            PLACE = Vec::new();
            PLACE_POS = 0;
            return -1;
        }
        PLACE = nargv[OPTIND as usize].clone().into_bytes();
        PLACE_POS = 0;
        if PLACE_POS >= PLACE.len() || PLACE[PLACE_POS] != b'-' {
            PLACE = Vec::new();
            PLACE_POS = 0;
            return -1;
        }
        PLACE_POS += 1;
        OPTOPT = if PLACE_POS < PLACE.len() { PLACE[PLACE_POS] as i32 } else { 0 };
        PLACE_POS += 1;
        if OPTOPT == '-' as i32 && place_at_end() {
            OPTIND += 1;
            PLACE = Vec::new();
            PLACE_POS = 0;
            return -1;
        }
        if OPTOPT == 0 {
            PLACE = Vec::new();
            PLACE_POS = 0;
            if !ostr_b.contains(&b'-') {
                return -1;
            }
            OPTOPT = '-' as i32;
        }
    } else {
        OPTOPT = PLACE[PLACE_POS] as i32;
        PLACE_POS += 1;
    }

    let oli_pos = if OPTOPT == ':' as i32 {
        None
    } else {
        ostr_b.iter().position(|&c| c as i32 == OPTOPT)
    };

    if OPTOPT == ':' as i32 || oli_pos.is_none() {
        if place_at_end() {
            OPTIND += 1;
        }
        if OPTERR != 0 && ostr_b.first() != Some(&b':') {
            eprintln!("{}: illegal option -- {}", getprogname(), OPTOPT as u8 as char);
        }
        return '?' as i32;
    }
    let oli = oli_pos.unwrap();

    if oli + 1 >= ostr_b.len() || ostr_b[oli + 1] != b':' {
        OPTARG = None;
        if place_at_end() {
            OPTIND += 1;
        }
    } else {
        if !place_at_end() {
            OPTARG = Some(String::from_utf8_lossy(&PLACE[PLACE_POS..]).to_string());
        } else if oli + 2 < ostr_b.len() && ostr_b[oli + 2] == b':' {
            OPTARG = None;
        } else {
            OPTIND += 1;
            if OPTIND < nargc {
                OPTARG = Some(nargv[OPTIND as usize].clone());
            } else {
                PLACE = Vec::new();
                PLACE_POS = 0;
                if ostr_b.first() == Some(&b':') {
                    return ':' as i32;
                }
                if OPTERR != 0 {
                    eprintln!(
                        "{}: option requires an argument -- {}",
                        getprogname(),
                        OPTOPT as u8 as char
                    );
                }
                return '?' as i32;
            }
        }
        PLACE = Vec::new();
        PLACE_POS = 0;
        OPTIND += 1;
    }
    OPTOPT
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let ostr = lines.next().unwrap_or("").to_string();
    let n: usize = lines.next().unwrap_or("0").trim().parse().unwrap_or(0);

    let mut argv: Vec<String> = vec!["prog".to_string()];
    for _ in 0..n {
        argv.push(lines.next().unwrap_or("").to_string());
    }
    let argc = argv.len() as i32;

    unsafe {
        loop {
            let c = getopt(argc, &argv, &ostr);
            if c == -1 {
                break;
            }
            if c == '?' as i32 {
                println!("opt=? arg=(null)");
            } else if c == ':' as i32 {
                println!("opt=: arg=(null)");
            } else {
                match &OPTARG {
                    Some(a) => println!("opt={} arg={}", c as u8 as char, a),
                    None => println!("opt={} arg=(null)", c as u8 as char),
                }
            }
        }
        for i in OPTIND..argc {
            println!("REST:{}", argv[i as usize]);
        }
    }
}
