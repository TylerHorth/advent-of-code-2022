use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use petgraph::{Graph, Directed, graph::NodeIndex, algo::dijkstra, Direction, visit::EdgeRef};
use regex::Regex;

type VGraph = Graph::<u32, u32, Directed>;

#[aoc_generator(day16)]
fn parse(data: &str) -> (NodeIndex, VGraph) {
    let re = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (([A-Z]{2}, )*[A-Z]{2})").unwrap();
    let mut lines = Vec::new();

    for line in data.lines() {
        let captures = re.captures(line).unwrap();
        let name = captures.get(1).unwrap().as_str();
        let rate = captures.get(2).unwrap().as_str().parse().unwrap();
        let edges = captures.get(3).unwrap().as_str().split(", ").collect();
        lines.push((name, rate, edges));
    }

    create_graph(lines)
}

fn create_graph(data: Vec<(&str, u32, Vec<&str>)>) -> (NodeIndex, VGraph) {
    let mut index = HashMap::new();
    let mut graph = Graph::new();

    for (name, rate, _) in &data {
        let node = graph.add_node(*rate);
        index.insert(*name, node);
    }

    for (name, _, edges) in data {
        let node = index[name];
        for edge in edges {
            graph.add_edge(node, index[edge], 1);
        }
    }

    compress(index["AA"], graph)
}

fn compress(root: NodeIndex, graph: VGraph) -> (NodeIndex, VGraph) {
    let mut new_graph = Graph::new();
    let non_zero: HashMap<_, _> = graph.node_indices()
        .filter(|&n| graph[n] > 0)
        .map(|n| (n, new_graph.add_node(graph[n])))
        .collect();

    let new_root = new_graph.add_node(graph[root]);
    let costs = dijkstra(&graph, root, None, |_| 1);
    for (&old_node, &new_node) in non_zero.iter() {
        new_graph.add_edge(new_root, new_node, costs[&old_node]);
    }

    for (&old_node, &new_node) in non_zero.iter() {
        let costs = dijkstra(&graph, old_node, None, |_| 1);
        for (&old_next, &new_next) in non_zero.iter() {
            if old_next != old_node {
                new_graph.add_edge(new_node, new_next, costs[&old_next]);
            }
        }
    }

    (new_root, new_graph)
}

fn key(node: NodeIndex) -> usize {
    1 << node.index()
}

fn max_pressure(graph: &VGraph, node: NodeIndex, minutes_remaining: u32, valves_opened: usize, pressure: u32, result: &mut Vec<u32>) {
    result[valves_opened] = result[valves_opened].max(pressure);

    for edge in graph.edges_directed(node, Direction::Outgoing) {
        let new_minutes_remaining = minutes_remaining.saturating_sub(edge.weight() + 1);
        if new_minutes_remaining == 0 {
            continue
        }

        let new_valves_opened = valves_opened | key(edge.target());
        if new_valves_opened == valves_opened {
            continue
        }

        let new_pressure = pressure + new_minutes_remaining * graph.node_weight(edge.target()).unwrap();
        max_pressure(graph, edge.target(), new_minutes_remaining, new_valves_opened, new_pressure, result);
    }
}

fn create_result_vec(graph: &VGraph) -> Vec<u32> {
    let max_index = graph.node_indices().map(NodeIndex::index).max().unwrap();
    assert!(max_index < 20); // sanity: 2^20 * 4 = 4 MB
    vec![0; (1 << (max_index + 1)) - 1]
}

#[aoc(day16, part1)]
fn part1((root_node, graph): &(NodeIndex, VGraph)) -> u32 {
    let mut result = create_result_vec(graph);
    max_pressure(graph, *root_node, 30, 0, 0, &mut result);
    result.into_iter().max().unwrap()
}

#[aoc(day16, part2)]
fn part2((root_node, graph): &(NodeIndex, VGraph)) -> u32 {
    let mut result = create_result_vec(graph);
    max_pressure(graph, *root_node, 26, 0, 0, &mut result);

    result.iter()
        .copied()
        .enumerate()
        .flat_map(|(v1, p1)| {
            result.iter()
                .copied()
                .enumerate()
                .filter(move |&(v2, _)| v1 & v2 == 0)
                .map(move |(_, p2)| p1 + p2)
        })
        .max()
        .unwrap()
}