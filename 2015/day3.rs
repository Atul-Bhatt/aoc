use std::fs;

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        return self.x == other.x && self.y == other.y
    }
}

fn main() {
    let input_string = fs::read_to_string("input/day3.txt")
        .expect("Cannot read file.");

    let mut vec_points: Vec<Point> = vec![];
    let mut curr_point = (0, 0);
    let mut curr_point_robo = (0, 0);
    let mut curr_point_santa = (0, 0);
    let mut temp_point: Point;

    for (idx, point) in input_string.chars().enumerate() {

        println!("curr_point: {:?}, robo: {:?}, santa: {:?}", curr_point, curr_point_robo, curr_point_santa);

        if idx % 2 == 0 {
            curr_point = curr_point_robo;
        } else {
            curr_point = curr_point_santa;
        }
        
        match point {
            '>' => {
                    temp_point = Point::new(curr_point.0 + 1, curr_point.1);
                    curr_point = (curr_point.0 + 1, curr_point.1);
                },

            '<' => {
                    temp_point = Point::new(curr_point.0 - 1, curr_point.1);
                    curr_point = (curr_point.0 - 1, curr_point.1);
                },

            '^' => {
                    temp_point = Point::new(curr_point.0, curr_point.1 - 1);
                    curr_point = (curr_point.0, curr_point.1 - 1);
                },

            'v' => {
                    temp_point = Point::new(curr_point.0, curr_point.1 + 1);
                    curr_point = (curr_point.0, curr_point.1 + 1);
                },
            _ => { temp_point = Point::new(0, 0); }
        }

        if !vec_points.contains(&temp_point) {
            vec_points.push(temp_point);
        }

        if idx % 2 == 0 {
            curr_point_robo = curr_point;
        } else {
            curr_point_santa = curr_point;
        }
    }

    println!("{}", vec_points.len());
}