extern crate regex;
use regex::Regex;
use std::io::{self, Read};

struct Room {
    pub encrypted_name: String,
    pub sector_id: i32,
    pub sorted_char_list: String,
}

impl Room {
    fn decrypt_name(&self) -> String {
        fn rotate_char(c: char, n: u32) -> char {
            if !c.is_alphabetic() {
                return ' ';
            }

            let mut c_code = c as u32;
            c_code -= 'a' as u32;
            c_code += n;
            c_code %= ('z' as u32) - ('a' as u32) + 1;
            c_code += 'a' as u32;
            std::char::from_u32(c_code).expect("Bad character")
        }

        self.encrypted_name.chars().map(|c| rotate_char(c, self.sector_id as u32)).collect::<String>()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Invalid input");

    let re = Regex::new(r"([a-z-]+)-(\d+)\[([a-z]+)\]").expect("Invalid regex");
    let room = re
        .captures_iter(&input)
        .map(|cap| {
            Room {
                encrypted_name: cap.at(1).expect("Malformed room").to_string(),
                sector_id: cap.at(2).expect("Malformed room").to_string().parse::<i32>().expect("Invalid sector ID"),
                sorted_char_list: cap.at(3).expect("Malformed room").to_string(),
            } }
        )
        .find(|r| r.decrypt_name().find("north").is_some());

    if let Some(r) = room {
        println!("North Pole ID: {}", r.sector_id)
    } else {
        println!("North Pole not found");
    }
}