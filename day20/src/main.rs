extern crate regex;
use regex::Regex;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Invalid input");

    let re = Regex::new(r"(\d+)-(\d+)").unwrap();
    let blacklist_ranges = input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            (
                caps.at(1).unwrap().parse::<u32>().unwrap(),
                caps.at(2).unwrap().parse::<u32>().unwrap()
            )
        })
        .collect::<Vec<_>>();

    let mut count = 0;
    let mut i = 0;
    let mut first_ip = None;
    loop {
        let mut blacklisted = false;
        for &(min, max) in blacklist_ranges.iter() {
            if i >= min && i <= max {
                blacklisted = true;
                i = max;
                break;
            }
        }
        if !blacklisted {
            count += 1;
            if first_ip.is_none() {
                first_ip = Some(i);
            }
        }

        if i == std::u32::MAX {
            break;
        }

        i += 1;
    }

    println!("Num IPs: {}", count);
    if count > 0 {
        println!("First IP: {}", first_ip.unwrap());
    }
}
