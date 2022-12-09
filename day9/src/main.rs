extern crate core;

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;
use common::{Pos, print_map};

fn main() {
    assert_eq!(13, solve1("test1"));
    assert_eq!(6367, solve1("input1"));

    assert_eq!(1, solve2("test1"));
    assert_eq!(2536, solve2("input1"));
}

fn solve1(filename: &str) -> usize {
    let s = fs::read_to_string(filename).expect("bad input");

    let mut knots = Vec::new();
    for _ in 0..2 {
        knots.push(Pos { row: 0, col: 0 });
    }

    solve(s, knots)
}

fn solve2(filename: &str) -> usize {
    let s = fs::read_to_string(filename).expect("bad input");

    let mut knots = Vec::new();
    for _ in 0..10 {
        knots.push(Pos { row: 0, col: 0 });
    }

    solve(s, knots)
}

fn solve(s: String, mut knots: Vec<Pos>) -> usize {
    let mut map: HashMap<Pos, TailVisit> = HashMap::new();

    for line in s.lines() {
        let (direction, count) = line.split_once(" ").expect("bad data");
        let count: u32 = count.parse().expect("bad data");
        match direction {
            "L" => knots = steps(&mut map, count, knots, Pos::left),
            "U" => knots = steps(&mut map, count, knots, Pos::top),
            "R" => knots = steps(&mut map, count, knots, Pos::right),
            "D" => knots = steps(&mut map, count, knots, Pos::bottom),
            _ => panic!("bad data")
        }
    }

    print_map(&map);

    map.len()
}

struct TailVisit {
    visited: bool,
}

impl Display for TailVisit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.visited { "#" } else { "." })
    }
}

fn steps(
    map: &mut HashMap<Pos, TailVisit>,
    count: u32,
    mut knots: Vec<Pos>,
    f_next: fn(&Pos) -> Pos,
) -> Vec<Pos> {
    for _ in 1..=count {
        let new_head = f_next(&knots.get(0).expect("bad state"));
        let mut new_knots = Vec::with_capacity(knots.len());
        new_knots.push(new_head);
        let mut prev_knot = new_head;
        for k in 1..knots.len() {
            let tail = knots.get(k).expect("bad state");
            let new_tail = step(&tail, prev_knot.clone());
            new_knots.push(new_tail);
            prev_knot = new_tail;
        }
        map.insert(prev_knot.clone(), TailVisit { visited: true });
        knots = new_knots;

        // print_map(&map);
    }
    // println!("moved {} times\n\n", count);
    knots
}

fn step(tail: &Pos, new_head: Pos) -> Pos {
    let mut new_tail_row = tail.row;
    let mut new_tail_col = tail.col;
    if (new_head.row - tail.row).abs() +
        (new_head.col - tail.col).abs() > 2 {
        new_tail_row = tail.row + (new_head.row - tail.row).signum();
        new_tail_col = tail.col + (new_head.col - tail.col).signum();
    } else if (new_head.row - tail.row).abs() > 1 {
        new_tail_row = tail.row + (new_head.row - tail.row).signum();
    } else if (new_head.col - tail.col).abs() > 1 {
        new_tail_col = tail.col + (new_head.col - tail.col).signum();
    }
    Pos {
        row: new_tail_row,
        col: new_tail_col,
    }
}

