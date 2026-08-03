use std::io;

fn sift_up(heap: &mut Vec<i32>, mut i: usize) {
    while i > 0 {
        let parent = (i - 1) / 2;
        if heap[parent] <= heap[i] {
            break;
        }
        heap.swap(parent, i);
        i = parent;
    }
}

fn sift_down(heap: &mut Vec<i32>, mut i: usize) {
    loop {
        let left = 2 * i + 1;
        let right = 2 * i + 2;
        let mut smallest = i;
        if left < heap.len() && heap[left] < heap[smallest] {
            smallest = left;
        }
        if right < heap.len() && heap[right] < heap[smallest] {
            smallest = right;
        }
        if smallest == i {
            break;
        }
        heap.swap(i, smallest);
        i = smallest;
    }
}

fn heap_push(heap: &mut Vec<i32>, value: i32) {
    heap.push(value);
    sift_up(heap, heap.len() - 1);
}

fn heap_pop(heap: &mut Vec<i32>) -> Option<i32> {
    if heap.is_empty() {
        return None;
    }
    let out = heap[0];
    let last = heap.pop().unwrap();
    if !heap.is_empty() {
        heap[0] = last;
        sift_down(heap, 0);
    }
    Some(out)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let n: usize = input.trim().parse().unwrap_or(0);

    let mut heap: Vec<i32> = vec![];

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).ok();
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        let op = parts[0];
        if op == "I" {
            if parts.len() >= 2 {
                let v: i32 = parts[1].parse().unwrap_or(0);
                heap_push(&mut heap, v);
            }
        } else {
            if let Some(out) = heap_pop(&mut heap) {
                println!("{}", out);
            } else {
                println!("EMPTY");
            }
        }
    }

    println!("size={}", heap.len());
}
