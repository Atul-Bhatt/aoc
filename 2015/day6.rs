use std::fs;

fn main() {
    // Take input from file
    let input = fs::read_to_string("input/day6.txt")
        .expect("Cannot read file");

    let mut grid = vec![[0; 1000]; 1000];

    for line in input.lines() {
        let line_vec: Vec<_> = line.split(" ").collect();
        let first_point: Point;
        let second_point: Point;

        // Find out the instruction type
        match line_vec[0] {
            "turn" => {
                match line_vec[1] {
                    "on" => {
                        let first: Vec<i32> = line_vec[2].split(",").map(|n| n.parse().unwrap()).collect();
                        let second: Vec<i32> = line_vec[4].split(",").map(|n| n.parse().unwrap()).collect();
                        
                        for i in first.0..=second.0 {
                            for j in first.1..=second.1 {
                                grid[i][j] += 1;
                            }
                        }
                    },

                    "off" => {
                        let first: Vec<i32> = line_vec[2].split(",").map(|n| n.parse().unwrap()).collect();
                        let second: Vec<i32> = line_vec[4].split(",").map(|n| n.parse().unwrap()).collect();
                        
                        for i in first.0..=second.0 {
                            for j in first.1..=second.1 {
                                if grid[i][j] > 0 {
                                    grid[i][j] -= 1;
                                } else {
                                    grid[i][j] = 0;
                                }
                            }
                        }
                    },

                    _ => { panic!("Wrong Instruction Type") }
                }
            },

            "toggle" => {
                let first: Vec<i32> = line_vec[1].split(",").map(|n| n.parse().unwrap()).collect();
                        let second: Vec<i32> = line_vec[3].split(",").map(|n| n.parse().unwrap()).collect();
                
                for i in first.0..=second.0 {
                    for j in first.1..=second.1 {
                        grid[i][j] += 2;
                    }
                }
            },

            _ => panic!("Wrong Instruction Type "),
        }

        // println!("{},{} {},{}", first_point.x, first_point.y, second_point.x, second_point.y);
    }
    let mut light_count = 0;

    for i in 0..1000 {
        for j in 0..1000 {
            light_count += grid[i][j];
        }
    }

    println!("{light_count}");
}