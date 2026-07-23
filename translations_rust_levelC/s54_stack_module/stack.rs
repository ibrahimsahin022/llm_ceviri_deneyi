pub const STACK_CAPACITY: usize = 100;

pub struct Stack {
    data: [i32; STACK_CAPACITY],
    top: i32,
}

impl Stack {
    pub fn new() -> Self {
        Stack { data: [0; STACK_CAPACITY], top: -1 }
    }

    pub fn is_empty(&self) -> bool {
        self.top < 0
    }

    pub fn is_full(&self) -> bool {
        self.top >= (STACK_CAPACITY as i32) - 1
    }

    pub fn push(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.top += 1;
        self.data[self.top as usize] = value;
        true
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }
        let v = self.data[self.top as usize];
        self.top -= 1;
        Some(v)
    }

    pub fn peek(&self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }
        Some(self.data[self.top as usize])
    }

    pub fn size(&self) -> i32 {
        self.top + 1
    }
}
