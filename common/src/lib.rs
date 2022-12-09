use std::collections::HashMap;
use std::fmt::{Debug, Display};


#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Pos {
    pub row: i32,
    pub col: i32,
}

impl Pos {
    pub fn left(&self) -> Pos {
        Pos { row: self.row, col: self.col - 1 }
    }
    pub fn top(&self) -> Pos {
        Pos { row: self.row - 1, col: self.col }
    }
    pub fn right(&self) -> Pos {
        Pos { row: self.row, col: self.col + 1 }
    }
    pub fn bottom(&self) -> Pos {
        Pos { row: self.row + 1, col: self.col }
    }
}

pub struct Size {
    pub min_row: i32,
    pub max_row: i32,
    pub min_col: i32,
    pub max_col: i32,
}

pub fn map_size<T>(map: &HashMap<Pos, T>) -> Size {
    Size {
        min_row: map.keys().map(|pos| pos.row).min().expect("bad data"),
        max_row: map.keys().map(|pos| pos.row).max().expect("bad data"),
        min_col: map.keys().map(|pos| pos.col).min().expect("bad data"),
        max_col: map.keys().map(|pos| pos.col).max().expect("bad data"),
    }
}

pub fn print_map<T>(map: &HashMap<Pos, T>) where T: Display {
    let size = map_size(map);
    for row in size.min_row..=size.max_row {
        for col in size.min_col..=size.max_col {
            let str = match map.get(&Pos { row, col }) {
                None => String::from("."),
                Some(state) => state.to_string()
            };
            print!("{}", str);
        }
        println!();
    }
    println!("---");
}
