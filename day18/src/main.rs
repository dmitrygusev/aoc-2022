use std::collections::{HashMap, HashSet};
use std::{fs, str};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};
use common::{dijkstra, Graph, NodeId};

fn main() {
    assert_eq!(64, solve1("test1"));
    assert_eq!(4450, solve1("input1"));

    assert_eq!(58, solve2("test1"));
    assert_eq!(2564, solve2("input1"));
}

fn solve2(filename: &str) -> usize {
    let cubes = read_cubes(filename);

    let factor: i32 = 2;
    let centers: HashMap<Pos3, &Cube> =
        cubes.iter()
            .map(|cube| ((*cube * factor as i32).center(), cube))
            .collect();

    // println!("centers={:?}", centers);

    let min_col = centers.keys().map(|pos| pos.col).min().unwrap();
    let min_row = centers.keys().map(|pos| pos.row).min().unwrap();
    let min_depth = centers.keys().map(|pos| pos.depth).min().unwrap();

    let max_col = centers.keys().map(|pos| pos.col).max().unwrap();
    let max_row = centers.keys().map(|pos| pos.row).max().unwrap();
    let max_depth = centers.keys().map(|pos| pos.depth).max().unwrap();

    let mesh = create_mesh(factor, min_col, min_row, min_depth, max_col, max_row, max_depth);

    println!("created mesh of {} nodes", mesh.len());

    let graph = create_graph(factor, &centers, &mesh);

    println!("running dijkstra for {} nodes and edges (from {} & to {})",
             graph.nodes.len(), graph.edges_from.len(), graph.edges_to.len());

    let (dist, _) = dijkstra(&graph, &to_node_id(&Pos3 {
        row: min_row - factor,
        col: min_col - factor,
        depth: min_depth - factor,
    }));

    println!("dijkstra solved for {} nodes", dist.len());

    mesh
        .iter()
        //  Only reachable positions
        .filter(|pos|
            if let Some(dist) = dist.get(&to_node_id(pos)) {
                *dist != u32::MAX
            } else {
                false   //  Cube centers are not part of the reachability graph directly
            })
        .flat_map(|pos| pos.neighbours(factor))
        //  Only neighbours that are centers of any cube
        .filter(|pos| centers.contains_key(pos))
        .map(|pos| centers[&pos])
        .count()
}

fn create_graph(factor: i32, centers: &HashMap<Pos3, &Cube>, mesh: &Vec<Pos3>) -> Graph<()> {
    let mut graph: Graph<()> = Graph::new();
    for pos in mesh {
        if centers.contains_key(&pos) {
            continue;
        }

        let from_node = to_node_id(&pos);

        for neighbour in pos.neighbours(factor) {
            if !mesh.contains(&neighbour)
                || centers.contains_key(&neighbour) {
                continue;
            }

            //  Naive copy-on-write implemented in Graph methods is too slow
            graph.nodes.insert(from_node.clone());
            graph.node_attributes.insert(from_node.clone(), ());

            let to_node = to_node_id(&neighbour);
            graph.nodes.insert(to_node.clone());
            graph.node_attributes.insert(to_node.clone(), ());

            if !graph.edges_to.contains_key(&to_node) {
                graph.edges_to.insert(to_node.clone(), HashSet::new());
            }
            graph.edges_to.get_mut(&to_node).expect("illegal state").insert(from_node.clone());

            if !graph.edges_from.contains_key(&from_node) {
                graph.edges_from.insert(from_node.clone(), HashSet::new());
            }
            graph.edges_from.get_mut(&from_node).expect("illegal state").insert(to_node.clone());
        }
    }
    graph
}

fn create_mesh(factor: i32, min_col: i32, min_row: i32, min_depth: i32, max_col: i32, max_row: i32, max_depth: i32) -> Vec<Pos3> {
    let mut mesh = Vec::new();
    for row in (min_row - factor..=max_row + factor).step_by(factor as usize) {
        for col in (min_col - factor..=max_col + factor).step_by(factor as usize) {
            for depth in (min_depth - factor..=max_depth + factor).step_by(factor as usize) {
                let pos = Pos3 { row, col, depth };
                mesh.push(pos);
            }
        }
    }
    mesh
}

