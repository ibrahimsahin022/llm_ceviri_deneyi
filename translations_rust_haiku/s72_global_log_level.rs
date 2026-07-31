use std::io;

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
    if io::stdin().read_line(&mut input).is_err() {
        return;
    }

    let lvl: i32 = match input.trim().parse() {
        Ok(l) => l,
        Err(_) => return,
    };

    set_log_level(lvl);
    log_msg(0, "baslangic");
    log_msg(1, "bilgi mesaji");
    log_msg(2, "uyari mesaji");
    log_msg(3, "hata mesaji");
}
