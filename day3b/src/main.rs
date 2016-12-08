use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Invalid input");

    let num_columns = input
        .lines()
        .map(str::split_whitespace)
        .next()
        .expect("Invalid input")
        .count();

    let numbers = input
        .split_whitespace()
        .map(|num| num.parse::<i32>().expect("Invalid triangle format"));

    let mut num_valid_triangles = 0;
    let mut i = 0;
    let mut j = 0;
    let mut triangles = [0; 9];
    for number in numbers {
        triangles[j*num_columns + i] = number;
        j = j+1;
        if j == num_columns {
            j = 0;
            i = i + 1;

            if i == 3 {
                num_valid_triangles += if is_valid_triangle(&(triangles[0], triangles[1], triangles[2])) { 1 } else { 0 };
                num_valid_triangles += if is_valid_triangle(&(triangles[3], triangles[4], triangles[5])) { 1 } else { 0 };
                num_valid_triangles += if is_valid_triangle(&(triangles[6], triangles[7], triangles[8])) { 1 } else { 0 };
                i = 0;
            }
        }
    }

    println!("{} valid triangles", num_valid_triangles)
}

fn is_valid_triangle(&(a, b, c): &(i32, i32, i32)) -> bool {
    return a+b > c && a+c > b && b+c > a;
}