pub struct Node {
    pub value: i32,
    pub next: Option<Box<Node>>,
}

pub struct List {
    pub head: Option<Box<Node>>,
    pub size: usize,
}

impl List {
    pub fn new() -> Self {
        List {
            head: None,
            size: 0,
        }
    }

    pub fn push_front(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn remove(&mut self, value: i32) -> bool {
        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            if node.value == value {
                *current = node.next.take();
                self.size -= 1;
                return true;
            }
            current = &mut node.next;
        }
        false
    }

    pub fn contains(&self, value: i32) -> bool {
        let mut current = &self.head;
        while let Some(node) = current {
            if node.value == value {
                return true;
            }
            current = &node.next;
        }
        false
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
