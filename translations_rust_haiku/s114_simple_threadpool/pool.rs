use std::sync::{Arc, Mutex};

pub struct TaskPool {
    pub next_task: usize,
    pub n_tasks: usize,
    pub total: i64,
}

pub struct TaskPoolSync {
    inner: Mutex<TaskPool>,
}

impl TaskPool {
    pub fn new(n_tasks: usize) -> Self {
        TaskPool {
            next_task: 0,
            n_tasks,
            total: 0,
        }
    }
}

impl TaskPoolSync {
    pub fn new(n_tasks: usize) -> Arc<Self> {
        Arc::new(TaskPoolSync {
            inner: Mutex::new(TaskPool::new(n_tasks)),
        })
    }

    pub fn next_task(&self) -> Option<usize> {
        let mut pool = self.inner.lock().unwrap();
        if pool.next_task < pool.n_tasks {
            let task_id = pool.next_task;
            pool.next_task += 1;
            Some(task_id)
        } else {
            None
        }
    }

    pub fn add_result(&self, value: i64) {
        let mut pool = self.inner.lock().unwrap();
        pool.total += value;
    }

    pub fn total(&self) -> i64 {
        let pool = self.inner.lock().unwrap();
        pool.total
    }
}
