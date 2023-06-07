use std::fs;

enum Direction {
    North,
    West,
    South,
    East,
}

fn main() {
    let input = fs::read_to_string("input/day1.txt")
        .expect("Cannot read file.");

    let mut facing = Direction::North;

    let mut point: (i16, i16) = (0, 0);

    let vec_input: Vec<_> = input.split(",")
        .map(|x| x.trim())
        .collect();

    for direction in vec_input {

        match facing {
            Direction::North => {
                match direction.as_bytes()[0] as char {
                    'L' => {
                        point.0 -= (direction.as_bytes()[1] as char).to_digit(10_u32).unwrap() as i16;
                        facing = Direction::East;
                    },
                    'R' => {
                        point.0 += (direction.as_bytes()[1] as char).to_digit(10_u32).unwrap() as i16;
                        facing = Direction::West;
                    },
                    _ => { }
                }
            },
            Direction::East => {
                match direction.as_bytes()[0] as char {
                    'L' => {
                        point.1 -= (direction.as_bytes()[1] as char).to_digit(10_u32).unwrap() as i16;
                        facing = Direction::South;
                    },
                    'R' => {
                        point.1 += (direction.as_bytes()[1] as char).to_digit(10_u32).unwrap() as i16;
                        facing = Direction::North;
                    },
                    _ => { }
                }
            },
            Direction::South => {
                match direction.as_bytes()[0] as char {
                    'L' => {
                        point.0 += (direction.as_bytes()[1] as char).to_digit(10_u32).unwrap() as i16;
                        facing = Direction::West;
                    },
                    'R' => {
                        point.0 -= (direction.as_bytes()[1] as char).to_digit(10_u32).unwrap() as i16;
                        facing = Direction::East;
                    },
                    _ => { }
                }
            },
            Direction::West => {
                match direction.as_bytes()[0] as char {
                    'L' => {
                        point.1 += (direction.as_bytes()[1] as char).to_digit(10_u32).unwrap() as i16;
                        facing = Direction::North;
                    },
                    'R' => {
                        point.1 -= (direction.as_bytes()[1] as char).to_digit(10_u32).unwrap() as i16;
                        facing = Direction::South;
                    },
                    _ => { }
                }
            }
        }
        println!("{:?}", point);
    }
    println!("{}", point.0.abs() + point.1.abs());
}