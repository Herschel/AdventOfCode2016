#![feature(conservative_impl_trait)]
extern crate bit_vec;
use bit_vec::BitVec;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let input_row = args.get(1).expect("Need input parameter");
    let num_rows = args.get(2).unwrap().parse::<usize>().unwrap();

    let mut row_a = BitVec::from_fn(input_row.len(), |i| {
        match input_row.as_bytes()[i] {
            b'.' => false,
            b'^' => true,
            _ => panic!("Unexpected character"),
        }
    });

    let mut row_b = BitVec::from_elem(input_row.len(), false);

    let mut num_safe = 0;

    let mut row_index = 0;
    for _ in 0..num_rows {
        if row_index == 0 {
            num_safe += row_a.iter().filter(|bit| *bit == false).count();
            run_cellular_automaton(&row_a, &mut row_b);
        } else {
            num_safe += row_b.iter().filter(|bit| *bit == false).count();
            run_cellular_automaton(&row_b, &mut row_a);
        }

        row_index = 1 - row_index;
    }

    println!("{} safe tiles", num_safe);
}

fn run_cellular_automaton(cur_state: &BitVec, new_state: &mut BitVec) {
    for i in 0..cur_state.len() {
        let is_left_trap = if i > 0 { cur_state[i-1] } else { false };
        let is_center_trap = cur_state.get(i).unwrap_or(false);
        let is_right_trap = cur_state.get(i+1).unwrap_or(false);

        let is_new_trap =
            (is_left_trap && is_center_trap && !is_right_trap) ||
            (!is_left_trap && is_center_trap && is_right_trap) ||
            (is_left_trap && !is_center_trap && !is_right_trap) ||
            (!is_left_trap && !is_center_trap && is_right_trap);

        new_state.set(i, is_new_trap);
    }
}

fn bitvec_to_string(state: &BitVec) -> String {
    state.iter().map(|bit| match bit {
        false => ".",
        true => "^",
    }).collect::<String>()
}