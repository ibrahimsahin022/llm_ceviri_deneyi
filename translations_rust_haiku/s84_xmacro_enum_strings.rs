use std::io;

fn main() {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return;
    }

    let buf = input.trim();

    let color_names = ["red", "green", "blue", "yellow", "black"];
    let color_count = color_names.len();

    let mut idx = -1;
    for i in 0..color_count {
        if color_names[i] == buf {
            idx = i as i32;
            break;
        }
    }

    println!("{}", idx);
}
