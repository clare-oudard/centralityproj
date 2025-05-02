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
    bfs_sample_nodes
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
    // === LOAD GRAPH ===
    let graph = load_graph_from_file("larger-sample.txt");
    let all_nodes: Vec<NodeIndex> = graph.node_indices().collect();
    let mut rng = thread_rng();

    // === RANDOM SAMPLING ANALYSIS ===
    let random_nodes: Vec<NodeIndex> = all_nodes
    .iter()
    .choose_multiple(&mut rng, 50.min(all_nodes.len()))
    .into_iter().cloned()
    .collect(); 

    let rand_b_scores = utils::betweenness_centrality_sampled(&graph, &random_nodes);
    let norm_rand_b = normalize_scores(&rand_b_scores);
    let rand_degrees = degree_centrality(&graph);

    println!("\n=== RANDOM SAMPLE RESULTS ===");
    println!("Node\tDegree\tBetweenness");
    for node in &random_nodes {
        let deg = rand_degrees.get(node).unwrap_or(&0.0);
        let bet = norm_rand_b.get(node).unwrap_or(&0.0);
        println!("{}\t{:.3}\t{:.3}", node.index(), deg, bet);
    }

    // === BFS SAMPLING COMPARISON ===
    let bfs_nodes = bfs_sample_nodes(&graph, all_nodes[0], 50);
    let bfs_b_scores = utils::betweenness_centrality_sampled(&graph, &bfs_nodes);
    let norm_bfs_b = normalize_scores(&bfs_b_scores);
    let bfs_degrees = degree_centrality(&graph);

    println!("\n=== BFS SAMPLE RESULTS ===");
    println!("Node\tDegree\tBetweenness");
    for node in &bfs_nodes {
        let deg = bfs_degrees.get(node).unwrap_or(&0.0);
        let bet = norm_bfs_b.get(node).unwrap_or(&0.0);
        println!("{}\t{:.3}\t{:.3}", node.index(), deg, bet);
    }

    // === TOP GLOBAL CENTRALITY NODES ===
    let top_rand_nodes = rank_top_n(&norm_rand_b, 5);
    println!("\nTop 5 Most Central Nodes (Random Sample):");
    for (node, score) in &top_rand_nodes {
        println!("Node {:?}: {:.6}", node.index(), score);
    }

    // === LOCAL SUBGRAPH ANALYSIS ===
    let center_node = top_rand_nodes[0].0;
    let subgraph = extract_neighborhood_subgraph(&graph, center_node, 2);
    let sub_nodes: Vec<NodeIndex> = subgraph.node_indices().collect();
    let sub_b = utils::betweenness_centrality_sampled(&subgraph, &sub_nodes);
    let sub_norm = normalize_scores(&sub_b);
    let sub_deg = degree_centrality(&subgraph);

}
