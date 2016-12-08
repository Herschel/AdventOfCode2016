use std::io::{self, Read};

type Coord = (i32, i32);
type KeypadDigit = Option<char>;

const KEYPAD: [[KeypadDigit; 5]; 5] = [
    [None, None, Some('1'), None, None],
    [None, Some('2'), Some('3'), Some('4'), None],
    [Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
    [None, Some('A'), Some('B'), Some('C'), None],
    [None, None, Some('D'), None, None]
];
const INITIAL_POS: Coord = (0, 2);
    
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Invalid input");

    let code = input.lines()
        .map(|step| step.chars().map(char_to_dir))
        .scan(INITIAL_POS,
            |pos, steps| {
                *pos = steps.fold(*pos, do_step);
                Some(*pos)
            }
        )
        .map(coord_to_digit)
        .map(|d| d.to_string())
        .collect::<Vec<_>>()
        .concat();

    println!("Code: {}", code);
}

fn do_step(pos: Coord, step: Coord) -> Coord {
    let new_pos = (pos.0 + step.0, pos.1 + step.1);
    if new_pos.0 < 0 || new_pos.0 >= KEYPAD[0].len() as i32 {
        return pos
    }
    if new_pos.1 < 0 || new_pos.1 >= KEYPAD.len() as i32 {
        return pos;
    }
    if KEYPAD[new_pos.1 as usize][new_pos.0 as usize].is_none() {
        return pos;
    }
    new_pos
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

fn coord_to_digit((x, y): Coord) -> char {
    KEYPAD[y as usize][x as usize].unwrap()
}