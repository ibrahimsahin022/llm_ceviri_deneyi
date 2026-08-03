use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let line = input.lines().next().unwrap_or("");
    let mut have = false;
    let mut mn: i8 = 0;
    let mut mx: i8 = 0;
    for b in line.bytes() {
        let c = b as i8;
        if !have {
            mn = c;
            mx = c;
            have = true;
        } else {
            if c < mn {
                mn = c;
            }
            if c > mx {
                mx = c;
            }
        }
    }
    println!("{} {}", mn as i32, mx as i32);
}
