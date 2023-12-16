use petgraph::graph::{NodeIndex, DiGraph};
use crate::parser::Product;
use std::collections::HashMap;

pub fn build_graph(products: &[Product]) -> (DiGraph<(String, String), ()>, HashMap<String, NodeIndex<u32>>) {
    let mut graph = DiGraph::<(String, String), ()>::new();
    let mut asin_index_map = HashMap::new();

    // Node creation: Store both ASIN and title as node weights
    for product in products {
        let node_index = graph.add_node((product.asin.clone(), product.title.clone()));
        asin_index_map.insert(product.asin.clone(), node_index);
    }

    // Edge creation: Create edges based on similar products
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

pub fn find_highly_connected_nodes(graph: &DiGraph<(String, String), ()>) -> Vec<(String, String, usize)> {
    let mut node_connections = graph.node_indices()
        .map(|node_idx| {
            let (asin, title) = &graph[node_idx];
            let degree = graph.edges(node_idx).count();
            (asin.clone(), title.clone(), degree)
        })
        .collect::<Vec<(String, String, usize)>>();

    node_connections.sort_by(|a, b| b.2.cmp(&a.2)); // Sort by top connection count

    node_connections.into_iter().take(5).collect()
}

pub fn analyze_degree_distribution(graph: &DiGraph<(String, String), ()>) -> HashMap<usize, (usize, f64)> {
    let mut degree_counts = HashMap::new();
    let total_nodes = graph.node_count();

    // Calculate degree for each node and tally them
    for node_idx in graph.node_indices() {
        let degree = graph.edges(node_idx).count();
        *degree_counts.entry(degree).or_insert(0) += 1;
    }

    // Create a map for degree, count, and percentage
    let mut degree_distribution = HashMap::new();
    for (degree, count) in degree_counts.iter() {
        let percentage = (*count as f64 / total_nodes as f64) * 100.0;
        degree_distribution.insert(*degree, (*count, percentage));
    }

    degree_distribution
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Product;
    use float_cmp::approx_eq;

    #[test] // Test that the graph gets built correctly
    fn test_build_graph() {
        let products = vec![
            Product {
                asin: "123".to_string(),
                title: "Product 123".to_string(),
                similar: vec!["456".to_string()],
            },
            Product {
                asin: "456".to_string(),
                title: "Product 456".to_string(),
                similar: vec!["123".to_string()],
            },
        ];

        let (graph, _) = build_graph(&products);

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 2); // Assuming bidirectional edges
    }


    #[test] // Test degree distribution
    fn test_analyze_degree_distribution() {
        let products = vec![
            Product {
                asin: "123".to_string(),
                title: "Product 123".to_string(),
                similar: vec![], // No connections
            },
            Product {
                asin: "456".to_string(),
                title: "Product 456".to_string(),
                similar: vec!["123".to_string()], // One connection to "123"
            },
        ];
    
        let (graph, _) = build_graph(&products);
        let degree_distribution = analyze_degree_distribution(&graph);
    
        // "123" should have a degree of 1 because only "456" points to it
        assert!(degree_distribution.contains_key(&1));
    }

    #[test]
    fn test_percentage_sum_in_degree_distribution() {
        // Create test data
        let products = vec![
            Product {
                asin: "A".to_string(),
                title: "Book A".to_string(),
                similar: vec!["B".to_string()], // One connection
            },
            Product {
                asin: "B".to_string(),
                title: "Book B".to_string(),
                similar: vec!["A".to_string(), "C".to_string()], // Two connections
            },
            Product {
                asin: "C".to_string(),
                title: "Book C".to_string(),
                similar: vec![], // No connections
            },
            Product {
                asin: "D".to_string(),
                title: "Book D".to_string(),
                similar: vec!["A".to_string(), "B".to_string()], // Two connections
            },
        ];

        // Build the graph and analyze the degree distribution
        let (graph, _) = build_graph(&products);
        let degree_distribution = analyze_degree_distribution(&graph);

        // Calculate the sum of the percentages
        let percentage_sum: f64 = degree_distribution.values()
            .map(|&(_, percentage)| percentage)
            .sum();

        // Check if the sum is approximately 100%
        assert!(approx_eq!(f64, percentage_sum, 100.0, epsilon = 0.01));
    }
}