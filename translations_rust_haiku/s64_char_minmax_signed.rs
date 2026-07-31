use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    if stdin.lock().read_line(&mut line).unwrap_or(0) == 0 {
        return;
    }

    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }

    let bytes = line.as_bytes();
    let mut have = false;
    let mut mn: i8 = 0;
    let mut mx: i8 = 0;

    for c in bytes {
        let c_signed = *c as i8;
        if !have {
            mn = c_signed;
            mx = c_signed;
            have = true;
        } else {
            if c_signed < mn {
                mn = c_signed;
            }
            if c_signed > mx {
                mx = c_signed;
            }
        }
    }

    println!("{} {}", mn as i32, mx as i32);
}
