mod buffer;

use buffer::BoundedBuffer;
use std::io::{self, Read};
use std::thread;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: i64 = input.trim().parse().unwrap();

    if n < 0 || n > 100000 {
        return;
    }

    let buf = BoundedBuffer::new();

    let prod_buf = buf.clone();
    let producer = thread::spawn(move || {
        for i in 1..=n {
            prod_buf.put(i as i32);
        }
    });

    let cons_buf = buf.clone();
    let consumer = thread::spawn(move || {
        let mut sum: i64 = 0;
        for _ in 0..n {
            sum += cons_buf.get() as i64;
        }
        sum
    });

    producer.join().unwrap();
    let sum = consumer.join().unwrap();

    println!("{}", sum);
}
