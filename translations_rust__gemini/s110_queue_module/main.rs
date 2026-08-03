use std::io::{self, Read};

const QUEUE_CAPACITY: usize = 100;

struct Queue {
    data: [i32; QUEUE_CAPACITY],
    front: usize,
    rear: usize,
    count: usize,
}

impl Queue {
    fn new() -> Self {
        Self {
            data: [0; QUEUE_CAPACITY],
            front: 0,
            rear: 0,
            count: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn is_full(&self) -> bool {
        self.count == QUEUE_CAPACITY
    }

    fn enqueue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.data[self.rear] = value;
        self.rear = (self.rear + 1) % QUEUE_CAPACITY;
        self.count += 1;
        true
    }

    fn dequeue(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }
        let out = self.data[self.front];
        self.front = (self.front + 1) % QUEUE_CAPACITY;
        self.count -= 1;
        Some(out)
    }

    fn size(&self) -> usize {
        self.count
    }
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let mut tokens = input.split_whitespace();

    let n: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    let mut q = Queue::new();

    for _ in 0..n {
        let op = match tokens.next() {
            Some(s) => s,
            None => return,
        };

        if op.starts_with('E') {
            let v: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
                Some(val) => val,
                None => return,
            };
            if q.enqueue(v) {
                println!("OK");
            } else {
                println!("FULL");
            }
        } else {
            if let Some(out) = q.dequeue() {
                println!("{}", out);
            } else {
                println!("EMPTY");
            }
        }
    }

    println!("size={}", q.size());
}
