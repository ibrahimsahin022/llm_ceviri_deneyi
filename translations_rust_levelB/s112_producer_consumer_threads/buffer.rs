use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

pub const BUFFER_CAPACITY: usize = 4;

pub struct BoundedBuffer {
    inner: Mutex<VecDeque<i32>>,
    not_full: Condvar,
    not_empty: Condvar,
}

impl BoundedBuffer {
    pub fn new() -> Arc<Self> {
        Arc::new(BoundedBuffer {
            inner: Mutex::new(VecDeque::new()),
            not_full: Condvar::new(),
            not_empty: Condvar::new(),
        })
    }

    pub fn put(&self, value: i32) {
        let mut q = self.inner.lock().unwrap();
        while q.len() == BUFFER_CAPACITY {
            q = self.not_full.wait(q).unwrap();
        }
        q.push_back(value);
        self.not_empty.notify_one();
    }

    pub fn get(&self) -> i32 {
        let mut q = self.inner.lock().unwrap();
        while q.is_empty() {
            q = self.not_empty.wait(q).unwrap();
        }
        let v = q.pop_front().unwrap();
        self.not_full.notify_one();
        v
    }
}
