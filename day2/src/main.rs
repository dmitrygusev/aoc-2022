use std::fs;

fn main() {
    let s = fs::read_to_string("input1").expect("bad input");

    let mut score = 0;

    for line in s.lines() {
        let (e, y) = line.split_once(' ').expect("bad input");
        score += score_f(y, e);
    }

    println!("{}", score);

    let mut score = 0;

    for line in s.lines() {
        let (e, y) = line.split_once(' ').expect("bad input");
        score += match y {
            "X" => match e { "A" => score_f("Z", e), "B" => score_f("X", e), "C" => score_f("Y", e), &_ => panic!("bad data") },
            "Y" => match e { "A" => score_f("X", e), "B" => score_f("Y", e), "C" => score_f("Z", e), &_ => panic!("bad data") },
            "Z" => match e { "A" => score_f("Y", e), "B" => score_f("Z", e), "C" => score_f("X", e), &_ => panic!("bad data") },
            &_ => panic!("bad data")
        };
    }

    println!("{}", score);
}

fn score_f(y: &str, e: &str) -> i32 {
    match y {
        "X" => 1 + match e { "A" => 3, "B" => 0, "C" => 6, &_ => panic!("bad data") },
        "Y" => 2 + match e { "A" => 6, "B" => 3, "C" => 0, &_ => panic!("bad data") },
        "Z" => 3 + match e { "A" => 0, "B" => 6, "C" => 3, &_ => panic!("bad data") },
        &_ => panic!("bad data {}", y)
    }
}
