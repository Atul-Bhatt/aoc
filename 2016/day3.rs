use std::cmp::max;
use std::cmp::min;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/day3.txt").expect("Cannot read file.");
    let mut count = 0;

    for line in input.lines() {
        let line = line.trim();
        let side_vec: Vec<_> = line.split_whitespace().collect();
        println!("{:?}", side_vec);
        let (x, y, z) = (
            side_vec[0].trim().parse::<i32>().unwrap(),
            side_vec[1].trim().parse::<i32>().unwrap(),
            side_vec[2].trim().parse::<i32>().unwrap(),
        );
        // Find whether sum of two smaller sides is bigger than the biggest side.
        if ((x + y) > z) && ((z + y) > x) && ((x + z) > y) {
            count += 1;
        }
    }
    println!("{count}");
}
