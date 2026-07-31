use std::io::{self, Read};

static mut G_LOG_LEVEL: i32 = 1;

fn set_log_level(lvl: i32) {
    unsafe {
        G_LOG_LEVEL = lvl;
    }
}

fn log_msg(lvl: i32, msg: &str) {
    let names = ["DEBUG", "INFO", "WARN", "ERROR"];
    unsafe {
        if lvl >= G_LOG_LEVEL {
            println!("[{}] {}", names[lvl as usize], msg);
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lvl: i32 = input.trim().parse().unwrap();
    set_log_level(lvl);
    log_msg(0, "baslangic");
    log_msg(1, "bilgi mesaji");
    log_msg(2, "uyari mesaji");
    log_msg(3, "hata mesaji");
}
