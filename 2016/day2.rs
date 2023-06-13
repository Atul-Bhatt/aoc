use std::fs;

fn main() {
    let input = fs::read_to_string("input/day2.txt").expect("Cannot read file.");
    let mut code = String::new();
    let mut position = (1, 1);

    for line in input.lines() {
        for ch in line.chars() {
            match ch {
                'L' => {
                    if position.1 > 0 {
                        position.1 -= 1;
                    }
                }
                'R' => {
                    if position.1 < 2 {
                        position.1 += 1;
                    }
                }
                'U' => {
                    if position.0 > 0 {
                        position.0 -= 1;
                    }
                }
                'D' => {
                    if position.0 < 2 {
                        position.0 += 1;
                    }
                }
                _ => {}
            }
        }
        match position {
            (0, 0) => {
                code = format!("{code}{}", "0".to_string());
            }
            (0, 1) => {
                code = format!("{code}{}", "1".to_string());
            }
            (0, 2) => {
                code = format!("{code}{}", "2".to_string());
            }
            (0, 3) => {
                code = format!("{code}{}", "3".to_string());
            }
            (1, 0) => {
                code = format!("{code}{}", "4".to_string());
            }
            (1, 1) => {
                code = format!("{code}{}", "5".to_string());
            }
            (1, 2) => {
                code = format!("{code}{}", "6".to_string());
            }
            (2, 0) => {
                code = format!("{code}{}", "7".to_string());
            }
            (2, 1) => {
                code = format!("{code}{}", "8".to_string());
            }
            (2, 2) => {
                code = format!("{code}{}", "9".to_string());
            }
            _ => {}
        }
        println!("{:?}", position);
    }
    println!("{code}");
}
