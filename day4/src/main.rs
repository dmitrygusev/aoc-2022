use std::{fs, ops::RangeInclusive};

#[test]
fn test_part1() {
    assert_eq!(556, part1("input1"));
    assert_eq!(3, part1("test1"));
}

#[test]
fn test_part2() {
    assert_eq!(876, part2("input1"));
}

fn part1(filename: &str) -> u32 {
    let s = fs::read_to_string(filename).expect("bad input");

    let mut count = 0;
    for line in s.lines() {
        let (p1, p2) = line.split_once(",").expect("bad input");
        let r1 = parse_range(p1);
        let r2 = parse_range(p2);

        let r1_contains_r2 = contains_all(&r1, &r2);
        let r2_contains_r1 = contains_all(&r2, &r1);

        if r1_contains_r2 || r2_contains_r1 {
            count += 1;
        }
    }

    count
}

fn part2(filename: &str) -> u32 {
    let s = fs::read_to_string(filename).expect("bad input");

    let mut count = 0;
    for line in s.lines() {
        let (p1, p2) = line.split_once(",").expect("bad input");
        let r1 = parse_range(p1);
        let r2 = parse_range(p2);

        let r1_contains_r2 = contains_any(&r1, &r2);
        let r2_contains_r1 = contains_any(&r2, &r1);

        if r1_contains_r2 || r2_contains_r1 {
            count += 1;
        }
    }

    count
}

fn contains_all(range1: &RangeInclusive<u32>, range2: &RangeInclusive<u32>) -> bool {
    range1.clone().all(|i| range2.contains(&i))
}

fn contains_any(range1: &RangeInclusive<u32>, range2: &RangeInclusive<u32>) -> bool {
    range1.clone().any(|i| range2.contains(&i))
}

#[test]
fn test_parse_range() {
    assert_eq!(1..=5u32, parse_range("1-5"))
}

fn parse_range(pair: &str) -> RangeInclusive<u32> {
    pair.split_once("-")
        .map(|(start, end)| start.parse::<u32>().expect("bad data")..=end.parse::<u32>().expect("bad data"))
        .expect("bad data")
}

#[test]
fn test_range() {
    let mut range = 1..5;
    range.all(|i| i > 0);
    println!("{:?}", range);
}

fn main() {}