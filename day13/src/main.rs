use Ordering::Equal;
use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Less};
use std::fs;
use std::ops::Add;

use crate::Elem::{Leaf, List};

fn main() {
    assert_eq!(13, solve1("test1"));
    assert_eq!(5623, solve1("input1"));

    assert_eq!(140, solve2("test1"));
    assert_eq!(20570, solve2("input1"));
}

fn solve2(filename: &str) -> u32 {
    let s = fs::read_to_string(filename).unwrap()
        .add("\n[[2]]")
        .add("\n[[6]]");

    let mut elems: Vec<Elem> =
        s.lines()
            .filter(|line| !line.is_empty())
            .map(parse_elem)
            .collect();

    elems.sort_by(|a, b| is_ordered((a, b)));

    // println!("{:?}", elems);

    let index_of2 = 1 + elems.iter().position(|a| {
        let b = List(vec![List(vec![Leaf(2)])]);
        a.eq(&b)
    }).unwrap();

    let index_of6 = 1 + elems.iter().position(|a| {
        let b = List(vec![List(vec![Leaf(6)])]);
        a.eq(&b)
    }).unwrap();

    index_of2 as u32 * index_of6 as u32
}

fn solve1(filename: &str) -> u32 {
    let s = fs::read_to_string(filename).unwrap();

    let pairs: Vec<(usize, (Elem, Elem))> =
        s.split("\n\n")
            .enumerate()
            .map(|(index, pair)| {
                let (first, second) = pair.split_once("\n").unwrap();
                (index, (parse_elem(first), parse_elem(second)))
            })
            .collect();

    // println!("{:?}", pairs);

    pairs.iter()
        .filter(|(_, (left, right))| is_ordered((&left, &right)) == Less)
        .map(|(index, _)| *index as u32 + 1)
        .sum()
}

fn is_ordered(pair: (&Elem, &Elem)) -> Ordering {
    match pair {
        (Leaf(a), Leaf(b)) => a.cmp(b),
        (Leaf(_), List(_)) => is_ordered((&pair.0.wrap(), pair.1)),
        (List(_), Leaf(_)) => is_ordered((pair.0, &pair.1.wrap())),
        (List(a), List(b)) => {
            let mut i = 0;
            loop {
                if i >= a.len() {
                    if a.len() == b.len() {
                        return Equal
                    }
                    return Less
                }

                if i >= b.len() {
                    return Greater
                }

                let ordered = is_ordered((&a[i], &b[i]));
                if ordered != Equal {
                    // println!("{:?} = {:?} <?> {:?}", ordered, a, b);
                    return ordered
                }

                i += 1
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Elem {
    Leaf(u32),
    List(Vec<Elem>),
}

impl Elem {
    pub(crate) fn wrap(&self) -> Elem {
        List(vec![self.clone()])
    }
}

fn parse_elem(s: &str) -> Elem {
    if s.starts_with("[") {
        let mut i: usize = 1;
        let elem = List(parse_list(s, &mut i));
        return elem;
    } else {
        panic!("bad data")
    }
}

fn parse_list(s: &str, mut i: &mut usize) -> Vec<Elem> {
    let mut result = Vec::new();

    let mut buf = String::new();
    loop {
        let ch = s[*i..=*i].chars().next().unwrap();
        *i += 1;
        if ch.is_digit(10) {
            buf.push(ch);
        } else if ch == ',' {
            if !buf.is_empty() {
                result.push(Leaf(buf.parse().unwrap()));
                buf.clear();
            }
        } else if ch == ']' {
            if !buf.is_empty() {
                result.push(Leaf(buf.parse().unwrap()));
            }
            return result;
        } else if ch == '[' {
            result.push(List(parse_list(s, &mut i)));
        }
    }
}
