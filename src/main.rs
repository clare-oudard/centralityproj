// Purpose: Run sampling-based centrality analysis (BFS and random) on a road network graph

mod subgraph;
mod utils;
use subgraph::extract_neighborhood_subgraph;
use petgraph::graph::{Graph, NodeIndex};
use std::collections::{HashMap, HashSet, VecDeque};
use rand::Rng;
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

/// Run BFS from a starting node and return distance to all reachable nodes.
/// Inputs graph and the starting node
/// Ouputs each node and its distance
fn bfs_closeness(graph: &Graph<(), (), petgraph::Undirected>, start: NodeIndex) -> HashMap<NodeIndex, usize> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    distances.insert(start, 0);

    // Explore neighbors level by level
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

/// Return the predecessors of each node in BFS, used to reconstruct shortest paths
/// Inputs graph and starting node
/// Outputs the node index and a vector of the nodes' predecessors
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

/// Compute closeness centrality for all nodes by inverting average shortest-path length.
/// If a node is isolated, score is 0.
/// Inputs graph
/// Outputs the node and its centrality index 
fn closeness_centrality(graph: &UndirectedGraph) -> HashMap<NodeIndex, f64> {
    let mut closeness_scores = HashMap::new();

    for node in graph.node_indices() {
        let distances = bfs_closeness(graph, node);

        if distances.len() == 1 {
            // Node is isolated
            closeness_scores.insert(node, 0.0);
            continue;
        }

        let total_distance: usize = distances.values().filter(|&&d| d > 0).sum();
        let reachable_nodes = distances.len() - 1;

        // Closeness is inverse of average distance
        let closeness = if total_distance > 0 {
            (reachable_nodes as f64) / (total_distance as f64)
        } else {
            0.0
        };

        closeness_scores.insert(node, closeness);
    }

    closeness_scores
}

/// Entry point of the program: load graph, sample nodes via BFS and randomly, compute and compare betweenness.
fn main() {
    let graph = load_graph_from_file("larger-sample.txt");
    let all_nodes: Vec<NodeIndex> = graph.node_indices().collect();

    // === Pick random starting point for BFS sampling ===
    let mut rng = thread_rng();
    let start_node = all_nodes[rng.gen_range(0..all_nodes.len())];
    println!("Starting at node: {}", start_node.index());

    // === Run BFS from that starting point ===
    let bfs_nodes = bfs_sample_nodes(&graph, start_node, 50);
    let bfs_scores = betweenness_centrality_sampled(&graph, &bfs_nodes);
    let bfs_normalized = normalize_scores(&bfs_scores);

    // Print top 5 by betweenness from BFS sample
    let mut bfs_top: Vec<_> = bfs_normalized.iter().collect();
    bfs_top.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    println!("\nTop 5 Most Central Nodes (BFS Sample from {}):", start_node.index());
    for (node, score) in bfs_top.iter().take(5) {
        println!("Node {}: {:.6}", node.index(), score);
    }

    // === Randomly sample 50 nodes (not neighborhood-based) ===
    let mut random_nodes = HashSet::new();
    while random_nodes.len() < 50 {
        let idx = rng.gen_range(0..all_nodes.len());
        random_nodes.insert(all_nodes[idx]);
    }
    let random_nodes: Vec<NodeIndex> = random_nodes.into_iter().collect();
    let rand_scores = betweenness_centrality_sampled(&graph, &random_nodes);
    let rand_normalized = normalize_scores(&rand_scores);

    // Print top 5 by betweenness from random sample
    let mut rand_top: Vec<_> = rand_normalized.iter().collect();
    rand_top.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    println!("\nTop 5 Most Central Nodes (Random Sample):");
    for (node, score) in rand_top.iter().take(5) {
        println!("Node {}: {:.6}", node.index(), score);
    }
}