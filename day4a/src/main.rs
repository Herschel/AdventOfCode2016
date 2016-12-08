extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};

struct Room {
    pub encrypted_name: String,
    pub sector_id: i32,
    pub sorted_char_list: String,
}

impl Room {
    fn is_valid(&self) -> bool {
        let mut char_map: HashMap<char, i32> = HashMap::new();
        for c in self.encrypted_name.chars().filter(|c| *c != '-') {
            *char_map.entry(c).or_insert(1) += 1;
        }
        let mut name_chars = char_map
            .into_iter()
            .map(|(a, b)| (-b, a))
            .collect::<Vec<_>>();
        name_chars.sort();
        name_chars.truncate(5);
        let name_chars = name_chars.into_iter().map(|c| c.1).collect::<Vec<_>>();
        name_chars == self.sorted_char_list.chars().collect::<Vec<char>>()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Invalid input");

    let re = Regex::new(r"([a-z-]+)-(\d+)\[([a-z]+)\]").expect("Invalid regex");
    let sum = re
        .captures_iter(&input)
        .map(|cap| {
            Room {
                encrypted_name: cap.at(1).expect("Malformed room").to_string(),
                sector_id: cap.at(2).expect("Malformed room").to_string().parse::<i32>().expect("Invalid sector ID"),
                sorted_char_list: cap.at(3).expect("Malformed room").to_string(),
            } }
        )
        .filter(Room::is_valid)
        .map(|room| room.sector_id)
        .sum::<i32>();

    println!("Sum of sector IDs: {}", sum)
}