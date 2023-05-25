use std::fs;

fn main() {
    let input = fs::read_to_string("input/day8.txt")
        .expect("Cannot read file.");

    let mut final_count = 0;

    for line in input.lines() {
        final_count += line.trim().len();
        let mut str_literal_count = line.trim().len() - 2;
        let mut is_slash = false;

        for (idx, c) in line.chars().enumerate() {
            if is_slash {
                is_slash = false;
                continue;
            }
            if c == '\\' {
                match line.as_bytes()[idx+1] as char {
                    '\\' => {
                        is_slash = true;
                        str_literal_count -= 1
                    }, 
                    'x' => str_literal_count -= 3,
                    _ => str_literal_count -= 1,
                }
            }
        }
        final_count -= str_literal_count;
    }
    println!("{final_count}");
}