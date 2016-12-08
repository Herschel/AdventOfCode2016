use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut path = String::new();
    io::stdin().read_to_string(&mut path).expect("Invalid input");

    let pos = find_repeat_pos(path).expect("No repeat position found");
    let distance = pos.0.abs() + pos.1.abs();
    println!("Destination: ({}, {})", pos.0, pos.1);
    println!("Manhattan Dinstance: {}", distance);
}

fn find_repeat_pos(path: String) -> Option<(i32, i32)> {
    // (0, -1) is North.
    let mut pos = (0i32, 0i32);
    let mut delta = (0i32, -1i32);
    let mut visited = HashSet::new();
    visited.insert(pos);

    for step in path.split(", ") {
        delta = match &step[0..1] {
            "R" => (-delta.1, delta.0),
            "L" => (delta.1, -delta.0),
            _ => panic!("Unexpected path direction"),
        };

        let num_steps = step[1..].parse::<i32>().expect("Invalid path format");
        for _ in 0..num_steps {
            pos = (pos.0 + delta.0, pos.1 + delta.1);
            if visited.contains(&pos) {
                return Some(pos);
            }
            visited.insert(pos);
        }
    }

    None
}