use std::io::{self, Read, Write};

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum UrlReject {
    RejectNada,
    RejectCtrl,
    RejectZero,
}

fn hexval(x: u8) -> u8 {
    if x.is_ascii_digit() {
        x - b'0'
    } else if (b'a'..=b'f').contains(&x) {
        x - b'a' + 10
    } else if (b'A'..=b'F').contains(&x) {
        x - b'A' + 10
    } else {
        0
    }
}

#[allow(non_snake_case)]
fn Curl_urldecode(
    mut string: &[u8],
    length: usize,
    ostring: &mut Option<Vec<u8>>,
    olen: Option<&mut usize>,
    ctrl: UrlReject,
) -> i32 {
    let mut alloc = if length != 0 {
        length
    } else {
        string.iter().position(|&b| b == 0).unwrap_or(string.len())
    };

    let mut ns = Vec::with_capacity(alloc + 1);

    while alloc > 0 {
        let mut input = string[0];
        if input == b'%'
            && alloc > 2
            && string[1].is_ascii_hexdigit()
            && string[2].is_ascii_hexdigit()
        {
            input = (hexval(string[1]) << 4) | hexval(string[2]);
            string = &string[3..];
            alloc -= 3;
        } else {
            string = &string[1..];
            alloc -= 1;
        }

        if (ctrl == UrlReject::RejectCtrl && input < 0x20)
            || (ctrl == UrlReject::RejectZero && input == 0)
        {
            return 1;
        }

        ns.push(input);
    }

    let len = ns.len();
    *ostring = Some(ns);

    if let Some(olen_ref) = olen {
        *olen_ref = len;
    }

    0
}

fn main() {
    let mut line = Vec::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = [0u8; 1];

    while line.len() < 511 {
        match handle.read(&mut buf) {
            Ok(1) => {
                line.push(buf[0]);
                if buf[0] == b'\n' {
                    break;
                }
            }
            _ => break,
        }
    }

    if line.is_empty() {
        return;
    }

    if let Some(pos) = line.iter().position(|&b| b == 0 || b == b'\r' || b == b'\n') {
        line.truncate(pos);
    }

    let mut out: Option<Vec<u8>> = None;
    let mut olen: usize = 0;

    if Curl_urldecode(&line, 0, &mut out, Some(&mut olen), UrlReject::RejectNada) == 0 {
        if let Some(out_buf) = out {
            let print_len = out_buf.iter().position(|&b| b == 0).unwrap_or(out_buf.len());
            let stdout = io::stdout();
            let mut out_handle = stdout.lock();
            let _ = out_handle.write_all(&out_buf[..print_len]);
            let _ = out_handle.write_all(b"\n");
        }
    }
}
