use std::sync::RwLock;

pub struct RwState {
    counter: RwLock<i64>,
}

impl RwState {
    pub fn new() -> Self {
        RwState {
            counter: RwLock::new(0),
        }
    }

    pub fn increment(&self) {
        let mut w = self.counter.write().unwrap();
        *w += 1;
    }

    pub fn read(&self) -> i64 {
        let r = self.counter.read().unwrap();
        *r
    }
}
