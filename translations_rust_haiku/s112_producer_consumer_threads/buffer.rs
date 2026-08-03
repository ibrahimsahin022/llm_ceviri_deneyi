use std::sync::{Arc, Mutex, Condvar};

pub const BUFFER_CAPACITY: usize = 4;

pub struct BoundedBuffer {
    pub data: [i32; BUFFER_CAPACITY],
    pub count: usize,
    pub in_idx: usize,
    pub out_idx: usize,
}

pub struct BoundedBufferSync {
    pub inner: Mutex<BoundedBuffer>,
    pub not_full: Condvar,
    pub not_empty: Condvar,
}

impl BoundedBuffer {
    pub fn new() -> Self {
        BoundedBuffer {
            data: [0; BUFFER_CAPACITY],
            count: 0,
            in_idx: 0,
            out_idx: 0,
        }
    }
}

impl BoundedBufferSync {
    pub fn new() -> Arc<Self> {
        Arc::new(BoundedBufferSync {
            inner: Mutex::new(BoundedBuffer::new()),
            not_full: Condvar::new(),
            not_empty: Condvar::new(),
        })
    }

    pub fn put(&self, value: i32) {
        let mut buf = self.inner.lock().unwrap();
        while buf.count == BUFFER_CAPACITY {
            buf = self.not_full.wait(buf).unwrap();
        }
        buf.data[buf.in_idx] = value;
        buf.in_idx = (buf.in_idx + 1) % BUFFER_CAPACITY;
        buf.count += 1;
        self.not_empty.notify_one();
    }

    pub fn get(&self) -> i32 {
        let mut buf = self.inner.lock().unwrap();
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
