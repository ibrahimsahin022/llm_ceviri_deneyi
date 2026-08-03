pub const QUEUE_CAPACITY: usize = 8;

pub struct Queue {
    pub data: [i32; QUEUE_CAPACITY],
    pub front: usize,
    pub rear: usize,
    pub count: usize,
}

impl Queue {
    pub fn new() -> Self {
        Queue {
            data: [0; QUEUE_CAPACITY],
            front: 0,
            rear: 0,
            count: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn is_full(&self) -> bool {
        self.count == QUEUE_CAPACITY
    }

    pub fn enqueue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.data[self.rear] = value;
        self.rear = (self.rear + 1) % QUEUE_CAPACITY;
        self.count += 1;
        true
    }

    pub fn dequeue(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }
        let value = self.data[self.front];
        self.front = (self.front + 1) % QUEUE_CAPACITY;
        self.count -= 1;
        Some(value)
    }

    pub fn size(&self) -> usize {
        self.count
    }
}
