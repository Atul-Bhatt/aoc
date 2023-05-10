// Find out what floor you are on based on paranthesis
use std::fs;

fn main() {
    // input parenthesis as string
    let input_paren = fs::read_to_string("input/day1.txt")
        .expect("Cannot read file.");

    let mut floor = 0;

    for paren in input_paren.chars() {
        if paren == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
    }
    
    println!("{floor}");
}