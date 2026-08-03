use std::sync::Mutex;

pub struct TaskPool {
    next_task: Mutex<i32>,
    n_tasks: i32,
    total: Mutex<i64>,
}

impl TaskPool {
    pub fn new(n_tasks: i32) -> Self {
        TaskPool {
            next_task: Mutex::new(0),
            n_tasks,
            total: Mutex::new(0),
        }
    }

    pub fn next_task(&self) -> Option<i32> {
        let mut nt = self.next_task.lock().unwrap();
        if *nt < self.n_tasks {
            let id = *nt;
            *nt += 1;
            Some(id)
        } else {
            None
        }
    }

    pub fn add_result(&self, value: i64) {
        let mut t = self.total.lock().unwrap();
        *t += value;
    }

    pub fn total(&self) -> i64 {
        *self.total.lock().unwrap()
    }
}
