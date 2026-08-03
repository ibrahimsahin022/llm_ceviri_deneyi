use std::io;

#[derive(Clone, Copy)]
enum Shape {
    Circle { radius: i32 },
    Rect { width: i32, height: i32 },
}

impl Shape {
    fn area(&self) -> i64 {
        match self {
            Shape::Circle { radius } => 3i64 * (*radius as i64) * (*radius as i64),
            Shape::Rect { width, height } => (*width as i64) * (*height as i64),
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut total = 0i64;

    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut parts = line.split_whitespace();
        let kind = parts.next().unwrap();

        let shape = if kind == "C" {
            let r: i32 = parts.next().unwrap().parse().unwrap();
            Shape::Circle { radius: r }
        } else {
            let w: i32 = parts.next().unwrap().parse().unwrap();
            let h: i32 = parts.next().unwrap().parse().unwrap();
            Shape::Rect { width: w, height: h }
        };

        let a = shape.area();
        println!("{}", a);
        total += a;
    }

    println!("total={}", total);
}
