use std::collections::HashSet;

fn main() {
    let mut args = std::env::args().skip(1);
    let fav_num = args.next().expect("Need input parameters").parse::<u32>().unwrap();
    let dest = (
        args.next().expect("Need input parameters").parse::<u32>().unwrap(),
        args.next().expect("Need input parameters").parse::<u32>().unwrap()
    );

    let origin = (1u32, 1u32);

    let mut visited = HashSet::new();
    visited.insert(origin);

    let mut positions = vec![origin];
    static DIRECTIONS: [Direction; 4] = [Direction::North, Direction::East, Direction::South, Direction::West];

    let mut num_steps = 0;
    let mut num_steps_to_dest = None;
    while num_steps < 50
    {
        positions = positions.iter()
            .flat_map(|pos| {
                DIRECTIONS
                    .into_iter()
                    .filter_map(|d| move_pos(*pos, *d))
                    .filter(|p| !is_wall(*p, fav_num) && !visited.contains(p))
                    .collect::<Vec<_>>()
                    .into_iter()
            }).collect::<Vec<_>>();

        positions.iter().map(|p| visited.insert(*p)).last();

        num_steps += 1;
        if num_steps_to_dest.is_none() {
            if let Some(_) = positions.iter().find(|&&p| p == dest) {
                num_steps_to_dest = Some(num_steps);
            }
        }
    }

    println!("Num steps to {:?}: {:?}", dest, num_steps_to_dest);
    println!("Num positions after 50 steps: {}", visited.len());
}

fn count_bits(mut n: u32) -> usize {
    let mut num_bits = 0;
    while n != 0 {
        if (n & 0b1) != 0 {
            num_bits += 1;
        }
        n = n >> 1;
    }
    num_bits
}

fn is_wall((x, y): (u32, u32), fav_num: u32) -> bool {
    let n = x*x + 3*x + 2*x*y + y + y*y + fav_num;
    (count_bits(n) & 0b1) != 0
}

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn move_pos((x, y): (u32, u32), dir: Direction) -> Option<(u32, u32)> {
    match dir {
        Direction::North => {
            if y > 0 {
                Some((x, y-1))
            } else {
                None
            }
        },
        Direction::East => Some((x+1, y)),
        Direction::South => Some((x, y+1)),
        Direction::West => {
            if x > 0 {
                Some((x-1, y))
            } else {
                None
            }
        }
    }
}