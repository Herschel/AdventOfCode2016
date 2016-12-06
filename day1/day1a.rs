use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut path = String::new();
    io::stdin().read_to_string(&mut path).expect("Invalid input");
    
    // (0, -1) is North.
    let mut delta = (0i32, -1i32);

    for step in path.split(", ") {
        delta = match &step[0..1] {
            "R" => (-delta.1, delta.0),
            "L" => (delta.1, -delta.0),
            _ => panic!("Unexpected path direction"),
        };

        let num_steps = step[1..].parse::<i32>().expect("Invalid path format");

        (0..num_steps).map(|_| pos = (pos.0 + delta.0, pos.1 + delta.1)).last();
    }

    let distance = pos.0.abs() + pos.1.abs();
    println!("Destination: ({}, {})", pos.0, pos.1);
    println!("Manhattan Dinstance: {}", distance);
}