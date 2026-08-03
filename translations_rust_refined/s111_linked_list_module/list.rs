pub struct List {
    head: Option<Box<Node>>,
    size: usize,
}

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl List {
    pub fn new() -> Self {
        List { head: None, size: 0 }
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
        let mut cur = &mut self.head;
        while let Some(node) = cur {
            if node.value == value {
                *cur = node.next.take();
                self.size -= 1;
                return true;
            }
            cur = &mut cur.as_mut().unwrap().next;
        }
        false
    }

    pub fn contains(&self, value: i32) -> bool {
        let mut cur = &self.head;
        while let Some(node) = cur {
            if node.value == value {
                return true;
            }
            cur = &node.next;
        }
        false
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
