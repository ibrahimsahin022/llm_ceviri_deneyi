mod buffer;

use std::io;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use buffer::BoundedBufferSync;

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).unwrap();
    let n: i32 = n_str.trim().parse().unwrap();

    if n < 0 || n > 100000 {
        return;
    }

    let buf = BoundedBufferSync::new();

    let buf_prod = buf.clone();
    let prod_thread = thread::spawn(move || {
        for i in 1..=n {
            buf_prod.put(i);
        }
    });

    let sum = Arc::new(Mutex::new(0i64));
    let sum_cons = sum.clone();
    let buf_cons = buf.clone();
    let cons_thread = thread::spawn(move || {
        for _ in 0..n {
            let val = buf_cons.get();
            let mut s = sum_cons.lock().unwrap();
            *s += val as i64;
        }
    });

    prod_thread.join().unwrap();
    cons_thread.join().unwrap();

    let total = *sum.lock().unwrap();
    println!("{}", total);
}
