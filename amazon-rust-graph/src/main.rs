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

            // Build graph
            let (graph, asin_index_map) = build_graph(&products);
            println!("Graph built with {} nodes", graph.node_count());

            // Find products with most connections
            let highly_connected_asins = find_highly_connected_nodes(&graph, &asin_index_map);
            println!("Highly connected products:");
            for asin in highly_connected_asins {
                println!("{}", asin);
            }
        },
        Err(e) => println!("Error parsing file: {}", e),
    }
}
