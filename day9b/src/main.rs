extern crate regex;
use regex::Regex;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Invalid input");
    let line = input.lines().next().expect("No input").split_whitespace().collect::<Vec<_>>().join("");

    let len = decompressed_len(line.as_ref());
    println!("Decompressed len: {}", len);
}

fn decompressed_len(line: &str) -> usize {

    let mut cur_char = 0;
    let mut len = 0;

    let re = Regex::new(r"\((\d+)x(\d+)\)").unwrap();

    while let Some(cap) = re.captures(&line[cur_char..]) {
        let (mut begin, mut end) = cap.pos(0).unwrap();
        begin += cur_char;
        end += cur_char;
        let num_chars: usize  = cap.at(1).unwrap().parse().unwrap();
        let num_repeats: usize = cap.at(2).unwrap().parse().unwrap();

        len += begin - cur_char;

        let sub_len = decompressed_len(&line[end..end+num_chars]);
        len += sub_len * num_repeats;

        cur_char = end + num_chars;
    }

    len += line.len() - cur_char;
    len
}