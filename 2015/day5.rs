use std::fs;

fn main() {
    let input = fs::read_to_string("input/day5.txt")
        .expect("Cannot read file");

    let mut nice_count = 0;

    for line in input.lines() {
        if contains_double(&line) {
            println!("{line}");
            nice_count += 1;
           }
    }

    println!("{nice_count}");
}

fn contains_vowels(input: &str) -> bool {
    let mut vowel_count = 0;

    for c in input.chars() {
        if "aeiou".contains(c) {
            vowel_count += 1;
        }
    }

    if vowel_count >= 3 {
        return true
    }

    false
}

fn contains_double(input: &str) -> bool {
    let mut occurence_set = std::collections::HashSet::new();
    let mut skip = false;

    for (idx, c) in input.clone().chars().enumerate() {
        if idx == 0 {
            continue;
        }

        if skip == true {
            skip = false;
            continue;
        }

        if c == input.as_bytes()[idx-1] as char {
            if !occurence_set.insert(c) {
                return true;
            }
            skip = true;
        }
    }

    false
}

fn contains_repeat(input: &str) -> bool {
    for (idx, c) in input.clone().chars().enumerate() {
        if idx == 0 || idx == 1 {
            continue;
        }

        if c == input.as_bytes()[idx-2] as char {
            return true;
        }
    }

    false
}

fn contains_bad(input: &str) -> bool {
    if input.contains("ab") || input.contains("cd") || input.contains("pq") || input.contains("xy") {
        return true;
    }
    
    false
} 