use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::fs;
use common::{create_dot_file, dijkstra, get_path, Graph, NodeId, print_graph_dot};
use regex::Regex;

fn main() {
    assert_eq!(1651, solve1("test1").releasing_pressure);
    assert_eq!(1947, solve1("input1").releasing_pressure);

    assert_eq!(1707, solve2("test1"));
    assert_eq!(2556, solve2("input1"));
}

#[derive(Clone, Debug)]
struct State {
    releasing_pressure: u32,
    segments: Vec<Vec<NodeId>>,
}

fn solve2(filename: &str) -> u32 {
    let graph = parse_graph(filename);

    let from = NodeId::from("AA");

    let mut all_states = create_states(
        &graph,
        &vec![],
        &from,
        &HashSet::new(),
        0,
        26);

    all_states.sort_by(|a, b| a.releasing_pressure.cmp(&b.releasing_pressure));

    let mut max = 0;
    for path1 in all_states.iter().rev() {
        if let Some(path2) = complement(&all_states, flatten(&path1.segments)) {
            let total_pressure = path1.releasing_pressure + path2.releasing_pressure;
            if max < total_pressure {
                max = total_pressure;
            }
        }
    }

    max
}

fn complement(all_states: &Vec<State>, flatten: HashSet<NodeId>) -> Option<&State> {
    all_states
        .iter()
        .rev()
        .find(|state|
            state.segments.iter()
                .find(|s| flatten.contains(s.first().unwrap()))
                .is_none())
}

fn solve1(filename: &str) -> State {
    let graph = parse_graph(filename);

    create_dot_file(&graph, &format!("{}.dot", filename));

    let from = NodeId::from("AA");

    let mut all_states = create_states(
        &graph,
        &vec![],
        &from,
        &HashSet::new(),
        0,
        30);

    all_states.sort_by(|a, b| a.releasing_pressure.cmp(&b.releasing_pressure));

    println!("{}", all_states.len());

    let tail = &all_states[all_states.len() - 2..];

    println!("{:?}", tail);

    all_states.last().unwrap().clone()
}

fn find_state(all_states: &Vec<State>, path: Vec<(NodeId, NodeId)>) -> Option<&State> {
    'outer:
    for state in all_states {
        let segments = state.segments.clone();
        if segments.len() != path.len() {
            continue;
        }
        for i in 0..path.len() {
            let segment = &segments[i];
            let (from, to) = &path[i];
            let first_in_segment = segment.first().unwrap();
            let last_in_segment = segment.last().unwrap();
            if !(first_in_segment == to && last_in_segment == from) {
                continue 'outer;
            }
        }
        return Some(state);
    }
    None
}

fn create_states(graph: &Graph<Valve>,
                 path_to_from: &Vec<Vec<NodeId>>,
                 from: &NodeId,
                 exclude: &HashSet<NodeId>,
                 previous_pressure_released: u32,
                 time_budget: u32) -> Vec<State> {

    // println!("from {:?} excluding {:?}", from, exclude);

    let (_, prev) = dijkstra(&graph, &from);

    let mut new_states = Vec::new();

    for node_id in &graph.nodes {
        let attr = &graph.node_attributes[node_id];
        if attr.rate > 0 && !exclude.contains(node_id) {
            let path = get_path(&prev, node_id);
            let mut path_segments = Vec::new();
            for p in path_to_from {
                path_segments.push(p.clone());
            }
            path_segments.push(path);
            let valves_visited = path_segments.iter().map(|p| p.len() as u32 - 1).sum::<u32>();
            let valves_opened = path_segments.len() as u32;
            let time_spent = valves_visited + valves_opened;
            if time_spent > time_budget {
                continue;
            }
            let releasing_pressure = previous_pressure_released + attr.rate * (time_budget - time_spent);

            // println!("{}[{}], releasing_pressure={}, new_path_to_from={:?}",
            //          node_id, attr, releasing_pressure, path_segments);

            new_states.push(State {
                segments: path_segments,
                releasing_pressure,
            });
        }
    }

    let mut all_child = Vec::new();

    for state in new_states.iter() {
        let child_states =
            create_states(
                &graph,
                &state.segments,
                &state.segments.last().unwrap().first().unwrap(),
                &flatten(&state.segments),
                state.releasing_pressure,
                time_budget,
            );

        for child_state in child_states {
            all_child.push(child_state);
        }
    }

    for state in all_child {
        new_states.push(state);
    }

    // println!("--");

    new_states
}

fn flatten(segments: &Vec<Vec<NodeId>>) -> HashSet<NodeId> {
    segments.iter()
        .map(|s| s.first().unwrap().clone())
        .collect()
}

#[derive(Clone)]
struct Valve {
    rate: u32,
}

impl Display for Valve {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "rate={}", self.rate)
    }
}

fn parse_graph(filename: &str) -> Graph<Valve> {
    let s = fs::read_to_string(filename).unwrap();

    let pattern =
        Regex::new("Valve (\\w+) has flow rate=(\\d+); tunnels? leads? to valves? (.*)").unwrap();

    let mut graph = Graph::new();

    for line in s.lines() {
        let matcher = pattern.captures(line).unwrap();
        let node_id = NodeId::from(&matcher[1]);
        let valve = Valve { rate: matcher[2].parse().unwrap() };
        graph = graph.add_node(&node_id, Some(valve));

        for target_node_id in matcher[3].split(", ") {
            let target_node_id = NodeId::from(target_node_id);
            graph = graph.add_node(&target_node_id, None);
            graph = graph.add_edge(&node_id, &target_node_id);
        }
    }

    graph
}