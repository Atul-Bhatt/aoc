// Find out what floor you are on based on paranthesis
use std::fs;

fn main() {
    // input parenthesis as string
    let input_paren = fs::read_to_string("input/day1.txt")
        .expect("Cannot read file.");

    let mut floor = 0;
    let mut first_basement_idx = 0;
    let mut flag = true;

    for (idx, paren) in input_paren.chars().enumerate() {
        if paren == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }

        // first position that causes -1 floor
        if floor == -1 && flag {
            first_basement_idx = idx+1;
            flag = false;
        }
    }
    
    println!("{first_basement_idx}");
}