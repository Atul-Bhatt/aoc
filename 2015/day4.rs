use md5;

fn main() {
    let mut input = "yzbqklnj".to_string();

    let mut target = 0;

    loop {
        let mut temp = input.clone();
        temp.push_str(target.to_string().as_str());
        let digest = md5::compute(temp.clone());
        
        if format!("{:x}", digest).starts_with("00000") {
            println!("{:x} {temp}", digest);
            break;
        }

        target += 1;
    }

    target = 0;

    loop {
        let mut temp = input.clone();
        temp.push_str(target.to_string().as_str());
        let digest = md5::compute(temp.clone());
        
        if format!("{:x}", digest).starts_with("000000") {
            println!("{:x} {temp}", digest);
            break;
        }

        target += 1;
    }
    
    println!("{target}");
}