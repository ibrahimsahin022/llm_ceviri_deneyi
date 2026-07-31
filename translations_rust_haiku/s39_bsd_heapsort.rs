use std::io::{self, BufRead};

fn heapsort<T, F>(arr: &mut [T], mut compar: F)
where
    F: FnMut(&T, &T) -> i32,
{
    let nmemb = arr.len();
    if nmemb <= 1 {
        return;
    }

    let mut l = nmemb / 2 + 1;
    while l > 0 {
        l -= 1;
        sift_down(arr, l, nmemb, &mut compar);
    }

    let mut n = nmemb;
    while n > 1 {
        arr.swap(0, n - 1);
        n -= 1;
        sift_down(arr, 0, n, &mut compar);
    }
}

fn sift_down<T, F>(arr: &mut [T], mut i: usize, n: usize, compar: &mut F)
where
    F: FnMut(&T, &T) -> i32,
{
    while 2 * i + 1 < n {
        let mut child = 2 * i + 1;
        if child + 1 < n && compar(&arr[child], &arr[child + 1]) < 0 {
            child += 1;
        }
        if compar(&arr[child], &arr[i]) <= 0 {
            break;
        }
        arr.swap(i, child);
        i = child;
    }
}

fn int_cmp(a: &i32, b: &i32) -> i32 {
    if a > b {
        1
    } else if a < b {
        -1
    } else {
        0
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines
        .next()
        .and_then(|line| line.ok())
        .and_then(|line| line.trim().parse().ok())
        .unwrap_or(0);

    let mut arr = Vec::with_capacity(n.max(1));
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            if let Ok(val) = line.trim().parse::<i32>() {
                arr.push(val);
            }
        }
    }

    heapsort(&mut arr, int_cmp);

    for (i, &val) in arr.iter().enumerate() {
        print!("{}", val);
        if i + 1 < arr.len() {
            print!(" ");
        }
    }
    println!();
}
