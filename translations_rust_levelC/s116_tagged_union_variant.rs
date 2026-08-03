use std::io::{self, Read};

enum Shape {
    Circle { radius: i64 },
    Rect { width: i64, height: i64 },
}

fn shape_area(s: &Shape) -> i64 {
    match s {
        Shape::Circle { radius } => 3 * radius * radius,
        Shape::Rect { width, height } => width * height,
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut total: i64 = 0;

    for _ in 0..n {
        let kind = it.next().unwrap();
        let shape = if kind == "C" {
            let r: i64 = it.next().unwrap().parse().unwrap();
            Shape::Circle { radius: r }
        } else {
            let w: i64 = it.next().unwrap().parse().unwrap();
            let h: i64 = it.next().unwrap().parse().unwrap();
            Shape::Rect { width: w, height: h }
        };
        let a = shape_area(&shape);
        println!("{}", a);
        total += a;
    }

    println!("total={}", total);
}
