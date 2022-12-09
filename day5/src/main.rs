use std::{fs, collections::VecDeque};
use regex::Regex;

fn main() {
    // part1("test1");
    // part1("input1");

    part2("test1");
    part2("input1");
}


fn part2(filename: &str) {
    let s = fs::read_to_string(filename).expect("bad input");

    let mut stacks: [VecDeque<char>; 10] = [VecDeque::new(), VecDeque::new(), VecDeque::new(), VecDeque::new(), VecDeque::new(), VecDeque::new(), VecDeque::new(), VecDeque::new(), VecDeque::new(), VecDeque::new()];
    for line in s.lines() {
       if line.contains("[") {
        let chars: Vec<char> = line.chars().collect();
        for i in 0..=8 {
            let j = 1 + i * 4;
            if j >= chars.len() { break; }
            if chars[j] != ' ' { stacks[i].push_front(chars[j]); }
        }
        continue;
       }

       if !line.contains("move") { continue; }

       let pattern = Regex::new("move (?P<n>\\d+) from (?P<from>\\d+) to (?P<to>\\d+)").unwrap();
       if let Some(matcher) = pattern.captures(line) {
        let n = matcher["n"].parse::<usize>().expect("bad data");
        let from = matcher["from"].parse::<usize>().expect("bad data");
        let to = matcher["to"].parse::<usize>().expect("bad data");

        // println!("{}", line);
        let j = stacks[from - 1].len() - n;
        for _i in 0..=n-1 {
            let v = stacks[from - 1].remove(j).expect("bad state");
            stacks[to - 1].push_back(v);
        }
      }
    }

    //  GTHGFJTJF
    //  PJWGFJZSL
    for stack in stacks {
        if let Some(ch) = stack.back() {
            print!("{}", ch);
        }
        else {
            print!("?");
        }
    }
}

fn part1(filename: &str) {
    let s = fs::read_to_string(filename).expect("bad input");

    let mut stacks: [VecDeque<char>; 10] = [VecDeque::new(), VecDeque::new(), VecDeque::new(), VecDeque::new(), VecDeque::new(), VecDeque::new(), VecDeque::new(), VecDeque::new(), VecDeque::new(), VecDeque::new()];
    for line in s.lines() {
       if line.contains("[") {
        let chars: Vec<char> = line.chars().collect();
        for i in 0..=8 {
            let j = 1 + i * 4;
            if j >= chars.len() { break; }
            if chars[j] != ' ' { stacks[i].push_front(chars[j]); }
        }
        continue;
       }

       if !line.contains("move") { continue; }

       let pattern = Regex::new("move (?P<n>\\d+) from (?P<from>\\d+) to (?P<to>\\d+)").unwrap();
       if let Some(matcher) = pattern.captures(line) {
        let n = matcher["n"].parse::<usize>().expect("bad data");
        let from = matcher["from"].parse::<usize>().expect("bad data");
        let to = matcher["to"].parse::<usize>().expect("bad data");

        // println!("{}", line);
        for _i in 1..=n {
            let v = stacks[from - 1].pop_back().expect("bad state");
            stacks[to - 1].push_back(v);
        }
      }
    }

    //  GTHGFJTJF
    //  PJWGFJZSL
    for stack in stacks {
        if let Some(ch) = stack.back() {
            print!("{}", ch);
        }
        else {
            print!("?");
        }
    }
}
