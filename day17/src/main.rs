extern crate core;

use std::collections::HashMap;
use std::ops::{Neg, Range, RangeInclusive};
use common::{map_size, Pos, print_map, Size};

type Figure = HashMap<Pos, char>;

const INPUT: &'static str = ">>>><<<<>>>><>>>><<<>><<<>>><<><<>>><<>>><<<>>>><>><<<<><<<>>><<><<>><>>>><<><<>><<<<>>>><<<<>><><>><<>>><<<>><<<><<<>><<<<><<><>>><>><<<><><<<<>>>><<<>>>><<>><<<<><<<<><<<<>>><>>><>>><>>>><<<>>><>>><<<>><<<>>>><<<>>><<>><<<<>>>><<<>>>><<>>>><<>>>><<>><<<>><>>>><<><<<<>>><>><>><<><<<>>>><<<><>><<<<>><>>>><<>>>><<<><>>><<<<>><<><>>>><<>>>><<<>><<<>>><<><>>><<<>>>><<>>>><><<<<>><>>>><<<<><><<<>>>><>>>><<<><<<><<<<><<>><>><><<<>>>><<<<><<<<><<<<>>>><<<<><<>>><<<><<<>>>><<<>><>>>><<<>>>><><>>><<>>><<<>><>>>><<>>>><>>>><<<<>>>><<<<>>><<<<>>><<<<>>>><<<>>>><<><<>>>><<<>><<<<>>>><<><<<><>>>><<<<><>><<<>>>><<<<><<<><<>>><<<>>>><>>><<<<>>>><<<>>><<>>>><<<<>><<<>>><<<<>>><<><<>><<><<<<>><<><<>>><<>>><<>>>><>><>>><<<<>>><<<>>>><<<><<<<>><<<<>>>><<<>><<<<><<<<>>><<<<>>><<><<>><<<>>><<<<>>><>>>><<><<>>><<<>>><<>>><<<>><<<>><<<>>><<<<>>>><<><<>>><<><<<>>>><<<<>>>><>>>><<<><><>><<<>>><<>><<<><<>>>><>><<<<>>>><<<><<>><><<<<>>><<>>><>><>><<<<>>>><<>>>><<<<>>>><<<<>>>><>><<>>><<>><<><>>><<<<><><<>>><>>>><<<<><<>>>><<<><>><<>>><<<><<>><><<<<>>><>>>><>>>><<<>>>><<>>>><<<><><>><<>>>><<<>>><<<>>>><<><<>><<<<>><<<>>><<<<>><>>>><<>>><>>>><<>>><<<<>>><>>>><>><<<<><<<>>><<<<>><<<<>><<>>><<<>><>>>><>>><<<<>><>><<<><<<>><<>>><>>>><<><<<<>><<<<>><>>><<<<><<<>>><<<>>><<<<>><<<>>><>>>><<<<>>>><<>>>><<><<>>>><<<<>>><<<<>>><<>>><<><<<<>>>><<>><<<>><>><>><<><<><<<><<<<>><<>>><<<<>><<<><<<<><><<><<<<>>><><<<<>><<<>><>>>><<><<<>>>><<<><<<><>>>><<>>><<<<><>>><<<<>><<<<>><<<>>><<<<>><>><<<<><>>><<<>><>><<>>>><<>>>><<<<><<<><<>>>><<<<>><<<<><<<<>>>><<<<>>><<<<>><<<>><<>>><<<<>><<<<>><<>>><<>>>><<<<>>>><<<>>><<>><<<<>>><>><<>><><<>><<<<>>><<<<><>>><<>>><>>><<><<<>>><<<<>>><<<<>><<><<<<>><>>>><>>>><<<<>><>><<<<>>>><<>>>><<><<>>>><<>>><<<><<<>>><<<>>><<<<><<><<<<>><<<>><<<<>>>><<<>>>><<>>><>>>><>>>><<<<>>>><<<>>>><<><<<<>>>><<>><<<<><<<>><<>>>><<<>><>>><>><>>>><<>>><><>><>><<>>><>><<<>>>><<<>><>>><<<>><<<>>><<>><<<<><<><>><>>><<>><>><<>>><>><<<>>>><<>><>><<<<><<<<>><<>><<<><<<<>>><>>><<<<><<>>>><<>><<<<><<>><<<>>>><<<>>><<>>><<<>>>><<><<<>><<>><><<<<>>>><<<<>><<>>><<><<<>>><<>>>><<<>>><<<>>><<>>><<<<>>><<<>>>><<<<>>><<<>>>><><>><>><<<><>><<>>>><<<>><<<<>>><<<><>>><<<<>>><<<<><<<>>>><<<>><<<<>><<<<><<>>><<<<>>><<<>><<><<<><<>>><>>>><>><<<<>><<<<><<<<>><<<<>>><<<>>><<>><<>>><<<><<<><<>>><<>>><<>>>><>>>><>>><<<<>>><<<<>>><<>>><<><<>>>><>><<>>>><>><>>>><<<><>><<>>><>>>><<<>>>><<>><>>>><>>><<><>><>><<>>>><<<>>>><>>><<<<>>><<>>><>>><<<>>>><<<<>>>><<>>><>><<<<><><<>>><<<>>><<>>><<<><>><<>>><<><>>><<>><<><>>><>>><<<><<<<><<<<>><><><<<<>>><<<>>>><<<<>>><<>>><>>><<><>>><>><>><<<<>><<<>><<<<>>>><<>>>><<<<>>><>>>><<<>>><>>>><<<<><<<<>>><><<<<>>><<><<>><<<>>><<><<<<><<<><>>>><>>><><<<>>>><<<<>>><<<<><<>>>><<><<>>><>><<<>>><<<>>><<<<>>>><<>>>><><><<<<>>><><<<><>>><<>><<<>><>>>><<<<>>>><<<><<><<><<<<>><<>>><><<>><<<>>>><<<<>>>><<<>>>><<<>><<<>>><<>><><>>><<<><<<>>>><<<<><>>><>>>><<<<>>><<<>>><<<>>><<>><>><<<>>><><>><<<<>>><<<>>>><<<>>><<<>><<<<>><<<<>>>><<<<>>>><<<>>>><<<<>>><<<<>>>><<<>>><<<>><<<>><>>>><<>><>><<<<>><<<<><><<<><<<<>>><>><<<>>><><<><>><<<>><<<<>><<<<>><>>>><<<><<<<>>><<<>>><<<<><<<>>>><<<<>>>><<>>>><<<>><<<<><>><<>>><<<><<<<>><<>><<<>>>><<>>>><<<><>>><>><<>>><<>>>><<>><<<<>>>><<<>>><<>>><<<<>><><<>>><<<<>>><<>><<<><<<<>><<<<>><<<><<>><<>>><<>><>>>><<<>><<<>>><<<>>><<<><><<><<<<>>>><<>>>><<<<><><<>>><<>><>>><>>>><>><>>><<<<>>><>><>>><<<<><<<><>>>><>>>><<<><<>>>><<<><<<<><<<>>>><<<<><<<>><<>><<<>>><>><<<><<<><<<>>>><<>><<<<><>><><>>>><<>>><><<<>>>><<>>><<<<><<<>>><<<<><<<<>><>>>><<>>><<>>>><<<>>><>><>>><<><><<>>><<<>>><<<>>>><<<>>><<<<>>><<<<><<<<>>>><<<<><>>><<<<>>>><<>><>>><<<><<>><>>>><<<<>>>><><<>><<<>>><>>>><>><<<<>>>><>><<<<>>><<<><<<>><<<><>>><<><<<>>><<<><<<<>>>><<<<>>><>>><><<><>><<>>><<><<<<>><>>>><<<<><>>><<>>>><<<<>><<>><<<><<<<>><<>>>><<>><<<<><<<<>><<<<>>>><<>>><<<<><><<<>>>><<<><<<<><>>><<>><<<<>>><<<<>>>><<<<>>>><<<>>>><><<<<>>>><><<<<>>><<<>>>><<<<>>><<>>>><<<<>>><>><><<<<>>>><<<<>>>><<><<>>><<<>><><<<>>>><<<<>><>>>><<<><<>>><<><<><>>><<>>>><<<>>><<<>>>><>>>><<<<>>>><<>><<>>>><<<>>><<<><<<><<>>><>>>><<>>>><><>><<<><<>><<>>>><>><<<>><<<<>>>><>><>>><<<>>><<<>><<>>>><<<>><><<<><>><<<>>>><<<<>>>><<>>><<>>><>>><><<<>><<<>><<<><<<<>><<>><<<>>>><<<>>>><<<>>><<>>>><<<>>><<<>><<>>><<<>>>><<<>>><><<<<>>><<<>><>>><><<<>>>><><<<>>><<<<>>><<>>>><<>>><<><<<<>><<<<><<>>>><<>>><><<<<><>>><<<<>><<<>><><<<><>>>><<<<>>><><<>>>><<>>><<<>><>>>><>>><<<><<><>><<<>>><>>><<<<>>>><>>><<<>>>><<<>>>><><<<><<<<>><<>>>><<<>><<<>>>><<>>><<<>>><<<<>><<<>>>><>>><<<>><<<<><<<>>>><<><<><<<<>>>><>>><>>><<<>>><>><<><<<<>><<>>>><<<<>>>><<<><<<>><<><<><<>>>><><<>>>><<<>>>><<><<<<>><<>><<<>>>><<>>><<<<>>><<<<>><<><<<>>><<<>><<<<>><<<<>>>><<>>><<<<>>><<>>>><>>><<>><<<><>>>><<>>><<><<>>><<<>>>><<>>>><<<<><<>><<<<>>><<<><<<>><<<<>><<<>><>>>><>>><<>>>><>>>><<<><<>>><<<<>>>><<<>><<<>>>><<><<<>>><<>>><<<>><<>>><>>>><<><<<>><<>>><<>>>><<>><<<<>><>>><>>><>>><<>>><<>>><<<<><<<<>>><<<<><<>>><<<>>><<<<>>>><<<>>><>>><<>>><>><<<<>>>><<<>><<<>>>><><<>><>>><>><>><<>>>><<<<>>><<<<>><<><<>>>><<>><<><<<>>>><<<><<<<>>>><<>>>><>><<>>>><<<>>><>>><<>>>><<>>>><<<<>>><>><<<>>>><<<<>><<>>><>>><<<<>>>><>>>><<<<><<<>>><>>>><>>><<>><<<<>><>><<<><<<<><<>>><<<<>>>><<<<>><>>>><<<>><<<<><<><<>>>><<<<>>><<><<<>>>><<<><>>><<<>>>><<>><<<><><<<><<>>>><>><<<>><<<>>>><<<<><>><<>><<<>>>><<><>>><>>>><<<>><<<>><<<>><><>>><<><<>>>><>>>><><<<>>>><<<>>><>>><<<<>>><<<>>><<>><<><<>><<<<><<<<>>><>>><<>>>><<<<>>><>>>><<<>>>><<<>>><<<<>>><<<>>>><<<>><>><<<<>>><<<>>><<<>>><<<>>>><<<<>>>><<<<>>><<<>>><<<><<<<>>><<>>><<>>>><<<>>><><<<>>><<<>>><<<>><>>>><<<>><<<>><<<<>>><<<<>><>><>>>><>>><<<<><<<>>><<<>>><<>><<<>>>><<<><<>>>><<<<><<>>><<<>>>><<<<>>><><<<<><<>>>><>>>><<>>><><>><>>>><<<<>>><<<<>><<<<><>>><<><<<<><>>>><>>><<><>>><<<>>>><<<<><<<<>>><<<<><<<<>>><<<<>>><<<<>>>><><<<<>>><<<><<<>>>><<<<>>>><<>><<<>><<<><<>>><<>><<><<><<<<>>>><<<<>>><<<<><<<<><<>>><<<<><<<<>>><>><<<>>><<<<>>><<<<><>>>><<<>><<<<><<<><>>>><>>><<<>><>>><<>>>><<>>><<<<>>>><<<<>>><<>><<>>><<<>><<<<><<>><<<>>><<<>><<<>><<><><<<<>>><<>><<><>>>><<<<>>>><<<>>><<<><<>>>><<<>>><<>><>>><<>><<<<>>>><>>><>>>><<<>>>><><>>><<<>><<<<>>>><<<<>><<>>><<<<><<>>>><<<<>><<<<><<><<>>>><<<>>><<<<>>>><<<<><>>>><<<<>><<>><>><<<>><<<<>>><<<>>><>>><>>>><<<>>>><<<<><<<<>>>><<<>>>><<<>>>><<>>><>>><<<>>><<>>>><<<<>>><>>><<>><<<>>>><<<><<>>><<<<>>><<<<>><<<><<<>><<<<>>><<<><<<<>>>><>>>><<>><<<<>>><><<<<>><<<<>><<<>>>><><>>><>>>><<<<>><<<>>><>>><<<>>>><<<>><<<<>>><<><<><<<<>>><>>>><<<>><>><<<<>>>><<<><<<<>>><<<><<>>>><<<<>>><<<<>>>><<<<>>>><><<>>><<>>>><<>>>><<>>>><<<<>><<<>>><>>><<<<><<>><<<><<<>>><>>>><<<>><>><<<>><<>>><<<<>><>><<<<>>><<>>><<<<>>><<<<>><<<>><><<<>><<<<>>>><<<<>>>><<<>>>><<>>>><<<<>>>><<<>>><<>><<<>><<>><<>>>><<<<>>><><<<>>>><<>><>><<<<><>><>>>><<><<<<>><<<<>><>>>><<<>><<<>>>><<<<>>><<<>>><<<<>><<<><<>><<<>>>><<<<><<<>><<<>>>><>><<>><>>>><<>>>><<<<>>><>>>><<>>>><<>><<<>><>><<<<>>>><<<<>>>><<<<>><<<<>>><<<>>><<<>>><<<>>>><<<><>>>><><<<><>><<<<>><>>><<>>><>>>><<<<>>>><<<><>>>><<<>><<<<>>><<><<<<>><<<>><>><<<>><>>>><<<>>><<>><<<>><>><<<><<<<>>>><>>><><<<<>>>><<>><>>><<<><<<>><<<<>>>><<<<>>><<<<>>>><<<<>>>><><>>>><<<<>>><><<<><<<<>><<<<>>>><<<<>><<<<>>><>>>><<<><<<<>><<>><<><><<>>>><<<<><><<>>><>>><<<<>>><<>>>><<><<>><<<<>>>><>>>><<<><<>>>><<<>><<<>>>><>><<>>>><<>>>><<<>>>><>>>><<>>><<>>>><><><<<>><<><><<<<>>><>>>><<><<>>><<<>>><<<>>>><<<>>>><<><>><<>><>>>><<<><<><<<<>>><<<>>>><<<<><<>>>><<<<>>>><<<><<<><<<><>>><<>><>>>><>>><<<>>>><<<>>>><>>><<<><<<<>>>><<>>>><><>>>><<<<><<>>><<<>>><>>>><>><<<>>><<><<>><<<<>>>><<>><<><>><<<>><><<<<><<<>><<<<><>>><<<<>>>><<<><<<<><<<>>><<<>>>><<<>>><<>>>><<<<>>>><<<>><<>>><<<<>>><<<<>><<><>><<>>><>>><<>><<>>>><<>>>><<>>><<<>>><><<>><<<>><<<><<>>>><<><<<>>>><>><<<>>>><<<>>>><<>>><<<<>>>><<>>><<<>>><<<<><<>>>><><>>>><<>>>><<<>>>><<<><<<<>>><<<<>><<><<<>><>>><<>>><<<>>>><<>>>><<<<>>>><<<<>>>><<<<>><<<>>>><<<>>>><<<>><<>>>><<<<><<<<>><<<>>>><<>>><<<<><<>>><<>>>><<<><>>><<<>>><>>><<><<<><<>>><<><>>>><<>>>><<><<><<<><<<>>><<<<>>><<><<<>><<<<><<<<>>>><<<>>><><<<<>><<<<>>>><<<>><<>>>><<<>>><>>><>><<><<>>><<><>><<>>><><<<<>>><<<>>><>>><<<<>><<<<><><<<>>>><>>><><<<>>><<<>>><<><<<>>>><>>>><<<>>>><>>>><<<<>>>><<<>><<<>>><<>><<<>>>><<<<><>><<<>>><<>><<<<>>>><<><><<<>><><><>>>><<<>>><<<>>><<<<>>>><<<<><<><<<<>><<<<>>>><<<<>>><>><<>><>>><<>><<><<>>>><<>><<>><<>><>>>><>><<<<>><<><<<<>>><<<>><<<>>>><<<>>>><<<>><>>><<<>>><>>>><<<<><>><<>>>><>>>><<<>>>><<<<>>><<<>>><>>>><<>>><<<>>>><<>><<<<>>>><>>>><<<>><<<<><<<><<<>>><<<>>>><<<><<<>>><<<<>>>><<><<>><<>><<<<>><>>><<<<><><<>>>><<<<>>><<<<>>>><>>><<<<>>>><<>><>>><<<<>>>><<<><<<><<<<>>>><>>>><<<<>>>><<<<>><<<>><<>>>><>>>><<>>>><<><<<><<><>><<<>>><<><<>><<<<>><<<>>>><<<>>>><<>>>><<>>><<>><<<<>><>>>><>><<<<>>>><<>>><>>><<<>>><<<<>>><<<<>>>><<<>>><<<>><<<<>>>><<<>>><<<<><<><<><<<>><<<<><>>>><<<>><>>>><<<><<<<>><<<><<>>><<<<>>>><<<><<<>><>><>>>><<>><<<<><<<>><<>>><<>>>><<>>><>>>><<<<>>>><<><<<<><<>><<>><>><<<<><>>><<>>>><<><<>>>><><<<><>>><<<><<<<><<<>>>><<<<>>>><<<<>><<<<>>>><<<<><<>><<<>><>><<<<>><><<<<>>><><<<>>>><<>><<<<>><<<>>><><<<>>>><<<<>>>><>>><<<>><<<>>><<<<>>><<>>>><>>><>>>><<<><<<>>><<<>>>><<<<>>>><<><<<<>>><<<>>>><>>><>><<<>>>><>>>><<><>><<<<><<><<<<><<<>>>><>>><>><>>>><<<>>><<<>><<<<><<>>><<<<>>>><<<>>>><><<<<>>><<<<>>><<><<<<>><><<<>><<>><<<<>>><<<>>>><>><<>>>><>><>>><<<>><<<><<<><<<>><>>><<><<>>>><<<>>><>>>><<><<<<>><<<<><<<<>><<>><>>>><><<>>>><>><>><<<<>><>>>><<<<>><<<>>><<>><<<>>>><<<><<<>><>><<<<><<><<>>><<<<>><<<<>><<<<>>>><<<<>>>><<<<>>>><<>>>><<>>>><<>>><<<<><<<<>>><>><><<>><<>><<<<>>><<<><<>>>><<<>><<>>><<<<><<<<>>><<<>><<<<>>>><<<<>><<>>>><<<<>>>><<>>>><<>>>><>>>><<>><>>><<<<>>><<><<>><<<>>>><<<>><<<<>><<<>><<<<>>>><<<<>>><<<<>>>><<<<>>>><<<>><<>><<>><<<>>><<<>>>><><>>>><<<<>>><>><<<<>><<<>><<<<>><>><<<<>>>><>><<>>><<<>>>><<<>><<<<>>><<<<>>>><<<>>><<<<>>><<<<>>><<>>><<<<><<>>>><>><>>><<<><<<><<<<>>>><<>><<<><<<<>><<>>>><<>><<<<>>><<<<>><<>><<>>><>>><<><<<><<<>>>><><<<>><<>><><<<>><<<<>>><<<>><<<<>><<>>>><>><<<>>>><><<<<>>>><<<<>>>><<<>>>><<<<><<<>>><<>><<>>><<>><<<>>><<<>>><<><<<>>>><<<<>><<<>><<<><<>>>><>>>><<<><<>>>><>><<<<>>>><>>>><>><>>>><<<<>><<<>><<>>>><<<<>>><<<<>><<<<>>>>";

