extern crate core;

use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use common::{map_size, Pos, print_map, Size};


fn main() {
    assert_eq!(21, solve1("test1"));
    assert_eq!(1816, solve1("input1"));

    assert_eq!(8, solve2("test1"));
    let solve2 = solve2("input1");
    assert_ne!(1359072, solve2); //   too high
    assert_eq!(383520, solve2);
}

fn solve2(filename: &str) -> u32 {
    let map = parse_map(filename);

    print_map(&map);

    let size = map_size(&map);

    let mut max_score = 0;
    for row in size.min_row..=size.max_row {
        for col in size.min_col..=size.max_col {
            let pos = Pos { row, col };
            let score = scenic_score(&map, &pos, &size);
            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score
}

fn scenic_score(map: &HashMap<Pos, State>, pos: &Pos, size: &Size) -> u32 {
    let left_to_right_view = left_to_right_views(&Size { min_row: pos.row, max_row: pos.row, min_col: pos.col, max_col: size.max_col });
    let right_to_left_view = right_to_left_views(&Size { min_row: pos.row, max_row: pos.row, min_col: size.min_col, max_col: pos.col });
    let top_to_bottom_view = top_to_bottom_views(&Size { min_row: pos.row, max_row: size.max_row, min_col: pos.col, max_col: pos.col });
    let bottom_to_top_view = bottom_to_top_views(&Size { min_row: size.min_row, max_row: pos.row, min_col: pos.col, max_col: pos.col });

    let left_to_right_score = view_score(map, pos, &left_to_right_view[0]);
    let right_to_left_score = view_score(map, pos, &right_to_left_view[0]);
    let top_to_bottom_score = view_score(map, pos, &top_to_bottom_view[0]);
    let bottom_to_top_score = view_score(map, pos, &bottom_to_top_view[0]);

    left_to_right_score * right_to_left_score * top_to_bottom_score * bottom_to_top_score
}

fn view_score(map: &HashMap<Pos, State>, pos: &Pos, view: &Vec<Pos>) -> u32 {
    let self_size = map.get(pos).expect("bad state").size;
    let mut count = 0;
    for next in view {
        if next.eq(pos) {
            continue;
        }
        let next_size = map.get(next).expect("bad state").size;
        if next_size >= self_size {
            return count + 1;
        }
        count += 1;
    }
    count
}

#[derive(Debug, Clone)]
struct State {
    size: u32,
    visible: bool,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}",
               self.size,
               if self.visible { "v" } else { "." })
    }
}

fn solve1(filename: &str) -> u32 {
    let map = parse_map(filename);

    print_map(&map);

    let size = map_size(&map);

    let new_map = scan_map(&map, left_to_right_views(&size), Pos::left);
    let new_map = scan_map(&new_map, right_to_left_views(&size), Pos::right);
    let new_map = scan_map(&new_map, top_to_bottom_views(&size), Pos::top);
    let new_map = scan_map(&new_map, bottom_to_top_views(&size), Pos::bottom);

    print_map(&new_map);

    let mut count = 0;
    for (_, state) in new_map {
        if state.visible {
            count += 1;
        }
    }

    count
}

fn bottom_to_top_views(size: &Size) -> Vec<Vec<Pos>> {
    let mut views = Vec::new();
    for col in size.min_col..=size.max_col {
        let mut view = Vec::new();
        for row in (size.min_row..=size.max_row).rev() {
            view.push(Pos { row, col });
        }
        views.push(view);
    }
    views
}

fn top_to_bottom_views(size: &Size) -> Vec<Vec<Pos>> {
    let mut views = Vec::new();
    for col in size.min_col..=size.max_col {
        let mut view = Vec::new();
        for row in size.min_row..=size.max_row {
            view.push(Pos { row, col });
        }
        views.push(view);
    }
    views
}

fn right_to_left_views(size: &Size) -> Vec<Vec<Pos>> {
    let mut views = Vec::new();
    for row in size.min_row..=size.max_row {
        let mut view = Vec::new();
        for col in (size.min_col..=size.max_col).rev() {
            view.push(Pos { row, col });
        }
        views.push(view);
    }
    views
}

fn left_to_right_views(size: &Size) -> Vec<Vec<Pos>> {
    let mut views = Vec::new();
    for row in size.min_row..=size.max_row {
        let mut view = Vec::new();
        for col in size.min_col..=size.max_col {
            view.push(Pos { row, col });
        }
        views.push(view);
    }
    views
}

fn scan_map(
    map: &HashMap<Pos, State>,
    views: Vec<Vec<Pos>>,
    f_neighbour: fn(&Pos) -> Pos,
) -> HashMap<Pos, State> {
    let mut new_map: HashMap<Pos, State> = map.clone();

    for view in views {
        let mut max: i32 = -1;
        for pos in view {
            if let Some(state) = map.get(&pos) {
                let neighbour_pos = f_neighbour(&pos);
                if state.size as i32 <= max {
                    continue;
                }
                max = state.size as i32;
                let new_state = match map.get(&neighbour_pos) {
                    None => State {
                        size: state.size,
                        visible: true,
                    },
                    Some(neighbour_state) => State {
                        size: state.size,
                        visible: state.visible || neighbour_state.size < state.size,
                    }
                };
                new_map.insert(pos, new_state);
            };
        }
    }

    new_map
}

fn parse_map(filename: &str) -> HashMap<Pos, State> {
    let mut map = HashMap::new();

    let s = fs::read_to_string(filename).expect("bad input");

    let mut row_index = 0;
    for line in s.lines() {
        for col_index in 0..line.len() {
            map.insert(
                Pos { row: row_index, col: col_index as i32 },
                State {
                    size: line[col_index..=col_index].parse().expect("bad data"),
                    visible: false,
                });
        }
        row_index += 1;
    }
    map
}