// const INPUTS: &str = r#"
// start-A
// start-b
// A-c
// A-b
// b-d
// A-end
// b-end
// "#;
const INPUTS: &str = include_str!("./inputs/day12.txt");

use std::collections::{HashMap, HashSet};

type Graph = HashMap<String, Vec<String>>;
pub fn run() {
    let mut graph: Graph = HashMap::new();
    for line in INPUTS.trim().lines() {
        let (from, to) = line.trim().split_once("-").unwrap();

        (*graph.entry(from.to_string()).or_insert(vec![])).push(to.to_string());
        (*graph.entry(to.to_string()).or_insert(vec![])).push(from.to_string());
    }

    let answer = walk(&graph, "start", HashSet::new());
    println!("answer: {}", answer);
}

fn walk<'a>(graph: &Graph, node: &'a str, mut seen: HashSet<&'a str>) -> i32 {
    if seen.contains(&node) {
        return 0;
    }

    if node == "end" {
        return 1;
    }

    if node == node.to_lowercase() {
        seen.insert(node);
    }

    graph[node]
        .iter()
        .map(|node| walk(graph, node, seen.clone()))
        .sum()
}
