use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};

pub struct Marker {
    pub visited: bool,
}

impl Display for Marker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.visited { "#" } else { "." })
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Pos {
    pub row: i32,
    pub col: i32,
}

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(r{},c{})", self.row, self.col)
    }
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
    pub fn neighbours(&self) -> [Pos; 4] {
        [self.left(), self.top(), self.right(), self.bottom()]
    }
}

#[derive(Debug)]
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
            print!("{} ", str);
        }
        println!();
    }
    println!("---");
}

// ---

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeId(String);

impl NodeId {
    pub fn from(s: &str) -> NodeId {
        NodeId(String::from(s))
    }
    pub fn from_path(path: &Vec<NodeId>) -> NodeId {
        NodeId(path
            .iter()
            .map(|n| n.0.as_str())
            .collect::<Vec<&str>>()
            .join("/")
            .replacen("//", "/", 1))
    }
    pub fn append(&self, node: &NodeId) -> NodeId {
        let mut path = Vec::with_capacity(2);
        path.push(self.clone());
        path.push(node.clone());
        NodeId::from_path(&path)
    }
}

pub struct Graph<NA> where NA: Clone {
    pub nodes: HashSet<NodeId>,
    pub node_attributes: HashMap<NodeId, NA>,
    pub edges_to: HashMap<NodeId, HashSet<NodeId>>,
    pub edges_from: HashMap<NodeId, HashSet<NodeId>>,
}

impl<NA> Graph<NA> where NA: Clone {
    pub fn get_incoming(&self, node: &NodeId) -> Option<&HashSet<NodeId>> {
        self.edges_to.get(node)
    }

    pub fn add_node(&self, node: &NodeId, attr: Option<NA>) -> Graph<NA> {
        Graph {
            nodes: {
                let mut set = self.nodes.clone();
                set.insert(node.clone());
                set
            },
            node_attributes: {
                match attr {
                    None => self.node_attributes.clone(),
                    Some(attr) => {
                        let mut map = self.node_attributes.clone();
                        map.insert(node.clone(), attr);
                        map
                    }
                }
            },
            edges_to: self.edges_to.clone(),
            edges_from: self.edges_from.clone(),
        }
    }

    pub fn add_edge(&self, from: &NodeId, to: &NodeId) -> Graph<NA> {
        assert!(self.nodes.contains(from));
        assert!(self.nodes.contains(to));

        Graph {
            nodes: self.nodes.clone(),
            node_attributes: self.node_attributes.clone(),
            edges_to: {
                let mut map = self.edges_to.clone();
                if !map.contains_key(to) {
                    map.insert((*to).clone(), HashSet::new());
                }
                map.get_mut(to).expect("illegal state").insert((*from).clone());
                map
            },
            edges_from: {
                let mut map = self.edges_from.clone();
                if !map.contains_key(from) {
                    map.insert((*from).clone(), HashSet::new());
                }
                map.get_mut(from).expect("illegal state").insert((*to).clone());
                map
            },
        }
    }

    pub fn new() -> Graph<NA> {
        Graph {
            nodes: HashSet::new(),
            node_attributes: HashMap::new(),
            edges_to: HashMap::new(),
            edges_from: HashMap::new(),
        }
    }
}
