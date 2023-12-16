mod parser;
mod graph;

use std::path::Path;
use graph::{build_graph, find_highly_connected_nodes};

fn main() {
    let file_path = Path::new("amazon-meta.txt");

    // Parse file
    match parser::parse_file(&file_path) {
        Ok(products) => {
            println!("Parsed {} products", products.len());

            // Build graph (Ignore asin index map for now)
            let (graph, _asin_index_map) = build_graph(&products);
            println!("Graph built with {} nodes and {} edges.", graph.node_count(), graph.edge_count());

            // Find products with most connections
            let highly_connected = find_highly_connected_nodes(&graph);
            println!("Highly connected products:");
            for (asin, title, num_connections) in highly_connected {
                println!("ASIN: {}, Title: {}, Connections: {}", asin, title, num_connections);
            }
            
            let degree_distribution = graph::analyze_degree_distribution(&graph);
            println!("Degree Distribution:");
            let mut degrees: Vec<_> = degree_distribution.keys().collect();
            degrees.sort();
            
            for degree in degrees {
                let (count, percentage) = degree_distribution[degree];
                println!("Degree: {}, Count: {}, Percentage: {:.2}%", degree, count, percentage);
            }        
        },
        Err(e) => println!("Error parsing file: {}", e),
    }
}