const TEST: &'static str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

fn main() {
    let figures = figures();
    for figure in figures.iter() {
        print_map(&figure)
    }

    assert_eq!(3068, solve1(&figures, TEST));
    assert_eq!(3224, solve1(&figures, INPUT));

    assert_eq!(3068, solve2(&figures, 2022, TEST, 2000));
    assert_eq!(1514285714288, solve2(&figures, 1000000000000, TEST, 2000));

    assert_eq!(1595988538691, solve2(&figures, 1000000000000, INPUT, 10000));
}

fn solve2(figures: &Vec<Figure>, total_figures: u64, instr: &str, probe_size: usize) -> u64 {
    let (field, row_details) = play(&figures, instr, probe_size);
    let size = map_size(&field);
    let rows_in_period = find_period(&field, 10, size.min_row / 2);
    println!("rows_in_period={}", rows_in_period);

    let mut rows_in_offset = 0;
    loop {
        let range1 = (-rows_in_period - rows_in_offset)..(rows_in_offset);
        let rows1 = select_rows(&field, range1);
        let range2 = (-2 * rows_in_period - rows_in_offset)..(-rows_in_period - rows_in_offset);
        if range2.start < size.min_row {
            panic!("no offset");
        }
        let rows2 = select_rows(&field, range2);
        if same_keys(rows_in_period, &rows2, &rows1) {
            println!("offset={}", rows_in_offset);
            break;
        }
        rows_in_offset += 1;
    }

    let row_details_at_offset = row_details.get(&-rows_in_offset).unwrap();

    println!("-offset={}, row_details_at_offset={:?}", -rows_in_offset, row_details_at_offset);
    let row_details_at_offset_and_period = row_details.get(&(-rows_in_period - rows_in_offset)).unwrap();
    println!("-rows_in_period-offset={}, row_details_at_offset_and_period={:?}", -rows_in_period - rows_in_offset, row_details_at_offset_and_period);

    let offset_in_figures = row_details_at_offset.figures_dropped as u64;
    let period_in_figures = row_details_at_offset_and_period.figures_dropped - row_details_at_offset.figures_dropped;
    let moves_without_offset = total_figures - offset_in_figures;
    let full_periods = moves_without_offset / period_in_figures as u64;
    let moves_after_full_periods = total_figures - offset_in_figures - period_in_figures as u64 * full_periods;

    println!("rows_in_period={}", rows_in_period);
    println!("rows_in_offset={}, details={:?}", rows_in_offset, row_details_at_offset);
    println!("period_in_figures={}", period_in_figures);
    println!("full_periods={}", full_periods);
    println!("rows in full_periods={}", full_periods * rows_in_period as u64);

    let (mut field, mut row_details) = play(&figures, instr, (offset_in_figures + period_in_figures as u64) as usize);
    print_map(&field);

    let size_before_continuation = map_size(&field);
    let continuation = row_details_at_offset_and_period.clone();

    play_from_state(&figures, instr, moves_after_full_periods as usize, &mut field, &mut row_details, &RowDetails {
        figures_dropped: continuation.figures_dropped,
        figure_index: continuation.figure_index + 1,
        instr_index: continuation.instr_index + 1
    });

    let size = map_size(&field);
    let latest_details = row_details.get(&size.min_row).unwrap();
    println!("size_before_continuation={:?}, size={:?}, latest_details={:?}", size_before_continuation, size, latest_details);

    print_map(&field);

    let ans = i32::abs(size.min_row - size_before_continuation.min_row) as u64 + rows_in_offset as u64 + full_periods * rows_in_period as u64;
    ans
}

