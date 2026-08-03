use std::io::{self, BufRead};

fn my_strsep<'a>(s: &mut Option<&'a str>, sep: &str) -> Option<&'a str> {
    let cur = (*s)?;
    match cur.find(|c| sep.contains(c)) {
        Some(pos) => {
            let tok = &cur[..pos];
            *s = Some(&cur[pos + 1..]);
            Some(tok)
        }
        None => {
            *s = None;
            Some(cur)
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let sep = lines.next().unwrap().unwrap();
    let text = lines.next().unwrap().unwrap();

    let mut rest = Some(text.as_str());
    while let Some(tok) = my_strsep(&mut rest, &sep) {
        println!("{}", tok);
    }
}
