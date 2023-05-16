use md5;

fn main() {
    let mut input = "yzbqklnj".to_string();

    let mut target = 0;

    loop {
        input.push_str(target.to_string().as_str());
        let digest = md5::compute(input.clone());
        
        if format!("{:x}", digest).starts_with("00000") {
            println!("{:x} {input}", digest);
            break;
        }

        target += 1;
    }
    
    println!("{target}");
}