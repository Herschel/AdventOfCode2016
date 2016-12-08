extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Invalid input");
    
    let mut code = vec![' '; 8];

    (0..)
        .map(
            |i| {
                let mut s = input.clone();
                s += &i.to_string();
                s
            }
        )
        .map(md5_string)
        .filter(|s| &s[0..5] == "00000")
        .filter_map(
            |s| {
                let i = s[5..6].parse::<usize>().unwrap_or(code.len());
                if i < code.len() && code[i] == ' ' {
                    code[i] = s.as_bytes()[6] as char;
                    Some(i)
                } else {
                    None
                }
            }
        )
        .take(8)
        .last();

    println!("Password: {}", code.into_iter().collect::<String>());
}

fn md5_string(input: String) -> String {
    let mut md5 = Md5::new();
    md5.input(input.as_bytes());
    let mut md5_out = [0u8; 16];
    md5.result(&mut md5_out);
    byte_slice_to_hex_string(&md5_out)
}

fn byte_slice_to_hex_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}
