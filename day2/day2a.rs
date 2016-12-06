use std::cmp::{min, max};
use std::io::{self, Read};

type Coord = (i32, i32);
const KEYPAD_WIDTH: i32 = 3;
const KEYPAD_HEIGHT: i32 = 3;
const INITIAL_POS: Coord = (1, 1);

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Invalid input");

    let code = input.lines()
        .map(|step| step.chars().map(char_to_dir))
        .scan(INITIAL_POS,
            |pos, steps| {
                *pos = steps.fold(*pos, |pos, step| clamp_pos((pos.0 + step.0, pos.1 + step.1)));
                Some(*pos)
            }
        )
        .map(coord_to_digit)
        .map(|d| d.to_string())
        .collect::<Vec<_>>()
        .concat();

    println!("Code: {}", code);
}

fn clamp_pos((x, y): Coord) -> Coord {
    (min(max(x, 0), KEYPAD_WIDTH - 1),
     min(max(y, 0), KEYPAD_HEIGHT - 1))
}

fn char_to_dir(c: char) -> Coord {
    match c {
        'U' => (0, -1),
        'D' => (0, 1),
        'L' => (-1, 0),
        'R' => (1, 0),
        _ => panic!("Invalid direction {}", c)
    }
}

fn coord_to_digit((x, y): Coord) -> i32 {
    x + y*KEYPAD_WIDTH + 1
}