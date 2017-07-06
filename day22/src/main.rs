#![feature(conservative_impl_trait)]

extern crate regex;
use regex::Regex;
use std::io::{self, Read};
use std::collections::HashSet;


#[derive(Debug,Clone,Hash,PartialEq,Eq)]
struct Grid {
    nodes: Vec<Vec<Node>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            nodes: vec![],
            width: 0,
            height: 0,
        }
    }

    fn add_node(&mut self, x: usize, y: usize, node: Node) {
        if x >= self.nodes.len() {
            self.nodes.resize(x+1, Vec::new());
            self.width = self.nodes.len();
        }

        if y >= self.nodes[x].len() {
            self.nodes[x].resize(y+1, Node::new());
            self.height = self.nodes[x].len();
        }

        self.nodes[x][y] = node;
    }

    fn move_data(&mut self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) {
        let mut from_node = self.nodes[from_x][from_y];
        let mut to_node = self.nodes[to_x][to_y];

        if from_node.used_size > to_node.free_size {
            panic!("OMG we don't have enough free space");
        }

        if from_node.is_magic_data {
            if to_node.used_size == 0 {
                to_node.is_magic_data = true;
                from_node.is_magic_data = false;
            } else {
                panic!("Moving magic data to non-empty disk!");
            }
        }

        to_node.used_size += from_node.used_size;
        to_node.free_size -= from_node.used_size;
        from_node.used_size = 0;
        from_node.free_size = from_node.total_size;

        self.nodes[from_x][from_y] = from_node;
        self.nodes[to_x][to_y] = to_node;
    }

    fn is_victorious(&self) -> bool {
        self.nodes[0][0].is_magic_data
    }

    fn print(&self) {
        println!("{} x {}", self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let node = &self.nodes[x][y];
                if node.is_magic_data {
                    print!("G");
                } else if node.used_size == 0 {
                    print!("_");
                } else if node.used_size > 400 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
    }
}

#[derive(Debug,Clone,Copy,Hash,PartialEq,Eq)]
struct Node {
    total_size: usize,
    used_size: usize,
    free_size: usize,
    is_magic_data: bool,
}

impl Node {
    fn new() -> Node {
        Node {
            total_size: 0,
            used_size: 0,
            free_size: 0,
            is_magic_data: false,
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Invalid input");

    let mut grid = Grid::new();

    let re = Regex::new(r"/dev/grid/(node-x(\d+)-y(\d+))\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+\d+%").unwrap();
    for line in input.lines() {
        if let Some(cap) = re.captures(line) {
            let x = cap.at(2).unwrap().parse::<usize>().unwrap();
            let y = cap.at(3).unwrap().parse::<usize>().unwrap();
            let total_size = cap.at(4).unwrap().parse::<usize>().unwrap();
            let used_size = cap.at(5).unwrap().parse::<usize>().unwrap();
            let free_size = cap.at(6).unwrap().parse::<usize>().unwrap();

            let node = Node {
                total_size: total_size,
                used_size: used_size,
                free_size: free_size,
                is_magic_data: false,
            };

            grid.add_node(x, y, node);
        }
    }

    grid.nodes[grid.width-1][0].is_magic_data = true;
    let magic_data_size = grid.nodes[grid.width-1][0].used_size;

    let mut visited = HashSet::new();

    grid.print();


    visited.insert(grid.clone());
    let mut states = vec![grid];
    
    let mut num_steps = 0;
    while states.len() > 0 && !states.iter().any(Grid::is_victorious) {
        states = states
            .iter()
            .flat_map(|grid| enumerate_next_states(grid))
            .filter(|grid| {
                visited.insert(grid.clone())
            })
            .collect::<Vec<_>>();       
        num_steps += 1;
    }

    println!("Number of steps to move magic data: {}", num_steps);
    
}

fn enumerate_next_states(grid: &Grid) -> impl Iterator<Item=Grid> {

    let mut viable_pairs = Vec::new();
    for x in 0..grid.width {
        for y in 0..grid.height {
            for i in 0..grid.width {
                for j in 0..grid.height {
                    let a = grid.nodes[x][y];
                    let b = grid.nodes[i][j];
                    if a.used_size > 0 && (x != i || y != j) && b.free_size >= a.used_size {
                        if (x == i && (y == j+1 || y+1 == j)) || (y == j && (x == i+1 || x+1 == i)) {
                                viable_pairs.push(((x,y), (i,j)));
                            }
                        }
                    }
                }
            }
        }
    }

    let new_grid = grid.clone();
    viable_pairs.into_iter().map(move |((from_x, from_y), (to_x, to_y))| {
        let mut new_state = new_grid.clone();
        new_state.move_data(from_x, from_y, to_x, to_y);
        new_state
    })
}
