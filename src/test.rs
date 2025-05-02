use super::*;
use petgraph::graph::{Graph, NodeIndex};
use crate::utils::{degree_centrality, normalize_scores, rank_top_n, bfs_sample_nodes, betweenness_centrality_sampled};

fn create_test_graph() -> Graph<(), (), petgraph::Undirected> {
    let mut graph = Graph::new_undirected();
    let a = graph.add_node(());
    let b = graph.add_node(());
    let c = graph.add_node(());
    let d = graph.add_node(());
    graph.add_edge(a, b, ());
    graph.add_edge(b, c, ());
    graph.add_edge(c, d, ());
    graph
}

#[test]
fn test_degree_centrality_simple() {
    let graph = create_test_graph();
    let degrees = degree_centrality(&graph);
    assert_eq!(degrees.get(&NodeIndex::new(0)).unwrap(), &0.3333333333333333);
    assert_eq!(degrees.get(&NodeIndex::new(1)).unwrap(), &0.6666666666666666);
}

#[test]
fn test_normalize_scores_range() {
    let mut scores = HashMap::new();
    scores.insert(NodeIndex::new(0), 5.0);
    scores.insert(NodeIndex::new(1), 10.0);
    let norm = normalize_scores(&scores);
    assert_eq!(norm[&NodeIndex::new(0)], 0.5);
    assert_eq!(norm[&NodeIndex::new(1)], 1.0);
}

#[test]
fn test_rank_top_n() {
    let mut scores = HashMap::new();
    scores.insert(NodeIndex::new(0), 5.0);
    scores.insert(NodeIndex::new(1), 15.0);
    scores.insert(NodeIndex::new(2), 10.0);
    let ranked = rank_top_n(&scores, 2);
    assert_eq!(ranked[0].0, NodeIndex::new(1));
    assert_eq!(ranked[1].0, NodeIndex::new(2));
}

#[test]
fn test_bfs_sample_nodes() {
    let graph = create_test_graph();
    let samples = bfs_sample_nodes(&graph, NodeIndex::new(0), 2);
    assert!(samples.len() <= 2);
    assert!(samples.contains(&NodeIndex::new(0)));
}

#[test]
fn test_betweenness_centrality_sampled() {
    let mut graph = Graph::<(), (), petgraph::Undirected>::new_undirected();
    let a = graph.add_node(());
    let b = graph.add_node(());
    let c = graph.add_node(());
    graph.add_edge(a, b, ());
    graph.add_edge(b, c, ());
    let nodes = vec![a];
    let scores = betweenness_centrality_sampled(&graph, &nodes);
    assert!(scores[&b] > 0.0);
}