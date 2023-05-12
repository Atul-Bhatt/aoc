// Calculate total wrapping paper to buy
use std::fs;
use std::cmp::min;

fn main() {
    let input_dims = fs::read_to_string("input/day2.txt")
        .expect("Cannot read file.");
    let mut wrapping_paper_size = 0;
    let mut ribbon_size = 0;

    for dimensions in input_dims.lines() {
        let dims_vec: Vec<&str> = dimensions.splitn(3, "x").collect();

        // parse the &str values i32
        let (l, w, h) = (dims_vec[0].to_string().parse::<i32>().unwrap(),
            dims_vec[1].to_string().parse::<i32>().unwrap(),
            dims_vec[2].to_string().parse::<i32>().unwrap()
        );

        wrapping_paper_size += 2 * l * w;
        wrapping_paper_size += 2 * w * h;
        wrapping_paper_size += 2 * h * l;

        // smallest side
        wrapping_paper_size += min(h*l, min(l*w, w*h));
        
        ribbon_size += l*w*h;
        ribbon_size += min(2*h + 2*l, min(2*l + 2*w, 2*w + 2*h));
    }

    println!("{ribbon_size}");
}