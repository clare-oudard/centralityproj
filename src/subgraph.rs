// Purpose: Provides a function to extract a subgraph centered around a node using BFS up to a given depth.

use petgraph::graph::{Graph, NodeIndex};
use std::collections::{HashMap, HashSet, VecDeque};

/// A type alias for undirected graphs used throughout the project.
pub type UndirectedGraph = Graph<(), (), petgraph::Undirected>;

/// Extracts a neighborhood subgraph of a node within a certain BFS depth.
/// Inputs `graph`: the original full graph and `center`: the node to center the subgraph around and `depth`: the number of hops to include
/// Outputs A new subgraph containing all nodes and edges up to `depth` hops away from `center`
/// Performs a BFS traversal up to `depth`, copying each node and edge into a new graph.
pub fn extract_neighborhood_subgraph(
    graph: &UndirectedGraph,
    center: NodeIndex,
    depth: usize,
) -> UndirectedGraph {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut subgraph = Graph::new_undirected();

    // Maps original graph's NodeIndex to subgraph's NodeIndex
    let mut node_map = HashMap::new();

    // Begin BFS from center node
    queue.push_back((center, 0));
    visited.insert(center);

    while let Some((node, level)) = queue.pop_front() {
        if level > depth {
            continue; // Stop expanding once max depth is reached
        }

        // Add node to subgraph if not already present
        let sub_node = *node_map
            .entry(node)
            .or_insert_with(|| subgraph.add_node(()));

        for neighbor in graph.neighbors(node) {
            // Only continue BFS for unvisited neighbors
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back((neighbor, level + 1));
            }

            // Add the neighbor node to the subgraph
            let sub_neighbor = *node_map
                .entry(neighbor)
                .or_insert_with(|| subgraph.add_node(()));

            // Only add edge if it doesn’t already exist in the subgraph
            if subgraph.find_edge(sub_node, sub_neighbor).is_none() {
                subgraph.add_edge(sub_node, sub_neighbor, ());
            }
        }
    }

    subgraph
}