use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use regex::{Captures, Regex};
use crate::Op::{Add, Mul};

fn main() {
    let f = |worry_level:u64, _:u64| worry_level / 3;
    assert_eq!(10605, solve1("test1", 20, f));
    assert_eq!(121450, solve1("input1", 20, f));

    let f = |worry_level:u64, multiplier:u64| worry_level % multiplier;
    assert_eq!(2713310158, solve1("test1", 10000, f));
    assert_eq!(28244037010, solve1("input1", 10000, f));
}

fn solve1(filename: &str, rounds: i32, f: fn(u64, u64) -> u64) -> u64 {
    let s = fs::read_to_string(filename).expect("bad input");

    let mut monkeys: Vec<Monkey> = s.split("\n\n").map(parse_monkey).collect();

    let multiplier = monkeys.iter()
            .map(|m| m.divisible_by)
            .fold(1, |a, b| a * b);

    let mut counts: HashMap<usize, u64> = HashMap::new();

    for _ in 0..rounds {

        for mi in 0..monkeys.len() {
            let m = &mut monkeys[mi];

            let mut recipients: HashMap<usize, Vec<u64>> = HashMap::new();

            for item in m.items.iter() {
                let worry_level = f(m.operation(*item), multiplier);

                let throw_to = if worry_level % m.divisible_by == 0 { m.if_true } else { m.if_false };
                if !recipients.contains_key(&throw_to) {
                    recipients.insert(throw_to, Vec::new());
                }

                recipients.get_mut(&throw_to).unwrap().push(worry_level);
            }

            counts.insert(m.id, m.items.len() as u64 + counts.get(&m.id).unwrap_or(&0));

            m.items.clear();
            for (id, items) in recipients {
                for item in items {
                    monkeys[id].items.push(item);
                }
            }
        }

        // print_monkeys(&monkeys);
    }

    println!("{:?}", counts);
    let mut values = counts.into_iter().map(|(_, v)| v).collect::<Vec<u64>>();
    values.sort();
    println!("{:?}", values);

    values.pop().unwrap() * values.pop().unwrap()
}

fn print_monkeys(monkeys: &Vec<Monkey>) {
    monkeys.iter().for_each(|it| println!("{:?}", it));
    println!("---\n");
}

#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    items: Vec<u64>,
    operation: (Option<u64>, Op, Option<u64>),
    divisible_by: u64,
    if_true: usize,
    if_false: usize
}

impl Monkey {
    fn operation(&self, old: u64) -> u64 {
        match self.operation.1 {
            Mul => self.operation.0.unwrap_or(old) * self.operation.2.unwrap_or(old),
            Add => self.operation.0.unwrap_or(old) + self.operation.2.unwrap_or(old)
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Mul,
    Add,
}

fn parse_monkey(s: &str) -> Monkey {
    let lines: Vec<&str> = s.split("\n").collect();
    Monkey {
        id: capture("Monkey (\\d+)", lines[0])[1].trim().parse().expect("bad data"),

        items: capture("Starting items: (\\d.*)", lines[1])[1]
            .split(", ")
            .flat_map(|s| s.parse::<u64>())
            .collect(),

        operation: {
            let captures = capture("Operation: new = (.+) (.+) (.+)", lines[2]);

            let a: Option<u64> = parse_arg(captures[1].as_ref());
            let b: Option<u64> = parse_arg(captures[3].as_ref());

            let op = match captures[2].as_ref() {
                "*" => Mul,
                "+" => Add,
                _ => panic!("bad data")
            };

            (a, op, b)
        },

        divisible_by: capture("Test: divisible by (\\d+)", lines[3])[1].parse().expect("bad data"),

        if_true: capture("If true: throw to monkey (\\d+)", lines[4])[1].parse().expect("bad data"),
        if_false: capture("If false: throw to monkey (\\d+)", lines[5])[1].parse().expect("bad data"),
    }
}

fn parse_arg(arg: &str) -> Option<u64> {
    if arg.eq("old") {
        None
    } else {
        Some(arg.parse::<u64>().expect("bad data"))
    }
}

fn capture<'a>(pattern: &'a str, line: &'a str) -> Captures<'a> {
    Regex::new(pattern)
        .unwrap()
        .captures(line)
        .expect("bad data")
}