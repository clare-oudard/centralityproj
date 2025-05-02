use petgraph::graph::{Graph, NodeIndex};
use std::collections::{HashMap, HashSet, VecDeque};

pub type UndirectedGraph = Graph<(), (), petgraph::Undirected>;

/// Extracts a subgraph of nodes within `depth` from `center`.
pub fn extract_neighborhood_subgraph(
    graph: &UndirectedGraph,
    center: NodeIndex,
    depth: usize,
) -> UndirectedGraph {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut subgraph = Graph::new_undirected();
    let mut node_map = HashMap::new(); // maps from original NodeIndex to subgraph NodeIndex

    queue.push_back((center, 0));
    visited.insert(center);

    while let Some((node, level)) = queue.pop_front() {
        if level > depth {
            continue;
        }

        let sub_node = *node_map
            .entry(node)
            .or_insert_with(|| subgraph.add_node(()));

        for neighbor in graph.neighbors(node) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back((neighbor, level + 1));
            }

            let sub_neighbor = *node_map
                .entry(neighbor)
                .or_insert_with(|| subgraph.add_node(()));

            if subgraph.find_edge(sub_node, sub_neighbor).is_none() {
                subgraph.add_edge(sub_node, sub_neighbor, ());
            }
        }
    }

    subgraph
}