fn to_node_id(pos: &Pos3) -> NodeId {
    NodeId::from(pos.to_string().as_str())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Pos3 {
    row: i32,
    col: i32,
    depth: i32,
}

impl Display for Pos3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{r:{},c:{},d:{}}}", self.row, self.col, self.depth)
    }
}

impl Sub for Pos3 {
    type Output = Pos3;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos3 {
            row: self.row - rhs.row,
            col: self.col - rhs.col,
            depth: self.depth - rhs.depth,
        }
    }
}

impl Add for Pos3 {
    type Output = Pos3;

    fn add(self, rhs: Self) -> Self::Output {
        Pos3 {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
            depth: self.depth + rhs.depth,
        }
    }
}

impl Div<i32> for Pos3 {
    type Output = Pos3;

    fn div(self, rhs: i32) -> Self::Output {
        //  Only allow integer division without remainders
        assert_eq!(0, self.row % rhs);
        assert_eq!(0, self.col % rhs);
        assert_eq!(0, self.depth % rhs);
        Pos3 {
            row: self.row / rhs,
            col: self.col / rhs,
            depth: self.depth / rhs,
        }
    }
}

impl Mul<i32> for Pos3 {
    type Output = Pos3;

    fn mul(self, rhs: i32) -> Self::Output {
        Pos3 {
            row: self.row * rhs,
            col: self.col * rhs,
            depth: self.depth * rhs,
        }
    }
}

impl Pos3 {
    fn plus_row(&self, row_delta: i32) -> Pos3 {
        Pos3 { row: self.row + row_delta, col: self.col, depth: self.depth }
    }
    fn plus_col(&self, col_delta: i32) -> Pos3 {
        Pos3 { row: self.row, col: self.col + col_delta, depth: self.depth }
    }
    fn plus_depth(&self, depth_delta: i32) -> Pos3 {
        Pos3 { row: self.row, col: self.col, depth: self.depth + depth_delta }
    }

