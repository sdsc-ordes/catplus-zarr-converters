mod jsonmapper;
use jsonmapper::{read_csv_to_mapping, add_json_to_graph};

mod serializing;
use serializing::serialize_to_jsonld;

use std::fs::File;
use std::io::{self, Read};
use serde_json::Value;
use oxrdf::Graph;


fn main() -> io::Result<()> {
    // Read the input JSON file
    let mut json_file = File::open("data/synth-raw.json")?;
    let mut json_string = String::new();
    json_file.read_to_string(&mut json_string)?;
    let mut json_raw: Value = serde_json::from_str(&json_string)?;
    
    // Read the mapping CSV file
    let mapping = read_csv_to_mapping("data/mapping.csv")?;

    // Create Graph
    let mut graph = Graph::default();
    
    // Add JSON as Triples to RDF graph
    let add_graph_result = add_json_to_graph(&mut graph,
                                                                                &mut json_raw, 
                                                                                &mapping);
    match add_graph_result {
        Ok(added_graph) => {
            // TO DO : understand why it is unhappy with a mutable Graph here.
            graph = added_graph;
        }
        Err(e) => {
            // Handle the error, printing it or taking other action
            eprintln!("Error adding JSON to graph: {}", e);
        }
    }

    // Save the processed JSON to a new file
    let output_file = File::create("data/output.json")?;
    // Serialize graph to quads then to JSON-LD
    serialize_to_jsonld(&mut graph, output_file);

    println!("Raw json has been converted to JSON-LD to this file: 'output.json'.");

    Ok(())
}



