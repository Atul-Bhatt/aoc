use std::fs;

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point {x, y}
    }
}

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
                        let mut first_point_iter = line_vec[2].split(",");
                        let mut second_point_iter = line_vec[4].split(",");
                        first_point = Point::new(first_point_iter.next().unwrap().parse::<usize>().unwrap(),
                            first_point_iter.next().unwrap().parse::<usize>().unwrap());
                        second_point = Point::new(second_point_iter.next().unwrap().parse::<usize>().unwrap(),
                            second_point_iter.next().unwrap().parse::<usize>().unwrap());
                        
                        for i in first_point.x..=second_point.x {
                            for j in first_point.y..=second_point.y {
                                grid[i][j] = 1;
                            }
                        }
                    },

                    "off" => {
                        let mut first_point_iter = line_vec[2].split(",");
                        let mut second_point_iter = line_vec[4].split(",");
                        first_point = Point::new(first_point_iter.next().unwrap().parse::<usize>().unwrap(),
                            first_point_iter.next().unwrap().parse::<usize>().unwrap());
                        second_point = Point::new(second_point_iter.next().unwrap().parse::<usize>().unwrap(),
                            second_point_iter.next().unwrap().parse::<usize>().unwrap());

                        for i in first_point.x..=second_point.x {
                            for j in first_point.y..=second_point.y {
                                grid[i][j] = 0;
                            }
                        }
                    },

                    _ => { panic!("Wrong Instruction Type") }
                }
            },

            "toggle" => {
                let mut first_point_iter = line_vec[1].split(",");
                let mut second_point_iter = line_vec[3].split(",");
                first_point = Point::new(first_point_iter.next().unwrap().parse::<usize>().unwrap(),
                    first_point_iter.next().unwrap().parse::<usize>().unwrap());
                second_point = Point::new(second_point_iter.next().unwrap().parse::<usize>().unwrap(),
                    second_point_iter.next().unwrap().parse::<usize>().unwrap());
                
                for i in first_point.x..=second_point.x {
                    for j in first_point.y..=second_point.y {
                        if grid[i][j] == 1 {
                            grid[i][j] = 0;
                        } else  {
                            grid[i][j] = 1;
                        }
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
            if grid[i][j] == 1 {
                light_count += 1;
            }
        }
    }

    println!("{light_count}");
}