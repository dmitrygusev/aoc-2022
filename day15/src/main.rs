use Ordering::Equal;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use common::{map_size, Pos, print_map};
use regex::Regex;

fn main() {
    assert_eq!(26, solve1("test1", 10));
    assert_eq!(5461729, solve1("input1", 2_000_000));

    assert_eq!(56000011, solve2("test1", 0, 20));
    assert_eq!(10621647166538, solve2("input1", 0, 4_000_000));
}

fn solve2(filename: &str, from_row: i32, to_row: i32) -> i64 {
    let (map, sensor_beacon_pairs) = parse_map(filename);

    if map_size(&map).max_col < 100 {
        print_map(&map);
    }

    for target_row in from_row..=to_row {
        let ranges = build_ranges(target_row, &sensor_beacon_pairs);

        let mut col = ranges.first().unwrap().0;
        for (start, end) in &ranges {
            if start > &col {
                return ((*start as i64 - 1) * 4000000) + target_row as i64
            }
            if end > &col {
                col = *end;
            }
        }
    }

    panic!("no solution")
}

fn solve1(filename: &str, target_row: i32) -> i32 {
    let (map, sensor_beacon_pairs) = parse_map(filename);

    if map_size(&map).max_col < 100 {
        print_map(&map);
    }

    let ranges = build_ranges(target_row, &sensor_beacon_pairs);

    println!("{:?}", ranges);

    let mut count = 0;
    let mut col = ranges.first().unwrap().0;
    for (start, end) in &ranges {
        if start > &col {
            panic!("should be no holes");
        }
        if end > &col {
            count += end - &col;
            col = *end;
        }
    }

    count
}

fn build_ranges(target_row: i32, sensor_beacon_pairs: &Vec<(Pos, Pos)>) -> Vec<(i32, i32)> {
    let mut ranges = Vec::new();
    sensor_beacon_pairs.iter().for_each(|(s, b)| {
        let distance = manhattan_distance(s, b);

        if s.row - distance <= target_row && s.row + distance >= target_row {
            capture_ranges(target_row, &mut ranges, s, distance);
        }
    });

    ranges.sort_by(|(s1, e1), (s2, e2)| {
        let ordering = s1.cmp(s2);
        match ordering {
            Equal => e1.cmp(e2),
            _ => ordering
        }
    });
    ranges
}

fn parse_map(filename: &str) -> (HashMap<Pos, char>, Vec<(Pos, Pos)>) {
    let s = fs::read_to_string(filename).unwrap();

    let mut map = HashMap::new();

    let pattern =
        Regex::new("Sensor at x=(-?\\d+), y=(-?\\d+): closest beacon is at x=(-?\\d+), y=(-?\\d+)").unwrap();

    let mut sensor_beacon_pairs = Vec::new();

    s.lines().for_each(|line| {
        if let Some(matcher) = pattern.captures(line) {
            let s = Pos {
                col: matcher[1].parse::<i32>().unwrap(),
                row: matcher[2].parse::<i32>().unwrap(),
            };
            let b = Pos {
                col: matcher[3].parse::<i32>().unwrap(),
                row: matcher[4].parse::<i32>().unwrap(),
            };
            map.insert(s, 'S');
            map.insert(b, 'B');

            sensor_beacon_pairs.push((s, b));
        }
    });
    (map, sensor_beacon_pairs)
}

fn manhattan_distance(a: &Pos, b: &Pos) -> i32 {
    i32::abs(a.col - b.col) + i32::abs(a.row - b.row)
}

fn capture_ranges(target_row: i32, ranges: &mut Vec<(i32, i32)>, s: &Pos, distance: i32) {
    if target_row >= s.row {
        let d = distance - (target_row - s.row);
        let p1 = Pos {
            col: s.col - d,
            row: target_row,
        };
        let p3 = Pos {
            col: s.col + d,
            row: target_row,
        };
        ranges.push((p1.col, p3.col));
    } else {
        //  target_row <= s.row
        let d = distance - (s.row - target_row);
        let p2 = Pos {
            col: s.col - d,
            row: target_row,
        };
        let p4 = Pos {
            col: s.col + d,
            row: target_row,
        };
        ranges.push((p2.col, p4.col));
    }
}

fn try_insert(map: &mut HashMap<Pos, char>, pos: Pos) {
    if map.contains_key(&pos) {
        return;
    }
    map.insert(pos, '#');
}
