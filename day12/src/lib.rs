use petgraph::graphmap::UnGraphMap;
use std::collections::HashSet;

#[derive(Clone)]
struct Graph<'a> {
    graph: UnGraphMap<&'a str, &'a str>,
    twice_node: Option<String>,
}

pub fn process(input: &str) -> Option<u32> {
    let lines: Vec<(&str, &str)> = input
        .lines()
        .map(|l| {
            let s = l.split_once('-').unwrap();
            s
        })
        .collect();

    let graph = Graph {
        graph: UnGraphMap::<&str, &str>::from_edges(lines.as_slice()),
        twice_node: None,
    };

    let mut visited_nodes = HashSet::new();

    visited_nodes.insert("start".to_owned());

    let c = walk(&graph, "start", &mut visited_nodes);

    Some(c)
}

pub fn process_pt2(input: &str) -> Option<u32> {
    let lines: Vec<(&str, &str)> = input
        .lines()
        .map(|l| {
            let s = l.split_once('-').unwrap();
            s
        })
        .collect();

    let mut graph = Graph {
        graph: UnGraphMap::<&str, &str>::from_edges(lines.as_slice()),
        twice_node: None,
    };

    let mut visited_nodes: HashSet<String> = HashSet::new();

    visited_nodes.insert("start".to_owned());

    let c = walk_twice(&mut graph, "start", &mut visited_nodes);

    Some(c)
}

fn walk(graph: &Graph, curr_node: &str, visited_nodes: &mut HashSet<String>) -> u32 {
    if curr_node == "end" {
        visited_nodes.remove(curr_node);
        return 1;
    }

    let neighbors = graph.graph.neighbors(curr_node);

    let mut count = 0;

    for n in neighbors {
        if n.to_ascii_lowercase() == *n && visited_nodes.contains(n) {
            continue;
        } else {
            visited_nodes.insert(n.to_owned());
        }

        count += walk(graph, n, visited_nodes);

        visited_nodes.remove(n);
    }

    count
}

fn walk_twice(graph: &mut Graph, curr_node: &str, visited_nodes: &mut HashSet<String>) -> u32 {
    if curr_node == "end" {
        visited_nodes.remove(curr_node);
        return 1;
    }

    let neighbors = graph.graph.neighbors(curr_node);

    let mut count = 0;

    for n in neighbors {
        if n.to_ascii_lowercase() != *n || !visited_nodes.contains(n) {
            visited_nodes.insert(n.to_owned());
        } else if graph.twice_node.is_none() && n != "start" && n != "end" {
            graph.twice_node = Some(n.to_owned());
        } else {
            continue;
        }

        if graph.twice_node.is_some() {
            count += walk(graph, n, visited_nodes);
        } else {
            count += walk_twice(&mut graph.clone(), n, visited_nodes);
        }

        if graph.twice_node == Some(n.to_owned()) {
            graph.twice_node = None;
        } else {
            visited_nodes.remove(n);
        }
    }

    count
}
