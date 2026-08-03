use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();

    if stdin.read_line(&mut line).is_err() || line.trim().is_empty() {
        return;
    }

    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    if parts.len() != 2 {
        return;
    }

    let nmemb_u64: u64 = match parts[0].parse() {
        Ok(n) => n,
        Err(_) => return,
    };

    let size_u64: u64 = match parts[1].parse() {
        Ok(n) => n,
        Err(_) => return,
    };

    let nmemb = nmemb_u64 as usize;
    let size = size_u64 as usize;

    // MUL_NO_OVERFLOW = (size_t)1 << (sizeof(size_t) * 4)
    let mul_no_overflow: usize = 1 << (std::mem::size_of::<usize>() * 4);

    // reallocarray overflow check
    let overflow = (nmemb >= mul_no_overflow || size >= mul_no_overflow) &&
                   nmemb > 0 &&
                   usize::MAX / nmemb < size;

    if overflow {
        println!("OVERFLOW");
    } else {
        println!("OK size={}", nmemb_u64 * size_u64);
    }
}
