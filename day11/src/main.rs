extern crate fixedbitset;
extern crate regex;
use fixedbitset::FixedBitSet;
use regex::Regex;
use std::collections::{HashSet, HashMap};
use std::hash::{Hash, Hasher, SipHasher};
use std::io::{self, Read};

type ElementId = usize;

#[derive(Debug,PartialEq,Eq,Clone,Copy,Hash)]
enum Item {
    Microchip(ElementId),
    Generator(ElementId),
}

const MAX_ELEMENTS: usize = 8;

#[derive(Debug,PartialEq,Clone,Eq,Hash)]
struct State {
    elevator_floor: usize,
    floors: Vec<FixedBitSet>,
}

impl State {
    fn new() -> State {
        State {
            elevator_floor: 0,
            floors: vec![FixedBitSet::with_capacity(MAX_ELEMENTS*2)],
        }
    }

    fn add_item_to_floor(&mut self, item: Item, floor: usize) {
        match item {
            Item::Microchip(n) => self.floors[floor].put(n),
            Item::Generator(n) => self.floors[floor].put(n + MAX_ELEMENTS),
        };
    }

    fn is_valid(&self) -> bool {
        for floor in &self.floors {
            let has_generator = (MAX_ELEMENTS..MAX_ELEMENTS*2).any(|i| floor[i]);

            if has_generator {
                if (0..MAX_ELEMENTS).any(|i| floor.contains(i) && !floor.contains(i+MAX_ELEMENTS)) {
                    return false;
                }
            }
        }
        
        true
    }

    fn is_complete(&self) -> bool {
        for floor in self.floors.iter().take(self.floors.len()-1) {
            if floor.as_slice()[..].into_iter().any(|&i| i != 0) {
                return false;
            }
        }

        self.elevator_floor == self.floors.len()-1
    }

