#![feature(slice_patterns)]
use std::io::{self, Read};

const SCREEN_WIDTH: usize = 50;
const SCREEN_HEIGHT: usize = 6;

enum PixelOp {
    Rect { width: usize, height: usize },
    RotateRow { y: usize, amount: usize },
    RotateCol { x: usize, amount: usize },
}

fn main() {
    let mut screen = [[false; SCREEN_HEIGHT]; SCREEN_WIDTH];

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Invalid input");

    let mut temp_row = [false; SCREEN_WIDTH];
    let mut temp_col = [false; SCREEN_HEIGHT];
    for line in input.lines() {
        let op = decode_op(line);
        match op {
            
            PixelOp::Rect { width, height } => {
                for x in 0..width {
                    for y in 0..height {
                        screen[x][y] = true;
                    }
                }
            },

            PixelOp::RotateRow { y, amount } => {
                for x in 0..SCREEN_WIDTH {
                    let x_shift = (x + amount) % SCREEN_WIDTH;
                    temp_row[x_shift] = screen[x][y];
                }
                for x in 0..SCREEN_WIDTH {
                    screen[x][y] = temp_row[x];
                }
            },

            PixelOp::RotateCol { x, amount } => {
                for y in 0..SCREEN_HEIGHT {
                    let y_shift = (y + amount) % SCREEN_HEIGHT;
                    temp_col[y_shift] = screen[x][y];
                }
                for y in 0..SCREEN_HEIGHT {
                    screen[x][y] = temp_col[y];
                }
            },
        }
    }

    // Output screen.

    let mut line = String::with_capacity(SCREEN_WIDTH);
    let mut num_set = 0;
    for y in 0..SCREEN_HEIGHT {
        line.clear();
        for x in 0..SCREEN_WIDTH {
            num_set += screen[x][y] as usize;
            line.push(if screen[x][y] { '#' } else { '.' });
        }
        println!("{}", line);
    }

    println!("Num pixels set: {}", num_set);
}

fn decode_op(line: &str) -> PixelOp {
    let params = line.split_whitespace().collect::<Vec<_>>();
    match &params[..] {

        &["rect", dimensions] => {
            match &dimensions.split("x").collect::<Vec<_>>()[..] {
                &[width, height] => {
                    let w = width.parse::<usize>().expect("Invalid width");
                    let h = height.parse::<usize>().expect("Invalid height");
                    PixelOp::Rect { width: w, height: h }
                },
                _ => panic!("Invalid dimensions"),
            }
        },

        &["rotate", "row", row, "by", amount] => {
            match &row.split("=").collect::<Vec<_>>()[..] {
                &["y", coord] => {
                    let y = coord.parse::<usize>().expect("Invalid row");
                    let n = amount.parse::<usize>().expect("Invalid rotate amount");
                    PixelOp::RotateRow { y: y, amount: n }
                },
                _ => panic!("Invalid rotate"),
            }
        },

        &["rotate", "column", column, "by", amount] => {
            match &column.split("=").collect::<Vec<_>>()[..] {
                &["x", coord] => {
                    let x = coord.parse::<usize>().expect("Invalid column");
                    let n = amount.parse::<usize>().expect("Invalid rotate amount");
                    PixelOp::RotateCol { x: x, amount: n }
                },
                _ => panic!("Invalid rotate"),
            }
        },

        _ => panic!("Invalid op")
    }
}