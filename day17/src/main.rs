#![feature(conservative_impl_trait)]
extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;

const MAZE_WIDTH: i32 = 4;
const MAZE_HEIGHT: i32 = 4;

type Position = (i32, i32);

#[derive(Debug,PartialEq,Eq,Clone)]
struct State {
    pos: Position,
    string: String,
}

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
enum Direction {
    North,
    South,
    West,
    East
}


fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let input = args.get(1).expect("Need input parameter").to_string();

    let origin = State {
        pos: (0, 0),
        string: input.clone(),
    };
    let mut states = vec![origin];
    let mut longest_path = (0, None);

    while states.len() > 0 {
        states = states.iter().flat_map(enumerate_moves).collect::<Vec<_>>();

        for state in &states {
            if state.pos == (3,3) {
                let num_steps = state.string.len() - input.len();
                if num_steps > longest_path.0 {
                    longest_path = (num_steps, Some(state.string[input.len()..].to_string()));
                }
            }
        }
    }

    if let (_, Some(path)) = longest_path {
        println!("Path: {}", path);
        println!("Num Steps: {}", path.len());
    } else {
        println!("Path not found");
    }
}

fn enumerate_moves(state: &State) -> Vec<State> {
    if state.pos == (3,3) {
        return vec![];
    }

    let md5_so_far = md5_string(&state.string);
    [Direction::North, Direction::South, Direction::East, Direction::West]
        .into_iter()
        .filter_map(|dir| {
            if let Some(pos) = move_dir(state.pos, *dir) {
                let c = md5_so_far.as_bytes()[*dir as usize];
                if is_door_open(c) {
                    let new_string = state.string.clone() + match *dir {
                        Direction::North => "U",
                        Direction::South => "D",
                        Direction::East => "R",
                        Direction::West => "L",
                    };
                    return Some(State {
                        pos: pos,
                        string: new_string,
                    })
                }
            }
            None
        }).collect::<Vec<_>>()
}

fn is_door_open(c: u8) -> bool {
    match c {
        b'b' | b'c' | b'd' | b'e' | b'f' => true,
        _ => false,
    }
}

fn move_dir((x, y): Position, dir: Direction) -> Option<Position> {
    let new_pos = match dir {
        Direction::North => (x, y - 1),
        Direction::South => (x, y + 1),
        Direction::West => (x - 1, y),
        Direction::East => (x + 1, y),
    };
    if new_pos.0 >= 0 && new_pos.0 < MAZE_WIDTH &&
        new_pos.1 >= 0 && new_pos.1 < MAZE_HEIGHT {
        Some(new_pos)
    } else {
        None
    }
}


fn md5_string(input: &str) -> String {
    let mut md5 = Md5::new();
    md5.input(input.as_bytes());
    let mut md5_out = [0u8; 16];
    md5.result(&mut md5_out);
    byte_slice_to_hex_string(&md5_out)
}

fn byte_slice_to_hex_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}
