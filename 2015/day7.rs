use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input/day7.txt")
        .expect("Cannot read file.");

    let mut operands: HashMap<&str, i16>  = HashMap::new();

    for line in input.lines() {
        let line_vec: Vec<_> = line.split(" ").collect();
        
        match line_vec.len() {
            3 => {
                match line_vec[0].parse::<i16>() {
                    Ok(v) => {
                        operands.insert(line_vec[2], v);
                    },
                    Err(_) => {
                        if operands.contains_key(line_vec[0]) {
                            operands.insert(line_vec[2], *operands.get(line_vec[0]).unwrap());
                        }
                    }
                }
            },
            4 => {
                match line_vec[1].parse::<i16>() {
                    // Only Not operation can be performed on single operand
                    Ok(v) => {
                        operands.insert(line_vec[3], v);
                    },
                    Err(_) => {
                        if operands.contains_key(line_vec[1]) {
                            operands.insert(line_vec[3], !operands.get(line_vec[1]).unwrap());
                        }
                    },
                }
            },
            5 => {
                match line_vec[1] {
                    "AND" => { 
                        if operands.contains_key(line_vec[0]) && operands.contains_key(line_vec[2]) {
                            operands.insert(line_vec[4], operands.get(line_vec[0]).unwrap() & operands.get(line_vec[2]).unwrap());
                        }
                    },
                    "OR" => {
                        if operands.contains_key(line_vec[0]) && operands.contains_key(line_vec[2]) {
                            operands.insert(line_vec[4], operands.get(line_vec[0]).unwrap() | operands.get(line_vec[2]).unwrap());
                        }
                    },
                    "RSHIFT" => {
                        if operands.contains_key(line_vec[0]) && operands.contains_key(line_vec[2]) {
                            operands.insert(line_vec[4], operands.get(line_vec[0]).unwrap() >> operands.get(line_vec[2]).unwrap());
                        }
                    },
                    "LSHIFT" => {
                        if operands.contains_key(line_vec[0]) && operands.contains_key(line_vec[2]) {
                            operands.insert(line_vec[4], operands.get(line_vec[0]).unwrap() << operands.get(line_vec[2]).unwrap());
                        }
                    },
                    _ => {},
                }
            },
            _ => {}
        }
    }

    println!("{:?}", operands);
}