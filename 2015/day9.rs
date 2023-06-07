use std::fs;

fn main() {
    let input = fs::read_to_string("input/day9.txt")
        .expect("Cannot read file.");


    for line in input.lines() {
        let line_vec = line.split("").collect();

    }
}