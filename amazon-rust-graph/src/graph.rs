use petgraph::graph::{NodeIndex, DiGraph};
use crate::parser::Product;
use std::collections::HashMap;
use petgraph::visit::NodeIndexable;

pub fn build_graph(products: &[Product]) -> (DiGraph<String, ()>, HashMap<String, NodeIndex>) {
    let mut graph = DiGraph::<String, ()>::new();
    let mut asin_index_map = HashMap::new();

    // Node creation
    for product in products {
        let node_index = graph.add_node(product.asin.clone());
        asin_index_map.insert(product.asin.clone(), node_index);
    }

    // Edge creation
    for product in products {
        let product_node = asin_index_map[&product.asin];
        for similar_asin in &product.similar {
            if let Some(&similar_node) = asin_index_map.get(similar_asin) {
                graph.add_edge(product_node, similar_node, ());
            }
        }
    }

    (graph, asin_index_map)
}

pub fn find_highly_connected_nodes(graph: &DiGraph<String, ()>, asin_index_map: &HashMap<String, NodeIndex<u32>>) -> Vec<String> {
    let mut node_connections = graph.node_indices()
        .map(|node| (graph.to_index(node), graph.edges(node).count()))
        .collect::<Vec<(usize, usize)>>();

    // Sort nodes by top connections
    node_connections.sort_by(|a, b| b.1.cmp(&a.1));

    // Get top 10 connected products
    let top_connected = node_connections.iter()
        .take(10)
        .map(|&(index, _)| graph.node_weight(NodeIndex::new(index)).unwrap().clone())
        .collect::<Vec<String>>();

    top_connected
}
