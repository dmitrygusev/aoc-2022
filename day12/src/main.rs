use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use common::{Pos, print_map};

fn main() {
    assert_eq!(31, solve1("test1"));
    assert_eq!(330, solve1("input1"));

    assert_eq!(29, solve2("test1"));
    assert_eq!(321, solve2("input1"));
}

fn solve2(filename: &str) -> usize {
    let (map, _, end) = parse_map(filename);

    print_map(&map);

    let (dist, prev) = dijkstra(&map, &end.unwrap(), -1);

    print_map(&dist);

    let mut paths = Vec::new();
    for (pos, ch) in map {
        if ch != 'a' { continue; }

        let path = get_path(&prev, &pos);
        if path.len() != 1 {
            paths.push(path.len() - 1);
        }
    }

    *paths.iter().min().unwrap()
}

fn solve1(filename: &str) -> usize {
    let (map, start, end) = parse_map(filename);

    print_map(&map);

    let (dist, prev) = dijkstra(&map, &start.unwrap(), 1);

    print_map(&dist);

    let path = get_path(&prev, &end.unwrap());
    println!("{:?} => {:?}", path.len(), path);

    path.len() - 1
}

fn parse_map(filename: &str) -> (HashMap<Pos, char>, Option<Pos>, Option<Pos>) {
    let s = fs::read_to_string(filename).expect("bad input");

    let mut map: HashMap::<Pos, char> = HashMap::new();

    let mut start: Option<Pos> = None;
    let mut end: Option<Pos> = None;

    let mut row = 0;
    for line in s.lines() {
        let mut col = 0;
        for mut ch in line.chars() {
            let pos = Pos { row, col };
            if ch == 'S' {
                ch = 'a';
                start = Some(pos);
            } else if ch == 'E' {
                ch = 'z';
                end = Some(pos);
            }
            map.insert(pos, ch);
            col += 1;
        }
        row += 1;
    }
    (map, start, end)
}

fn get_path(prev: &HashMap<Pos, Pos>, from: &Pos) -> Vec<Pos> {
    let mut path = Vec::new();
    path.push(from.clone());
    while prev.contains_key(&path.last().unwrap()) {
        path.push(prev[&path.last().unwrap()]);
    }
    path
}

fn dijkstra<'a>(map: &HashMap<Pos, char>, start: &Pos, gradient: i32) -> (HashMap<Pos, u32>, HashMap<Pos, Pos>) {
    let mut dist = HashMap::new();
    let mut prev = HashMap::new();
    let mut queue = Vec::new();

    for (v_pos, _) in map {
        dist.insert(v_pos.clone(), u32::MAX);
        queue.push(v_pos.clone());
    }

    dist.insert(start.clone(), 0);

    while !queue.is_empty() {
        let min_index = queue.iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| dist[a].cmp(&dist[b]))
            .map(|(index, _)| index)
            .unwrap();

        let u = queue.remove(min_index);
        let u_ch = map[&u];

        for d in u.neighbours() {
            if !queue.contains(&d) { continue; }
            let d_ch = map[&d];
            let diff = gradient * d_ch as i32 - gradient * u_ch as i32;
            if diff > 1 {
                //  constraint
                continue;
            }
            if dist[&u] == u32::MAX {
                //  TODO Why is this happening? Unreachable because of other constraints?
                // print_map(&dist);
                // panic!("{}", dbg!(&u));
                continue;
            }
            let alt = dist[&u] + 1;
            if alt < dist[&d] {
                dist.insert(d.clone(), alt);
                prev.insert(d.clone(), u.clone());
            }
        }
    }

    (dist, prev)
}
