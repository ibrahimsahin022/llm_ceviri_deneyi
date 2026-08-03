use std::io::{self, Read};

macro_rules! colors {
    ($($name:ident => $str:expr),* $(,)?) => {
        #[allow(dead_code)]
        #[repr(i32)]
        enum Color {
            $($name),*
        }

        static COLOR_NAMES: &[&str] = &[
            $($str),*
        ];
    };
}

colors! {
    RED => "red",
    GREEN => "green",
    BLUE => "blue",
    YELLOW => "yellow",
    BLACK => "black",
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let word = match input.split_whitespace().next() {
        Some(w) => w,
        None => return,
    };

    let word = if word.len() > 31 {
        let mut end = 31;
        while !word.is_char_boundary(end) {
            end -= 1;
        }
        &word[..end]
    } else {
        word
    };

    let mut idx: i32 = -1;
    for (i, &name) in COLOR_NAMES.iter().enumerate() {
        if name == word {
            idx = i as i32;
            break;
        }
    }
    println!("{}", idx);
}