fn solve1(figures: &Vec<Figure>, instr: &str) -> i32 {
    let (field, _) = play(&figures, instr, 2022);

    let size = map_size(&field);

    // print_map(&field);
    i32::abs(size.min_row)
}

fn find_period(field: &HashMap<Pos, char>, min: i32, starting_from: i32) -> i32 {
    let size = map_size(field);
    let mut period = min;
    loop {
        let range1 = starting_from..(starting_from + period);
        if range1.end > size.max_row {
            panic!("period not found")
        }
        let rows1 = select_rows(&field, range1);

        let range2 = (starting_from + period)..(starting_from + 2 * period);
        if range2.end > size.max_row {
            panic!("period not found")
        }
        let rows2 = select_rows(&field, range2);

        if same_keys(period, &rows1, &rows2) {
            return period;
        } else {
            // print_map(&rows1);
            // print_map(&rows2);
            period += 1
        }
    }
}

fn same_keys(period: i32, rows1: &HashMap<Pos, char>, rows2: &HashMap<Pos, char>) -> bool {
    rows1.keys().all(|k| rows2.contains_key(&(*k + Pos { row: period, col: 0 })))
}

fn select_rows<T>(map: &HashMap<Pos, T>, range: Range<i32>) -> HashMap<Pos, T> where T: Clone {
    let mut result = HashMap::new();
    for (key, value) in map.iter() {
        if range.contains(&key.row) {
            result.insert(key.clone(), value.clone());
        }
    };
    result
}

