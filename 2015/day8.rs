use std::fs;

fn main() {
    let input = fs::read_to_string("input/day8.txt")
        .expect("Cannot read file.");

    let mut final_count: i32 = 0;

    for line in input.lines() {
        final_count += (line.trim().len() + 4) as i32;
        let str_literal_count: i32 = (line.trim().len() + 2) as i32;

        for c in line.chars() {
            match c {
                '"' => final_count += 1,
                '\\' => final_count += 1,
                _ => {}
            }
        }
        final_count -= str_literal_count;
    }
    println!("{final_count}");
}