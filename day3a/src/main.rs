use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Invalid input");

    let num_valid_triangles = input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|nums| nums.map(|num| num.parse::<i32>().expect("Invalid triangle format")))
        .map(|nums| iterator_to_tuple(nums))
        .filter(is_valid_triangle)
        .count();

    println!("{} valid triangles", num_valid_triangles)
}

fn iterator_to_tuple<I, T>(mut n: I) -> (T, T, T)
    where I: Iterator<Item=T> {
    (
        n.next().unwrap(),
        n.next().unwrap(),
        n.next().unwrap()
    )
}

fn is_valid_triangle(&(a, b, c): &(i32, i32, i32)) -> bool {
    return a+b > c && a+c > b && b+c > a;
}