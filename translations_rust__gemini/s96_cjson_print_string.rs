use std::io::{self, Read, Write};

fn print_string_ptr(input: &[u8], output: &mut [u8]) {
    let mut escape_characters = 0;

    for &b in input {
        match b {
            b'"' | b'\\' | 0x08 | 0x0C | b'\n' | b'\r' | b'\t' => {
                escape_characters += 1;
            }
            _ => {
                if b < 32 {
                    escape_characters += 5;
                }
            }
        }
    }

    let output_length = input.len() + escape_characters;

    if escape_characters == 0 {
        output[0] = b'"';
        output[1..1 + input.len()].copy_from_slice(input);
        output[output_length + 1] = b'"';
        output[output_length + 2] = 0;
        return;
    }

    output[0] = b'"';
    let mut out_idx = 1;

    for &b in input {
        if b > 31 && b != b'"' && b != b'\\' {
            output[out_idx] = b;
            out_idx += 1;
        } else {
            output[out_idx] = b'\\';
            out_idx += 1;
            match b {
                b'\\' => {
                    output[out_idx] = b'\\';
                    out_idx += 1;
                }
                b'"' => {
                    output[out_idx] = b'"';
                    out_idx += 1;
                }
                0x08 => {
                    output[out_idx] = b'b';
                    out_idx += 1;
                }
                0x0C => {
                    output[out_idx] = b'f';
                    out_idx += 1;
                }
                b'\n' => {
                    output[out_idx] = b'n';
                    out_idx += 1;
                }
                b'\r' => {
                    output[out_idx] = b'r';
                    out_idx += 1;
                }
                b'\t' => {
                    output[out_idx] = b't';
                    out_idx += 1;
                }
                _ => {
                    let s = format!("u{:04x}", b);
                    output[out_idx..out_idx + 5].copy_from_slice(s.as_bytes());
                    out_idx += 5;
                }
            }
        }
    }
    output[output_length + 1] = b'"';
    output[output_length + 2] = 0;
}

fn main() {
    let mut stdin = io::stdin().lock();
    let mut line = [0u8; 256];
    let mut len = 0;

    while len < 255 {
        let mut buf = [0u8; 1];
        match stdin.read(&mut buf) {
            Ok(1) => {
                line[len] = buf[0];
                len += 1;
                if buf[0] == b'\n' {
                    break;
                }
            }
            _ => break,
        }
    }

    if len == 0 {
        return;
    }

    let mut end = len;
    for i in 0..len {
        if line[i] == b'\r' || line[i] == b'\n' {
            end = i;
            break;
        }
    }

    let mut out = [0u8; 1024];
    print_string_ptr(&line[..end], &mut out);

    let out_len = out.iter().position(|&b| b == 0).unwrap_or(out.len());
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let _ = handle.write_all(&out[..out_len]);
    let _ = handle.write_all(b"\n");
}
