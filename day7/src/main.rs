use std::fs;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use common::{Graph, NodeId};
use crate::NodeAttr::{Dir, File};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum NodeAttr {
    Dir,
    File(u64),
}

fn main() {
    let graph = read_input("test1");
    let size = get_size(&graph, &NodeId::from("/"));
    assert_eq!(48381165, size); //  different size metric
    let size2 = get_size2(&graph);
    assert_eq!(95437, size2);

    assert_eq!(24933642, part2(&graph));

    let graph = read_input("input1");
    // println!("{:?}", graph.node_attributes);
    let size2 = get_size2(&graph);
    assert_ne!(48518336, size2); //  48518336 too high
    assert_ne!(94853, size2); //  94853 too low
    assert_eq!(1428881, size2);

    assert_eq!(10475598, part2(&graph));
}

fn part2(graph: &Graph<NodeAttr>) -> u64 {
    let disk_size = 70000000;
    let space_needed = 30000000;

    let size = get_size(&graph, &NodeId::from("/"));

    let unused_space = disk_size - size;
    let need_to_delete = space_needed - unused_space;

    graph.node_attributes
        .iter()
        .filter(|(_, attr)| (**attr) == Dir)
        .map(|(node, _)| get_size(&graph, node))
        .filter(|size| *size >= need_to_delete)
        .min()
        .expect("bad data")
}

fn get_size2(graph: &Graph<NodeAttr>) -> u64 {
    graph.node_attributes
        .iter()
        .filter(|(_, attr)| (**attr) == Dir)
        .map(|(node, _)| get_size(graph, node))
        .filter(|size| *size <= 100000)
        .sum()
}

fn get_size(graph: &Graph<NodeAttr>, node: &NodeId) -> u64 {
    return match graph.node_attributes.get(node).expect("bad data") {
        Dir => {
            if let Some(child_nodes) = graph.edges_from.get(&node) {
                let size = child_nodes
                    .iter()
                    .map(|child| get_size(graph, child))
                    .sum();

                return size;
            }
            //  Empty dir?
            0
        }
        File(size) => *size
    };
}

fn read_input(filename: &str) -> Graph<NodeAttr> {
    let s = fs::read_to_string(filename).expect("bad input");

    let mut graph = Graph::new();

    let mut path = Vec::<NodeId>::new();
    path.push(NodeId::from("/"));

    graph = graph.add_node(&NodeId::from_path(&path), Some(Dir));

    for line in s.lines().skip(1) {
        // println!("{}", line);
        if let Some((_, cd)) = line.split_once("$ cd ") {
            match cd {
                ".." => {
                    path.pop();
                    let cwd = NodeId::from_path(&path);
                    //  No incoming edges for root "/"
                    if let Some(incoming) = graph.get_incoming(&cwd) {
                        assert_eq!(incoming.len(), 1);
                    }
                }
                _ => {
                    let from = NodeId::from_path(&path);
                    path.push(NodeId::from(cd));
                    let to = NodeId::from_path(&path);
                    graph = graph.add_node(&to, Some(Dir));
                    graph = graph.add_edge(&from, &to);
                }
            }
        } else if line.eq("$ ls") {
            continue;
        } else if let Some((attr, name)) = line.split_once(" ") {
            let from = NodeId::from_path(&path);
            let to = from.append(&NodeId::from(name));
            match attr {
                "dir" => {
                    graph = graph.add_node(&to, Some(Dir));
                }
                _ => {
                    graph = graph.add_node(&to, Some(File(attr.parse().expect("bad data"))));
                }
            }
            graph = graph.add_edge(&from, &to);
        } else {
            panic!("bad input")
        }
    }

    let more_than_one_parent = graph.edges_to.values().filter(|v| v.len() > 1).count();

    assert_eq!(0, more_than_one_parent);

    graph
}