#[derive(Debug, Copy, Clone)]
struct RowDetails {
    figures_dropped: usize,
    instr_index: usize,
    figure_index: usize,
}

fn play(figures: &Vec<Figure>, instr: &str, move_count: usize) -> (HashMap<Pos, char>, HashMap<i32, RowDetails>) {
    let mut field = HashMap::new();
    for col in 0..7 {
        field.insert(Pos { row: 0, col }, '~');
    }

    let mut row_details = HashMap::<i32, RowDetails>::new();

    play_from_state(&figures, instr, move_count, &mut field, &mut row_details, &RowDetails { figure_index: 0, figures_dropped: 0, instr_index: 0 });

    (field, row_details)
}

fn play_from_state(figures: &Vec<Figure>,
                   instr: &str,
                   move_count: usize,
                   field: &mut HashMap<Pos, char>,
                   row_details: &mut HashMap<i32, RowDetails>,
                   state: &RowDetails) {
    if move_count == 0 {
        return;
    }

    let start_pos = Pos { row: -4, col: 2 };
    let mut figure_index = state.figure_index;
    let mut figures_dropped = state.figures_dropped;
    let mut instr_index = state.instr_index;
    let mut offset = start_pos + Pos { row: map_size(&field).min_row, col: 0 };
    let mut move_counter = 0;
    loop {
        let figure = &figures[figure_index % figures.len()];
        let ch = instr.chars().nth(instr_index % instr.len()).unwrap();
        match ch {
            '>' => if !collides(&field, &offset.right(), figure) {
                offset = offset.right();
            },
            '<' => if !collides(&field, &offset.left(), figure) {
                offset = offset.left();
            },
            _ => panic!("bad data")
        }
        if !collides(&field, &offset.bottom(), figure) {
            offset = offset.bottom();
        } else {
            add_to_field(field, &offset, figure);

            let size = map_size(&field);

            figures_dropped += 1;

            row_details.insert(size.min_row, RowDetails {
                figures_dropped,
                figure_index,
                instr_index,
            });

            move_counter += 1;
            if move_counter == move_count {
                break;
            }

            offset = start_pos + Pos { row: size.min_row, col: 0 };
            figure_index += 1;
        }
        instr_index += 1;
    }
}

