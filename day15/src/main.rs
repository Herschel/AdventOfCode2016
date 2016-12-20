extern crate regex;
use std::io::{self, Read};
use regex::Regex;

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
struct Disc {
    disc_num: usize,
    initial_position: usize,
    num_positions: usize,
}

impl Disc {
    fn new(disc_num: usize, initial_position: usize, num_positions: usize) -> Disc {
        Disc {
            disc_num: disc_num,
            initial_position: initial_position,
            num_positions: num_positions,
        }
    }

    fn position_iter(&self) -> Box<Iterator<Item=usize>> {
        Box::new((0..self.num_positions).cycle().skip(self.initial_position))
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Invalid input");

    let mut discs = vec![];

    let re = Regex::new(r"Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+).").unwrap();
    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let disc_num = captures.at(1).unwrap().parse::<usize>().unwrap();
        let num_positions = captures.at(2).unwrap().parse::<usize>().unwrap();
        let initial_position = captures.at(3).unwrap().parse::<usize>().unwrap();
        discs.push(Disc::new(disc_num, initial_position, num_positions));
    }

    let mut disc_states = discs.into_iter()
        .map(|disc| disc.position_iter().skip(disc.disc_num))
        .collect::<Vec<_>>();

    let result = (0..)
        .position(|_| {
            disc_states.iter_mut().map(|disc| disc.next().unwrap()).sum::<usize>() == 0
        });

    if let Some(t) = result {
        println!("Time to drop capsule: {}", t);
    } else {
        println!("No solution");
    }
}