extern crate crypto;
extern crate itertools;
use crypto::digest::Digest;
use crypto::md5::Md5;
use std::io::{self, Read};
use itertools::multipeek;

fn main() {
    let mut args = std::env::args();
    args.next();
    let salt = args.next().expect("Expected MD5 parameter");

    println!("{}", salt);
    let mut keys_iter = multipeek((0..).map(|i| {
            let mut s = salt.to_string();
            s += &i.to_string();
            s = md5_string(&s);

            for _ in (0..2016) {
                s = md5_string(&s);
            }

            (i, s)
        })
        .filter_map(|(i, ref s)| {
            if let Some(c) = find_repeat_char(s, 3) {
                Some((i, c, s.clone()))
            } else {
                None
            }
        }));

    //let keys = keys_iter.take(40).collect::<Vec<_>>();

    let mut keys = vec![];
    while let Some((i, c, ref s)) = keys_iter.next() {
        let mut has_5 = false;

        while let Some(&(j, d, ref s2)) = keys_iter.peek() {
            if j - i > 1000 {
                break;
            }

            if count_consecutive_char(s2, c) >= 5 {
                has_5 = true;
                break;
            }
        }

        if has_5 {
            keys.push((i, s.clone()));

            if keys.len() >= 64 {
                break;
            }
        }
    }

    println!("{:?}", keys);
}

fn md5_string(input: &str) -> String {
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

fn find_repeat_char(s: &str, times: usize) -> Option<u8> {
    let bytes = s.as_bytes();
    let mut max_repeats = 1;
    let mut max_byte = 0;
    for i in 0..bytes.len() {
        let c = bytes[i];
        let mut repeats = 1;
        for j in i+1..bytes.len() {
            if bytes[j] != c {
                break;
            }
            repeats += 1;
        }

        if repeats >= times {
            return Some(c);
        }
    }

    None
}

fn count_consecutive_char(s: &str, c: u8) -> usize {
    let bytes = s.as_bytes();
    let mut max_repeats = 0;
    let mut repeats = 0;
    for i in 0..bytes.len() {
        if bytes[i] == c {
            repeats += 1;
            if repeats >= max_repeats {
                max_repeats = repeats;
            }
        } else {
            repeats = 0;
        }
    }

    max_repeats
}
