mod subgraph;
mod utils;
use subgraph::extract_neighborhood_subgraph;
use petgraph::graph::{Graph, NodeIndex};
use std::collections::{HashMap, HashSet, VecDeque};
use rand::seq::IteratorRandom;
use rand::thread_rng;
use utils::{
    load_graph_from_file,
    normalize_scores,
    rank_top_n,
    degree_centrality,
    bfs_sample_nodes,
    betweenness_centrality_sampled
};
#[cfg(test)]
mod test;



fn bfs_closeness(graph: &Graph<(), (), petgraph::Undirected>, start: NodeIndex) -> HashMap<NodeIndex, usize> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    distances.insert(start, 0);

    while let Some((node, distance)) = queue.pop_front() {
        for neighbor in graph.neighbors(node) {
            if !distances.contains_key(&neighbor) {
                distances.insert(neighbor, distance + 1);
                queue.push_back((neighbor, distance + 1));
            }
        }
    }
    
    distances
}

fn bfs_paths(graph: &Graph<(), (), petgraph::Undirected>, start: NodeIndex) -> HashMap<NodeIndex, Vec<NodeIndex>> {
    let mut predecessors = HashMap::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(start);
    visited.insert(start);

    while let Some(node) = queue.pop_front() {
        for neighbor in graph.neighbors(node) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
                predecessors.entry(neighbor).or_insert_with(Vec::new).push(node);
            }
        }
    }

    predecessors
}

type UndirectedGraph = Graph<(), (), petgraph::Undirected>;

fn closeness_centrality(graph: &UndirectedGraph) -> HashMap<NodeIndex, f64> { 
    let mut closeness_scores = HashMap::new();

    for node in graph.node_indices() {
        let distances = bfs_closeness(graph, node);

        // If there are no neighbors or the node is isolated, set closeness to 0
        if distances.len() == 1 {
            closeness_scores.insert(node, 0.0);
            continue;
        }

        // Exclude the node's distance to itself and count only reachable nodes
        let total_distance: usize = distances.values().filter(|&&dist| dist > 0).sum();
        let reachable_nodes = distances.len() - 1; // Exclude the node itself

        // If there are reachable nodes and the total distance is non-zero, compute closeness
        let closeness = if reachable_nodes > 0 && total_distance > 0 {
            (reachable_nodes as f64) / (total_distance as f64)
        } else {
            0.0 // For isolated nodes or nodes with no reachable nodes
        };

        closeness_scores.insert(node, closeness);
    }

    closeness_scores
}

fn main() {
    let graph = load_graph_from_file("larger-sample.txt");

    // Compute closeness centrality for all nodes
    let closeness_scores = closeness_centrality(&graph);
    println!("\nCloseness Centrality (sampled):");
    for node in graph.node_indices().take(10) {
        if let Some(score) = closeness_scores.get(&node) {
            println!("Node {}: {:.6}", node.index(), score);
        }
    }

    // Take the same 10 nodes for sampled betweenness centrality
    let sample_nodes: Vec<NodeIndex> = graph.node_indices().take(10).collect();
    let b_scores = betweenness_centrality_sampled(&graph, &sample_nodes);
    let normalized = normalize_scores(&b_scores);

    println!("\nNormalized Betweenness Centrality (Sampled):");
    for (node, score) in &normalized {
        println!("Node {}: {:.6}", node.index(), score);
    }

    let mut top: Vec<_> = normalized.iter().collect();
    top.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    println!("\nTop 5 Most Central Nodes (by Betweenness):");
    for (node, score) in top.iter().take(5) {
        println!("Node {}: {:.6}", node.index(), score);
    }
}