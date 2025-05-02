use std::collections::{HashMap, HashSet, VecDeque};
use petgraph::graph::NodeIndex;
use std::fs::File;
use petgraph::Undirected;
use std::io::{BufWriter, Write};
use crate::UndirectedGraph;
use petgraph::graph::Graph;
use std::io::BufRead;
use crate::bfs_paths;

pub fn load_graph_from_file(file_path: &str) -> Graph<(), (), petgraph::Undirected> {
    use std::collections::HashMap;
    let mut graph = Graph::new_undirected();
    let mut node_map: HashMap<String, NodeIndex> = HashMap::new();

    let file = std::fs::File::open(file_path).expect("Failed to open file");
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.trim().is_empty() || line.trim().starts_with('#') {
            continue;
        }

        let nodes: Vec<&str> = line.split_whitespace().collect();
        if nodes.len() != 2 {
            continue;
        }

        let node1 = nodes[0].to_string();
        let node2 = nodes[1].to_string();

        let node1_index = *node_map.entry(node1.clone()).or_insert_with(|| graph.add_node(()));
        let node2_index = *node_map.entry(node2.clone()).or_insert_with(|| graph.add_node(()));

        graph.add_edge(node1_index, node2_index, ());
    }

    graph
}
/// Normalize centrality scores to the range [0, 1]
pub fn normalize_scores(scores: &HashMap<NodeIndex, f64>) -> HashMap<NodeIndex, f64> {
    let max = scores.values().cloned().fold(f64::NEG_INFINITY, f64::max);
    if max == 0.0 || !max.is_finite() {
        return scores.clone(); // nothing to normalize
    }

    scores.iter()
        .map(|(&node, &score)| (node, score / max))
        .collect()
}

pub fn rank_top_n(
    scores: &HashMap<NodeIndex, f64>,
    n: usize,
) -> Vec<(NodeIndex, f64)> {
    let mut ranked: Vec<(NodeIndex, f64)> = scores.iter().map(|(&node, &score)| (node, score)).collect();
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    ranked.into_iter().take(n).collect()
}


/// Computes degree centrality (normalized)
pub fn degree_centrality(graph: &UndirectedGraph) -> HashMap<NodeIndex, f64> {
    let n = graph.node_count() as f64;
    graph.node_indices()
        .map(|node| {
            let degree = graph.neighbors(node).count() as f64;
            (node, degree / (n - 1.0))
        })
        .collect()
}


pub fn bfs_sample_nodes(graph: &UndirectedGraph, start_node: NodeIndex, limit: usize) -> Vec<NodeIndex> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut result = Vec::new();

    visited.insert(start_node);
    queue.push_back(start_node);
    result.push(start_node);

    while let Some(node) = queue.pop_front() {
        for neighbor in graph.neighbors(node) {
            if visited.len() >= limit {
                break;
            }
            if visited.insert(neighbor) {
                queue.push_back(neighbor);
                result.push(neighbor);
            }
        }
    }

    result
}
pub fn betweenness_centrality_sampled(
    graph: &Graph<(), (), petgraph::Undirected>,
    sample_nodes: &[NodeIndex]) -> HashMap<NodeIndex, f64> {
    let mut betweenness_scores: HashMap<NodeIndex, f64> = HashMap::new();

    // Iterate over all pairs of nodes as source and target
    for source in sample_nodes.iter().cloned() {
        let predecessors = bfs_paths(graph, source);

        for target in graph.node_indices() {
            if source != target {
                let mut path = vec![target];
                let mut current = target;

                // Trace the shortest path back to the source using predecessors
                while let Some(predecessor) = predecessors.get(&current).and_then(|p| p.get(0)) {
                    path.push(*predecessor);
                    current = *predecessor;
                }

                // If path length is greater than 2, the central node will always appear as intermediary
                if path.len() > 2 {
                    // Count intermediary nodes (excluding source and target)
                    for node in path.iter().skip(1).take(path.len() - 2) {
                        *betweenness_scores.entry(*node).or_insert(0.0) += 1.0;
                    }
                }
            }
        }
    }

    betweenness_scores
}