fn collides(field: &HashMap<Pos, char>, offset: &Pos, figure: &Figure) -> bool {
    for (key, value) in figure {
        let in_field_pos = Pos { row: offset.row + key.row, col: offset.col + key.col };
        if in_field_pos.col < 0
            || in_field_pos.col >= 7
            || in_field_pos.row >= 0
            || field.contains_key(&in_field_pos) {
            return true;
        }
    }
    false
}

fn add_to_field(field: &mut HashMap<Pos, char>, offset: &Pos, figure: &Figure) {
    for (key, value) in figure {
        let in_field_pos = *offset + *key;
        field.insert(in_field_pos, value.clone());
    }
}

fn figures() -> Vec<Figure> {
    vec![
        //  HLine
        HashMap::from([
            (Pos { row: 0, col: 0 }, '@'),
            (Pos { row: 0, col: 1 }, '@'),
            (Pos { row: 0, col: 2 }, '@'),
            (Pos { row: 0, col: 3 }, '@'),
        ]),
        //  Plus
        HashMap::from([
            (Pos { row: 0, col: 1 }, '@'),
            (Pos { row: -1, col: 0 }, '@'),
            (Pos { row: -1, col: 1 }, '@'),
            (Pos { row: -1, col: 2 }, '@'),
            (Pos { row: -2, col: 1 }, '@'),
        ]),
        //  Reverse L
        HashMap::from([
            (Pos { row: 0, col: 0 }, '@'),
            (Pos { row: 0, col: 1 }, '@'),
            (Pos { row: 0, col: 2 }, '@'),
            (Pos { row: -1, col: 2 }, '@'),
            (Pos { row: -2, col: 2 }, '@'),
        ]),
        //  VLine
        HashMap::from([
            (Pos { row: 0, col: 0 }, '@'),
            (Pos { row: -1, col: 0 }, '@'),
            (Pos { row: -2, col: 0 }, '@'),
            (Pos { row: -3, col: 0 }, '@'),
        ]),
        //  Square
        HashMap::from([
            (Pos { row: 0, col: 0 }, '@'),
            (Pos { row: 0, col: 1 }, '@'),
            (Pos { row: -1, col: 0 }, '@'),
            (Pos { row: -1, col: 1 }, '@'),
        ]),
    ]
}