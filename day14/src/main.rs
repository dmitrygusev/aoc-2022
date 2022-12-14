use std::collections::HashMap;
use std::fs;
use common::{map_size, Pos, print_map, Size};

static SAND_SOURCE: Pos = Pos { row: 0, col: 500 };

fn main() {
    assert_eq!(24, solve1("test1"));
    assert_eq!(625, solve1("input1"));

    assert_eq!(93, solve2("test1"));
    assert_eq!(25193, solve2("input1"));
}

fn solve2(filename: &str) -> usize {
    let s = fs::read_to_string(filename).unwrap();

    let mut map = parse_map(s);

    let size = &map_size(&map);

    for col in SAND_SOURCE.col - size.max_row - 2..=SAND_SOURCE.col + size.max_row + 2 {
        map.insert(Pos { row: size.max_row + 2, col }, '#');
    }

    let size = &map_size(&map);

    while drop_grain_of_sand(
        &mut map,
        size,
        SAND_SOURCE) {}

    print_map(&map);

    map.values().filter(|v| **v == 'o').count()
}

fn solve1(filename: &str) -> usize {
    let s = fs::read_to_string(filename).unwrap();

    let mut map = parse_map(s);

    let size = &map_size(&map);

    while drop_grain_of_sand(
        &mut map,
        size,
        SAND_SOURCE) {}

    map.values().filter(|v| **v == 'o').count()
}

fn drop_grain_of_sand(map: &mut HashMap<Pos, char>, map_size: &Size, mut sand_pos: Pos) -> bool {
    for row in 1..=map_size.max_row {
        sand_pos.row = row;
        if !map.contains_key(&sand_pos) {
            continue;
        }
        sand_pos.col -= 1;
        if !map.contains_key(&sand_pos) {
            continue;
        }
        sand_pos.col += 2;
        if !map.contains_key(&sand_pos) {
            continue;
        }
        sand_pos.row -= 1;
        sand_pos.col -= 1;
        map.insert(sand_pos, 'o');
        if sand_pos.eq(&SAND_SOURCE) {
            return false;
        }
        return true;
    }
    false
}

fn parse_map(s: String) -> HashMap<Pos, char> {
    let mut map = HashMap::new();

    for line in s.lines() {
        let parts: Vec<Pos> = line.split(" -> ")
            .map(|part| {
                let (col, row) = part.split_once(",").unwrap();
                let col = col.parse().unwrap();
                let row = row.parse().unwrap();
                Pos { row, col }
            })
            .collect();

        parts.windows(2).for_each(|item| {
            let a = item[0];
            let b = item[1];

            for row in i32::min(a.row, b.row)..=i32::max(a.row, b.row) {
                map.insert(Pos { col: a.col, row }, '#');
            }

            for col in i32::min(a.col, b.col)..=i32::max(a.col, b.col) {
                map.insert(Pos { row: a.row, col }, '#');
            }
        });
    }

    map.insert(SAND_SOURCE, '+');

    map
}
