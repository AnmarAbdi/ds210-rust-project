use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;

// Store the product data here
pub struct Product {
    pub asin: String,
    pub title: String,
    pub similar: Vec<String>,
}

pub fn parse_file(file_path: &Path) -> io::Result<Vec<Product>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut products = Vec::new();
    let mut current_product: Option<Product> = None;
    // Loop through each line in the file
    for line in reader.lines() {
        let line = line?;
        
        if line.starts_with("Id:") {
            if let Some(product) = current_product.take() { 
                products.push(product);
            }
            current_product = Some(Product {
                asin: String::new(),
                title: String::new(),
                similar: Vec::new(),
            });
        } else if let Some(ref mut product) = current_product {
            if line.starts_with("ASIN:") {
                product.asin = line[6..].trim().to_string();
            } else if line.starts_with("  title:") {
                product.title = line[9..].trim().to_string();
            } else if line.starts_with("  similar:") {
                let parts: Vec<&str> = line[11..].trim().split_whitespace().collect();
                if let Ok(count) = parts[0].parse::<usize>() {
                    product.similar = parts.iter().skip(1).take(count).map(|s| s.to_string()).collect();
                }
            }
        }
    }

    // Logic to add the last product since theres no "Id:" line after to trigger it
    if let Some(product) = current_product {
        products.push(product);
    }

    Ok(products)
}