    fn neighbours(&self, delta: i32) -> [Pos3; 6] {
        [
            Pos3 { row: self.row - delta, col: self.col, depth: self.depth },
            Pos3 { row: self.row + delta, col: self.col, depth: self.depth },
            Pos3 { row: self.row, col: self.col - delta, depth: self.depth },
            Pos3 { row: self.row, col: self.col + delta, depth: self.depth },
            Pos3 { row: self.row, col: self.col, depth: self.depth - delta },
            Pos3 { row: self.row, col: self.col, depth: self.depth + delta },
        ]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Cube {
    v1: Pos3,
    v2: Pos3,
    v3: Pos3,
    v4: Pos3,
    v5: Pos3,
    v6: Pos3,
    v7: Pos3,
    v8: Pos3,
}

impl Mul<i32> for Cube {
    type Output = Cube;

    fn mul(self, rhs: i32) -> Self::Output {
        Cube {
            v1: self.v1 * rhs,
            v2: self.v2 * rhs,
            v3: self.v3 * rhs,
            v4: self.v4 * rhs,
            v5: self.v5 * rhs,
            v6: self.v6 * rhs,
            v7: self.v7 * rhs,
            v8: self.v8 * rhs,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Side {
    points: [Pos3; 4],
}

impl Side {
    fn from4(p1: Pos3, p2: Pos3, p3: Pos3, p4: Pos3) -> Side {
        Side { points: [p1, p2, p3, p4] }
    }
}

#[test]
fn test_cube_center() {
    let cube = Cube::from_pos(Pos3 { row: 2, col: 2, depth: 2 });
    assert_eq!(Cube {
        v1: Pos3 { row: 2, col: 2, depth: 2 },
        v2: Pos3 { row: 3, col: 2, depth: 2 },
        v3: Pos3 { row: 2, col: 3, depth: 2 },
        v4: Pos3 { row: 2, col: 2, depth: 3 },
        v5: Pos3 { row: 2, col: 3, depth: 3 },
        v6: Pos3 { row: 3, col: 2, depth: 3 },
        v7: Pos3 { row: 3, col: 3, depth: 3 },
        v8: Pos3 { row: 3, col: 3, depth: 2 },
    }, cube);
    let scaled = cube * 2;
    assert_eq!(Cube {
        v1: Pos3 { row: 4, col: 4, depth: 4 },
        v2: Pos3 { row: 6, col: 4, depth: 4 },
        v3: Pos3 { row: 4, col: 6, depth: 4 },
        v4: Pos3 { row: 4, col: 4, depth: 6 },
        v5: Pos3 { row: 4, col: 6, depth: 6 },
        v6: Pos3 { row: 6, col: 4, depth: 6 },
        v8: Pos3 { row: 6, col: 6, depth: 4 },
        v7: Pos3 { row: 6, col: 6, depth: 6 },
    }, scaled);
    assert_eq!(Pos3 { row: 5, col: 5, depth: 5 }, scaled.center());
    assert_eq!([
                   Pos3 { row: 3, col: 5, depth: 5 },
                   Pos3 { row: 7, col: 5, depth: 5 },
                   Pos3 { row: 5, col: 3, depth: 5 },
                   Pos3 { row: 5, col: 7, depth: 5 },
                   Pos3 { row: 5, col: 5, depth: 3 },
                   Pos3 { row: 5, col: 5, depth: 7 }
               ],
               scaled.center().neighbours(2));
}

/*
        v6 ---- v7
      /  |    /|
    v2 ---- v8 |
    |  v4 --|- v5
    | /     | /
    v1 ---- v3
*/

impl Cube {
    fn center(&self) -> Pos3 {
        Pos3 {
            row: ((self.v1 + self.v2) / 2).row,
            col: ((self.v1 + self.v3) / 2).col,
            depth: ((self.v1 + self.v4) / 2).depth,
        }
    }

    fn sides(&self) -> [Side; 6] {
        [
            Side::from4(self.v1, self.v2, self.v8, self.v3),
            Side::from4(self.v4, self.v6, self.v7, self.v5),
            Side::from4(self.v1, self.v2, self.v6, self.v4),
            Side::from4(self.v3, self.v8, self.v7, self.v5),
            Side::from4(self.v1, self.v4, self.v5, self.v3),
            Side::from4(self.v2, self.v6, self.v7, self.v8),
        ]
    }

    fn from_pos(pos: Pos3) -> Cube {
        let v1 = pos.clone();
        let v2 = pos.plus_row(1);
        let v3 = pos.plus_col(1);
        let v4 = pos.plus_depth(1);
        let v5 = v4.plus_col(1);
        let v6 = v4.plus_row(1);
        let v7 = v5.plus_row(1);
        let v8 = v2.plus_col(1);

        Cube { v1, v2, v3, v4, v5, v6, v7, v8 }
    }
}

fn solve1(filename: &str) -> usize {
    let cubes = read_cubes(filename);

    part1(cubes.iter().collect())
}

fn part1(cubes: Vec<&Cube>) -> usize {
    let mut sides =
        cubes.iter()
            .flat_map(|cube| cube.sides())
            .collect::<Vec<Side>>();

    let all = sides.len();
    sides.sort();
    sides.dedup();
    let after_dedup = sides.len();

    all - (all - after_dedup) * 2
}

fn read_cubes(filename: &str) -> Vec<Cube> {
    let s = fs::read_to_string(filename).unwrap();
    let cubes: Vec<Cube> = s.lines()
        .map(|line| {
            let split: Vec<&str> = line.split(",").collect();
            Cube::from_pos(Pos3 {
                row: split[0].parse().unwrap(),
                col: split[1].parse().unwrap(),
                depth: split[2].parse().unwrap(),
            })
        })
        .collect();
    cubes
}