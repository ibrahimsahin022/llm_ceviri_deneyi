use std::io::{self, Read};
use std::sync::atomic::{AtomicI32, Ordering};

static G_LOG_LEVEL: AtomicI32 = AtomicI32::new(1);

fn set_log_level(lvl: i32) {
    G_LOG_LEVEL.store(lvl, Ordering::SeqCst);
}

fn log_msg(lvl: usize, msg: &str) {
    let names = ["DEBUG", "INFO", "WARN", "ERROR"];
    if (lvl as i32) >= G_LOG_LEVEL.load(Ordering::SeqCst) {
        println!("[{}] {}", names[lvl], msg);
    }
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let lvl: i32 = match input.split_whitespace().next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    set_log_level(lvl);
    log_msg(0, "baslangic");
    log_msg(1, "bilgi mesaji");
    log_msg(2, "uyari mesaji");
    log_msg(3, "hata mesaji");
}
