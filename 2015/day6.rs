use std::fs;

enum InstructionType {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn new(x: i16, y: i16) -> Point {
        Point {x, y}
    }
}

fn main() {
    // Take input from file
    let input = fs::read_to_string("input/day6.txt")
        .expect("Cannot read file");

    for line in input.lines() {
        let line_vec: Vec<_> = line.split(" ").collect();
        let instruction: InstructionType;
        let first_point: Point;
        let second_point: Point; 

        // Find out the instruction type
        match line_vec[0] {
            "turn" => {
                match line_vec[1] {
                    "on" => {
                        instruction = InstructionType::TurnOn;
                    },

                    "off" => {
                        instruction = InstructionType::TurnOff;
                    },

                    _ => { panic!("Wrong Instruction Type") }
                }
            },

            "toggle" => {
                instruction = InstructionType::Toggle;
            },

            _ => panic!("Wrong Instruction Type "),
        }

        match instruction {
            InstructionType::TurnOn => {
                let mut first_point_iter = line_vec[2].split(",");
                let mut second_point_iter = line_vec[4].split(",");
                first_point = Point::new(first_point_iter.next().unwrap().parse::<i16>().unwrap(),
                    first_point_iter.next().unwrap().parse::<i16>().unwrap());
                second_point = Point::new(second_point_iter.next().unwrap().parse::<i16>().unwrap(),
                    second_point_iter.next().unwrap().parse::<i16>().unwrap());
            },

            InstructionType::TurnOff => {
                let mut first_point_iter = line_vec[2].split(",");
                let mut second_point_iter = line_vec[4].split(",");
                first_point = Point::new(first_point_iter.next().unwrap().parse::<i16>().unwrap(),
                    first_point_iter.next().unwrap().parse::<i16>().unwrap());
                second_point = Point::new(second_point_iter.next().unwrap().parse::<i16>().unwrap(),
                    second_point_iter.next().unwrap().parse::<i16>().unwrap());
            },

            InstructionType::Toggle => {
                let mut first_point_iter = line_vec[1].split(",");
                let mut second_point_iter = line_vec[3].split(",");
                first_point = Point::new(first_point_iter.next().unwrap().parse::<i16>().unwrap(),
                    first_point_iter.next().unwrap().parse::<i16>().unwrap());
                second_point = Point::new(second_point_iter.next().unwrap().parse::<i16>().unwrap(),
                    second_point_iter.next().unwrap().parse::<i16>().unwrap());
            }
        }
    }
}

