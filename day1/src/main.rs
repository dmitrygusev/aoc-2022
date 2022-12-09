use std::fs;

fn main() {
    let s = fs::read_to_string("input1").expect("bad input");
    
    let mut current:u32 = 0;

    let mut vec = Vec::new();

    for line in s.lines() {
        if line.eq("") {
            vec.push(current);
            current = 0;
            continue;
        }
        current += line.parse::<u32>().expect("bad format");
    }

    vec.sort();
    vec.reverse();

    println!("{}", vec[0]);
    println!("{}", vec[0] + vec[1] + vec[2]);
}
