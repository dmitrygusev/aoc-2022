use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter, Write};
use std::fs::File;
use std::io::Write as IoWrite;

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
        println!(" {}", row);
    }
    println!("---");
}

// ---

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeId(String);

impl Display for NodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

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

pub fn print_graph_dot<NA>(graph: &Graph<NA>) where NA: Display, NA: Clone {
    let buf = build_graph_dot(graph);
    println!("{}", buf);
}

pub fn create_dot_file<NA>(graph: &Graph<NA>, filename: &str) where NA: Display, NA: Clone {
    let buf = build_graph_dot(graph);
    let mut file = File::create(filename).unwrap();
    write!(file, "{}", buf);
}

fn build_graph_dot<NA>(graph: &Graph<NA>)  -> String where NA: Display, NA: Clone {
    let mut buf = String::new();
    writeln!(buf, "digraph G {{");
    for node_id in graph.nodes.iter() {
        let attr = match graph.node_attributes.get(node_id) {
            Some(state) => state.to_string(),
            None => String::from("")
        };
        write!(buf, "  {0} [label=\"{0}, {1}\"{2}]\n", node_id, attr,
               if attr.contains("rate=0") {""} else {",fillcolor=\"green\",style=\"filled\",fontcolor=\"white\""});
        if let Some(to_nodes) = graph.edges_from.get(node_id) {
            if !to_nodes.is_empty() {
                write!(buf, "  {} -> {{", node_id);
                write_node_ids(&mut buf, to_nodes);
                writeln!(buf, "}}");
            }
        }
    }
    write!(buf, "}}");
    buf
}

fn write_node_ids(buf: &mut String, nodes: &HashSet<NodeId>) {
    let mut first = true;
    for node_id in nodes.iter() {
        if first {
            first = false;
        } else if nodes.len() > 1 {
            write!(buf, "; ").unwrap();
        }
        write!(buf, "{}", node_id);
    }
}

pub type Dist = HashMap<NodeId, u32>;
pub type Prev = HashMap<NodeId, NodeId>;

pub fn dijkstra<NA>(graph: &Graph<NA>, start: &NodeId) -> (Dist, Prev) where NA: Clone
{
    let mut dist = HashMap::new();
    let mut prev = HashMap::new();
    let mut queue = Vec::new();

    for node_id in graph.nodes.iter() {
        dist.insert(node_id.clone(), u32::MAX);
        queue.push(node_id.clone());
    }

    dist.insert(start.clone(), 0);

    while !queue.is_empty() {
        let min_index = queue.iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| dist[a].cmp(&dist[b]))
            .map(|(index, _)| index)
            .unwrap();

        let u = queue.remove(min_index);

        for d in graph.edges_to[&u].iter() {
            if !queue.contains(&d) { continue; }
            let alt = dist[&u] + 1;
            if alt < dist[&d] {
                dist.insert(d.clone(), alt);
                prev.insert(d.clone(), u.clone());
            }
        }
    }

    (dist, prev)
}

pub fn get_path(prev: &Prev, from: &NodeId) -> Vec<NodeId> {
    let mut path = Vec::new();
    path.push(from.clone());
    while prev.contains_key(&path.last().unwrap()) {
        path.push(prev[path.last().unwrap()].clone());
    }
    path
}
