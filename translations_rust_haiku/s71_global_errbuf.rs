use std::io;

static mut G_ERRBUF: [u8; 256] = [0; 256];

fn set_error(msg: &str) {
    unsafe {
        let bytes = msg.as_bytes();
        let len = bytes.len().min(255);
        G_ERRBUF[..len].copy_from_slice(&bytes[..len]);
        G_ERRBUF[len] = 0;
    }
}

fn get_error() -> String {
    unsafe {
        let mut end = 0;
        for i in 0..256 {
            if G_ERRBUF[i] == 0 {
                end = i;
                break;
            }
        }
        String::from_utf8_lossy(&G_ERRBUF[..end]).to_string()
    }
}

fn safe_divide(a: i32, b: i32) -> Result<i32, ()> {
    if b == 0 {
        set_error("division by zero");
        Err(())
    } else {
        Ok(a / b)
    }
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 2 {
                continue;
            }

            let a: i32 = match parts[0].parse() {
                Ok(v) => v,
                Err(_) => continue,
            };

            let b: i32 = match parts[1].parse() {
                Ok(v) => v,
                Err(_) => continue,
            };

            match safe_divide(a, b) {
                Ok(r) => println!("{}", r),
                Err(()) => println!("HATA: {}", get_error()),
            }
        }
    }
}
