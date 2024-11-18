mod parser;
mod graph;

use std::fs;
use crate::graph::graph_builder::GraphBuilder;
use parser::parser::parse_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read JSON file
    let file_path = "data/synth.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load the file.");

    // Parse json into Struct Batch
    match parse_json(&contents) {

        // json was parses successfully
        Ok(batch) => {
            // Initialize the graph builder
            let mut graph_builder = GraphBuilder::new()?;

            // Add Batch to the graph
            graph_builder.add_batch(&batch)?;

            // Serialize and print the graph
            let serialized_graph = graph_builder.serialize_to_turtle()?;
            println!("The resulting graph:\n{}", serialized_graph);

            Ok(())
        }
        Err(err) => {
            eprintln!("Error parsing JSON: {}", err);
            Err(Box::new(err)) // Return error as Result
        }
    }
}
