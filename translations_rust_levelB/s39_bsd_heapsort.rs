use std::io::{self, Read};

fn sift_down(arr: &mut [i32], mut root: usize, end: usize) {
    loop {
        let mut child = 2 * root + 1;
        if child >= end {
            break;
        }
        if child + 1 < end && arr[child] < arr[child + 1] {
            child += 1;
        }
        if arr[root] < arr[child] {
            arr.swap(root, child);
            root = child;
        } else {
            break;
        }
    }
}

fn heapsort(arr: &mut [i32]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    for start in (0..n / 2).rev() {
        sift_down(arr, start, n);
    }
    for end in (1..n).rev() {
        arr.swap(0, end);
        sift_down(arr, 0, end);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap_or("0").parse().unwrap_or(0);
    let mut arr: Vec<i32> = (0..n).map(|_| it.next().unwrap().parse().unwrap()).collect();
    heapsort(&mut arr);
    let strs: Vec<String> = arr.iter().map(|x| x.to_string()).collect();
    println!("{}", strs.join(" "));
}
