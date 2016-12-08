use std::collections::HashMap;
use std::io::{self, Read};
use std::iter;

type FrequencyMap = HashMap<char, i32>;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Invalid input");

    let password_len = input.lines().next().expect("Invalid input").len(); 

    // Create frequency list for each character in the password.
    let mut frequency_maps = iter::repeat(HashMap::new()).take(password_len).collect::<Vec<FrequencyMap>>();
    
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            let count = frequency_maps[i].entry(c).or_insert(0);
            *count += 1;
        }
    }

    let mut password = String::with_capacity(password_len);
    for frequency_map in frequency_maps.into_iter() {
        let mut char_list = frequency_map.into_iter().collect::<Vec<_>>();
        char_list.sort_by_key(|&(_, n)| -n);
        if char_list.len() > 0 {
            password.push(char_list[0].0);
        }
    }

    println!("Password: {}", password);
}