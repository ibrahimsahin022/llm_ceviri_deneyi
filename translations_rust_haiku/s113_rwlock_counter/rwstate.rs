use std::sync::{Arc, RwLock};

pub struct RwState {
    pub counter: Arc<RwLock<i64>>,
}

impl RwState {
    pub fn new() -> Self {
        RwState {
            counter: Arc::new(RwLock::new(0)),
        }
    }

    pub fn increment(&self) {
        let mut guard = self.counter.write().unwrap();
        *guard += 1;
    }

    pub fn read(&self) -> i64 {
        let guard = self.counter.read().unwrap();
        *guard
    }
}

impl Clone for RwState {
    fn clone(&self) -> Self {
        RwState {
            counter: self.counter.clone(),
        }
    }
}
