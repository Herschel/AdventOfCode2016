extern crate linked_list;
use linked_list::Cursor;
use linked_list::LinkedList;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let mut num_elves = args.get(1).expect("Expected input parameter").parse::<usize>().unwrap();

    let mut elves = (0..num_elves).collect::<LinkedList<usize>>();

    {
        let mut cursor = elves.cursor();
        cursor.seek_forward(num_elves / 2);
        while num_elves > 1 {
            while cursor.peek_next().is_none() {
                cursor.next();
            }
            cursor.remove();

            if num_elves & 1 != 0 {
                while cursor.peek_next().is_none() {
                    cursor.next();
                }
                cursor.next();
            }
            num_elves -= 1;
        }
    }

    if let Some(elf) = elves.front() {
        println!("Elf {} gets all the presents", elf+1);
    } else {
        println!("Something went wrong; no elf has presents");
    }
}