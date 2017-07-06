#![feature(conservative_impl_trait)]
extern crate bit_vec;
use std::io::{self, Read};
use bit_vec::BitVec;
use std::collections::HashSet;

#[derive(Clone,Debug,PartialEq,Eq)]
struct Maze {
    width: usize,
    height: usize,
    maze: BitVec,
    locations: Vec<Position>,
}

type Position = (usize, usize);

impl Maze {
    fn new(width: usize, height: usize) -> Maze {
        Maze {
            width: width,
            height: height,
            maze: BitVec::from_elem(width*height, false),
            locations: Vec::new(),
        }
    }

    fn set_wall(&mut self, (x, y): Position, is_wall: bool) {
        self.maze.set(x + y*self.width, is_wall);
    }

    fn get_wall(&self, (x, y): Position) -> bool {
        self.maze.get(x + y*self.width).unwrap_or(true)
    }

    fn set_location(&mut self, pos: Position, i: usize) {
        if i >= self.locations.len() {
            self.locations.resize(i+1, (0, 0));
        }

        self.locations[i] = pos;
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(i) = self.locations.iter().position(|&pos| pos == (x, y)) {
                    print!("{}", i);
                } else {

                    print!("{}", if self.get_wall((x, y)) { '#' } else { '.' });
                }
            }
            print!("\n");
        }
    }
}

#[derive(Clone,Debug,PartialEq,Eq,Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

static DIRECTIONS: [Direction; 4] = [Direction::North, Direction::South, Direction::East, Direction::West];

#[derive(Clone,Debug,PartialEq,Eq,Hash)]
struct State {
    robot_pos: Position,
    visited_locations: BitVec,
    // path: Vec<Direction>,
}

impl State {
    fn new(maze: &Maze) -> State {
        let mut visited_locations = BitVec::from_elem(maze.locations.len(), false);
        State {
            robot_pos: maze.locations[0],
            visited_locations: visited_locations,
            // path: vec![],
        }
    }

    fn move_robot(&self, maze: &Maze, dir: Direction) -> Option<State> {
        let (x, y) = self.robot_pos;
        let new_pos = match dir {
            Direction::North => (x, y-1),
            Direction::South => (x, y+1),
            Direction::West => (x-1, y),
            Direction::East => (x+1, y),
        };

        if maze.get_wall(new_pos) {
            return None;
        }

        let mut new_state = self.clone();
        new_state.robot_pos = new_pos;

        if let Some(i) = maze.locations.iter().position(|&pos| pos == new_pos) {
            if i != 0 || new_state.visited_locations.iter().skip(1).all(|b| b) {
                new_state.visited_locations.set(i, true);
            }
        }

        // new_state.path.push(dir);

        Some(new_state)
    }

    fn enumerate_moves<'a>(&'a self, maze: &'a Maze) -> impl Iterator<Item=State> + 'a {
        DIRECTIONS
            .into_iter()
            .filter_map(move |dir| self.move_robot(maze, *dir))
    }

    fn has_visited_all(&self) -> bool {
        self.visited_locations.iter().all(|b| b)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Invalid input");

    let lines = input.lines().collect::<Vec<_>>();
    if lines.len() == 0 {
        panic!("Expected input");
    }

    let width = lines[0].len();
    let height = lines.len();

    let mut maze = Maze::new(width, height);
    let mut y = 0;
    for line in lines {
        let mut x = 0;
        for c in line.chars() {
            match c {
                '.' => (),
                '#' => maze.set_wall((x, y), true),
                '0' ... '9' => maze.set_location((x, y), c.to_digit(10).unwrap() as usize),
                _ => panic!("Unexpected character"),
            }
            x += 1;
        }
        y += 1;
    }

    maze.print();

    
    let initial_state = State::new(&maze);
    let mut visited = HashSet::new();
    visited.insert(initial_state.clone());

    let mut states = vec![initial_state];
    let mut num_steps = 0;
    while states.len() > 0 && !states.iter().any(State::has_visited_all) {
        let new_states = states
            .iter()
            .flat_map(|state| state.enumerate_moves(&maze))
            .filter(|state| visited.insert(state.clone()))
            .collect::<Vec<_>>();
        states = new_states;
        num_steps += 1;
    }

    // println!("Shortest path: {:?}", states.iter().find(|state| state.has_visited_all()).unwrap());
    println!("Shortest number of steps: {}", num_steps);

}