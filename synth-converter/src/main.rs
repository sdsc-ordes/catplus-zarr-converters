mod jsonld;
use jsonld::manip_jsonld;
mod ttl;
use ttl::search_ttl;
mod quads;
use quads::create_quad_dataset;

mod jsonmapper;
use jsonmapper::{read_csv_to_mapping, process_json_value};


use std::fs::File;
use std::io::{self, Read};
use serde_json::{Value};


fn main() -> io::Result<()> {
    // Read the input JSON file
    let mut json_file = File::open("data/synth-raw.json")?;
    let mut json_string = String::new();
    json_file.read_to_string(&mut json_string)?;

    let mut json_value: Value = serde_json::from_str(&json_string)?;
    let mapping = read_csv_to_mapping("data/mapping.csv")?;

    // Process the entire JSON structure
    process_json_value(&mut json_value, &mapping);

    // Save the processed JSON to a new file
    let output_file = File::create("data/output.json")?;
    serde_json::to_writer_pretty(output_file, &json_value)?;

    println!("Processed JSON has been saved to 'output.json'.");

    Ok(())
}