    fn enumerate_next_states(&self) -> Vec<State> {
        let mut next_states = Vec::new();

        for i in 0..MAX_ELEMENTS*2 {
            if self.floors[self.elevator_floor].contains(i) {
                if self.elevator_floor + 1 < self.floors.len() {
                    let next_floor = self.elevator_floor + 1;
                    let mut next_state = self.clone();

                    next_state.floors[self.elevator_floor].set(i, false);
                    next_state.floors[next_floor].set(i, true);

                    next_state.elevator_floor = next_floor;
                    if next_state.is_valid() {
                        next_states.push(next_state);
                    }
                }
                if self.elevator_floor > 0 {
                    let next_floor = self.elevator_floor - 1;
                    let mut next_state = self.clone();

                    next_state.floors[self.elevator_floor].set(i, false);
                    next_state.floors[next_floor].set(i, true);

                    next_state.elevator_floor = next_floor;
                    if next_state.is_valid() {
                        next_states.push(next_state);
                    }
                }
                for j in i+1..MAX_ELEMENTS*2 {
                    if self.floors[self.elevator_floor].contains(j) {
                        if self.elevator_floor + 1 < self.floors.len() {
                            let next_floor = self.elevator_floor + 1;
                            let mut next_state = self.clone();

                            next_state.floors[self.elevator_floor].set(i, false);
                            next_state.floors[next_floor].set(i, true);

                            next_state.floors[self.elevator_floor].set(j, false);
                            next_state.floors[next_floor].set(j, true);

                            next_state.elevator_floor = next_floor;
                            if next_state.is_valid() {
                                next_states.push(next_state);
                            }
                        }
                        if self.elevator_floor > 0 {
                            let next_floor = self.elevator_floor - 1;
                            let mut next_state = self.clone();

                            next_state.floors[self.elevator_floor].set(i, false);
                            next_state.floors[next_floor].set(i, true);

                            next_state.floors[self.elevator_floor].set(j, false);
                            next_state.floors[next_floor].set(j, true);

                            next_state.elevator_floor = next_floor;
                            if next_state.is_valid() {
                                next_states.push(next_state);
                            }
                        }
                    }
                }
            }
        }

        next_states
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Invalid input");

    let mut state = State::new();

    let line_re = Regex::new(r"The (\w+) floor contains (nothing relevant|[\w\s-,]+).").unwrap();
    let component_re = Regex::new(r"\s*a (\w+)( generator|-compatible microchip)").unwrap();
    let component_split_re = Regex::new(r", and | and |, ").unwrap();

    let mut element_ids = HashMap::new();

    for line in input.lines() {
        let line_caps = line_re.captures(line).unwrap();
        let floor = nth_string_to_int(line_caps.at(1).unwrap_or(""));
        if state.floors.len() <= floor {
            state.floors.resize(floor+1, FixedBitSet::with_capacity(MAX_ELEMENTS*2));
        }

        match line_caps.at(2) {
            Some("nothing relevant") => (),
            Some(stuff) => {
                for component in component_split_re.split(stuff) {
                    let caps = component_re.captures(component).unwrap();
                    let element_name = caps.at(1).unwrap().to_string();
                    let len = element_ids.len();
                    let element_id = *element_ids.entry(element_name).or_insert_with(|| {
                        len
                    });

                    if caps.at(2).unwrap() == " generator" {
                        state.add_item_to_floor(Item::Generator(element_id), floor);
                    } else {
                        state.add_item_to_floor(Item::Microchip(element_id), floor);
                    }
                }
            }
            _ => panic!(),
        }
    }

    let mut visited = HashSet::new();
    visited.insert(state.clone());

    println!("{:?}\n", state);

    let mut states = vec![state];
    let mut num_steps = 0;
    while states.len() > 0 && !states.iter().any(State::is_complete) {
        states = states.into_iter()
            .flat_map(|s| s.enumerate_next_states().into_iter())
            .filter(|s| visited.insert(s.clone()))
            .collect();

        //states.iter().map(|s| visited.insert(s.get_hash())).last();
        println!("{:?}\n", num_steps);
        println!("{:?}\n", states.len());
        num_steps += 1;
    }

    println!("Complete in {} steps", num_steps);
}

fn nth_string_to_int(s: &str) -> usize {
    match s {
        "first" => 0,
        "second" => 1,
        "third" => 2,
        "fourth" => 3,
        _ => panic!(),
    }
}


/*
fn enumerate_next_states(state: &State) -> Vec<State> {
    let mut next_states = vec![];
    let combos = elevator_combinations(&state.floors[state.elevator_floor]);
    if state.elevator_floor + 1 < state.floors.len() {
        for &(a, b) in &combos {
            let mut new_state = state.clone();
            let new_floor = state.elevator_floor + 1;
            new_state.floors[state.elevator_floor].items.remove(&a);
            new_state.floors[new_floor].items.insert(a);
            if let Some(item) = b {
                new_state.floors[state.elevator_floor].items.remove(&item);
                new_state.floors[new_floor].items.insert(item);
            }
            new_state.elevator_floor = new_floor;
            if new_state.is_valid() {
                next_states.push(new_state);
            }
        }
    }

    if state.elevator_floor > 0 {
        for &(a, b) in &combos {
            let mut new_state = state.clone();
            let new_floor = state.elevator_floor - 1;
            new_state.floors[state.elevator_floor].items.remove(&a);
            new_state.floors[new_floor].items.insert(a);
            if let Some(item) = b {
                new_state.floors[state.elevator_floor].items.remove(&item);
                new_state.floors[new_floor].items.insert(item);
            }
            new_state.elevator_floor = new_floor;
            if new_state.is_valid() {
                next_states.push(new_state);
            }
        }
    }

    next_states
}


fn elevator_combinations(floor: &Floor) -> Vec<(Item, Option<Item>)> {
    let mut combos = vec![];

    for i in &floor.items {
        combos.push((*i, None));
        for j in &floor.items {
            let i_n = match i {
                &Item::Microchip(i) => i,
                &Item::Generator(i) => i + 9000,
            };
            let j_n = match j {
                &Item::Microchip(j) => j,
                &Item::Generator(j) => j + 9000,
            };
            if i != j && i_n <= j_n {
                combos.push((*i, Some(*j)));
            }
        }
    }

    combos
}*/