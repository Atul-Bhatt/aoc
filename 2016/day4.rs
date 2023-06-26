use std::fs;

fn main() {
    let input = fs::read_to_string("input/day4.txt").expect("Cannot read file.");
    let mut sector_count = 0;

    for line in input.lines() {
        println!("{:?}", line.split("-"));
    }
}
