use std::fs;

fn main() {
    let input = fs::read_to_string("input/day3.txt").expect("Cannot read file.");
    let mut count = 0;
    let mut r_count = 0;
    let mut x = [[0i32; 3]; 3];

    for line in input.lines() {
        if r_count > 2 {
            r_count = 0;
        }
        let line = line.trim();
        let side_vec: Vec<_> = line.split_whitespace().collect();
        (x[r_count][0], x[r_count][1], x[r_count][2]) = (
            side_vec[0].trim().parse::<i32>().unwrap(),
            side_vec[1].trim().parse::<i32>().unwrap(),
            side_vec[2].trim().parse::<i32>().unwrap(),
        );
        if r_count == 2 {
            for i in 0..3 {
                if ((x[0][i] + x[1][i]) > x[2][i])
                    && ((x[2][i] + x[1][i]) > x[0][i])
                    && ((x[0][i] + x[2][i]) > x[1][i])
                {
                    count += 1;
                }
            }
        }
        r_count += 1;
    }
    println!("{count}");
}
