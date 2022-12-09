use std::fs;
use std::collections::HashSet;

fn main() {
    part1("test1");
    part1("input1");
    part2("input1");
}


fn part2(filename: &str) {
    let s = fs::read_to_string(filename).expect("bad input");

    let mut total = 0;

    let mut i = 0;
    let mut groups: [HashSet<char>; 3] = [HashSet::new(), HashSet::new(), HashSet::new()];
    for line in s.lines() {
        groups[i] = line.chars().collect();

        if i == 2 {
            total += groups[0]
                .intersection(&groups[1]).map(|ch| *ch).collect::<HashSet<char>>()
                .intersection(&groups[2])
                .map(|ch| priority(ch))
                .sum::<u32>();

            i = 0;
        } else {
            i += 1;
        }
    }

    println!("{}", total)
}

fn part1(filename: &str) {
    let s = fs::read_to_string(filename).expect("bad input");

    let mut total = 0;

    for line in s.lines() {
        let half = line.len()/2;
        let first: HashSet<char> = line[0..half].chars().collect();
        let second: HashSet<char> = line[half..line.len()].chars().collect();
        
        let shared = first.intersection(&second);

        let mapped: HashSet<u32> = shared.map(|ch| priority(ch)).collect();

        total += mapped.iter().sum::<u32>();
        // println!("{:?}", mapped);
    }

    println!("{}", total)
}

fn priority(ch: &char) -> u32 {
    match ch {
        'A'..='Z' => ch.clone() as u32 - 64 + 26,
        'a'..='z' => ch.clone() as u32 - 96,
        &_ => panic!("bad data")
    }
}
