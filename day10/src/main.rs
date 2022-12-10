use std::collections::HashMap;
use std::fs;
use common::{Marker, Pos, print_map};

fn main() {
    assert_eq!(13140, solve1("test1"));
    assert_eq!(17180, solve1("input1"));
}

fn solve1(filename: &str) -> i32 {
    let s = fs::read_to_string(filename).expect("bad input");

    let mut r = 0;
    let mut x = 1;
    let mut cycle = 1;

    let mut screen: HashMap<Pos, Marker> = HashMap::new();

    for line in s.lines() {
        if line.eq("noop") {
            tick(&mut cycle, x, &mut r, &mut screen);
            continue;
        }

        match line.split_once(" ") {
            Some(("addx", arg1)) => {
                tick(&mut cycle, x, &mut r, &mut screen);
                x += arg1.parse::<i32>().expect("bad data");
                tick(&mut cycle, x, &mut r, &mut screen);
            }

            _ => panic!("bad data")
        };
    }

    print_map(&screen);

    r
}

fn tick(cycle: &mut i32, x: i32, r: &mut i32, screen: &mut HashMap<Pos, Marker>) {
    *cycle += 1;
    // println!("cycle {} = {}, r = {}", cycle, x, r);
    if (*cycle - 20) % 40 == 0 {
        *r += *cycle * x;
    }

    let sprite_min_col = x - 1;
    let sprite_max_col = x + 1;

    let row = (*cycle - 1) / 40;
    let col = (*cycle - 1) % 40;

    if col >= sprite_min_col && col <= sprite_max_col {
        screen.insert(Pos { row, col }, Marker { visited: true });
    }
}