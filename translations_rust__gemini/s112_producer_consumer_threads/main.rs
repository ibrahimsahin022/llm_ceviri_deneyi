use std::io::{self, Read};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

const BUFFER_CAPACITY: usize = 10;

struct BoundedBuffer {
    data: [i32; BUFFER_CAPACITY],
    count: usize,
    in_idx: usize,
    out_idx: usize,
}

struct SharedBuffer {
    buffer: Mutex<BoundedBuffer>,
    not_full: Condvar,
    not_empty: Condvar,
}

impl SharedBuffer {
    fn new() -> Self {
        SharedBuffer {
            buffer: Mutex::new(BoundedBuffer {
                data: [0; BUFFER_CAPACITY],
                count: 0,
                in_idx: 0,
                out_idx: 0,
            }),
            not_full: Condvar::new(),
            not_empty: Condvar::new(),
        }
    }

    fn put(&self, value: i32) {
        let mut buf = self.buffer.lock().unwrap();
        while buf.count == BUFFER_CAPACITY {
            buf = self.not_full.wait(buf).unwrap();
        }
        buf.data[buf.in_idx] = value;
        buf.in_idx = (buf.in_idx + 1) % BUFFER_CAPACITY;
        buf.count += 1;
        self.not_empty.notify_one();
    }

    fn get(&self) -> i32 {
        let mut buf = self.buffer.lock().unwrap();
        while buf.count == 0 {
            buf = self.not_empty.wait(buf).unwrap();
        }
        let value = buf.data[buf.out_idx];
        buf.out_idx = (buf.out_idx + 1) % BUFFER_CAPACITY;
        buf.count -= 1;
        self.not_full.notify_one();
        value
    }
}

fn read_int() -> Option<i32> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok()?;
    let mut words = input.split_whitespace();
    let word = words.next()?;
    word.parse::<i32>().ok()
}

fn main() {
    let n = match read_int() {
        Some(n) => n,
        None => return,
    };

    if n < 0 || n > 100000 {
        return;
    }

    let buf = Arc::new(SharedBuffer::new());

    let buf_prod = Arc::clone(&buf);
    let prod_thread = thread::spawn(move || {
        for i in 1..=n {
            buf_prod.put(i);
        }
    });

    let buf_cons = Arc::clone(&buf);
    let cons_thread = thread::spawn(move || {
        let mut sum: i64 = 0;
        for _ in 0..n {
            sum += buf_cons.get() as i64;
        }
        sum
    });

    prod_thread.join().unwrap();
    let sum = cons_thread.join().unwrap();

    println!("{}", sum);
}
