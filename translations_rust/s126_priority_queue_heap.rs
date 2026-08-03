use std::io::{self, Read};

struct MinHeap {
    data: Vec<i32>,
}

impl MinHeap {
    fn new() -> Self {
        MinHeap { data: Vec::new() }
    }

    fn sift_up(&mut self, mut i: usize) {
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.data[parent] <= self.data[i] {
                break;
            }
            self.data.swap(parent, i);
            i = parent;
        }
    }

    fn sift_down(&mut self, mut i: usize) {
        loop {
            let left = 2 * i + 1;
            let right = 2 * i + 2;
            let mut smallest = i;
            if left < self.data.len() && self.data[left] < self.data[smallest] {
                smallest = left;
            }
            if right < self.data.len() && self.data[right] < self.data[smallest] {
                smallest = right;
            }
            if smallest == i {
                break;
            }
            self.data.swap(i, smallest);
            i = smallest;
        }
    }

    fn push(&mut self, value: i32) {
        self.data.push(value);
        self.sift_up(self.data.len() - 1);
    }

    fn pop(&mut self) -> Option<i32> {
        if self.data.is_empty() {
            return None;
        }
        let out = self.data[0];
        let last = self.data.pop().unwrap();
        if !self.data.is_empty() {
            self.data[0] = last;
            self.sift_down(0);
        }
        Some(out)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut heap = MinHeap::new();

    for _ in 0..n {
        let op = it.next().unwrap();
        if op == "I" {
            let v: i32 = it.next().unwrap().parse().unwrap();
            heap.push(v);
        } else {
            match heap.pop() {
                Some(v) => println!("{}", v),
                None => println!("EMPTY"),
            }
        }
    }

    println!("size={}", heap.data.len());
}
