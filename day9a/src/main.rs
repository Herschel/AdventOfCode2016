extern crate regex;
use regex::Regex;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Invalid input");
    let line = input.lines().next().expect("No input").split_whitespace().collect::<Vec<_>>().join("");

    let mut out = String::new();
    let mut cur_char = 0;
    
    let re = Regex::new(r"\((\d+)x(\d+)\)").unwrap();
    while let Some(cap) = re.captures(&line[cur_char..]) {
        let (mut begin, mut end) = cap.pos(0).unwrap();
        begin += cur_char;
        end += cur_char;
        let num_chars: usize  = cap.at(1).unwrap().parse().unwrap();
        let num_repeats: usize = cap.at(2).unwrap().parse().unwrap();
        out += &line[cur_char..begin];
        let repeated = std::iter::repeat(&line[end..end+num_chars]).take(num_repeats).collect::<String>();
        out += repeated.as_ref();
        cur_char = end + num_chars;
    }

    out += &line[cur_char..];

    println!("Result:\n{}", out);
    println!("Decompressed len: {}", out.len());
}