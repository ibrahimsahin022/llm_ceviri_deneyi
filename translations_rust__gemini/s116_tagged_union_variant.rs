use std::io::{self, Read};

enum Shape {
    Circle { radius: i32 },
    Rect { width: i32, height: i32 },
}

fn shape_area(s: &Shape) -> i64 {
    match s {
        Shape::Circle { radius } => 3 * (*radius as i64) * (*radius as i64),
        Shape::Rect { width, height } => (*width as i64) * (*height as i64),
    }
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut words = input.split_whitespace();

    let n: i32 = match words.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    let mut total: i64 = 0;
    for _ in 0..n {
        let kind = match words.next() {
            Some(k) => k,
            None => return,
        };

        let shape = if kind.starts_with('C') {
            let r: i32 = match words.next().and_then(|s| s.parse().ok()) {
                Some(val) => val,
                None => return,
            };
            Shape::Circle { radius: r }
        } else {
            let w: i32 = match words.next().and_then(|s| s.parse().ok()) {
                Some(val) => val,
                None => return,
            };
            let h: i32 = match words.next().and_then(|s| s.parse().ok()) {
                Some(val) => val,
                None => return,
            };
            Shape::Rect { width: w, height: h }
        };

        let a = shape_area(&shape);
        println!("{}", a);
        total += a;
    }

    println!("total={}", total);
}
