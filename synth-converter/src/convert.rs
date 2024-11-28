use crate::{graph::graph_builder::GraphBuilder, parser::parser::parse_json};
use std::error::Error;

/// Parse JSON and serialize the RDF graph to turtle format
///
/// The input JSON should conform to the structure defined in the `Batch` struct.
/// An example input file, `1-Synth.json`, is available in the `example` directory.
///
/// # Returns
/// A `Result` containing the serialized graph as a string or an error if the process fails.
pub fn json_to_turtle(input_content: &str) -> Result<String, Box<dyn Error>> {

    let batch = parse_json(input_content)?;

    let mut graph_builder = GraphBuilder::new()?;
    graph_builder.insert_a_batch(&batch)?;
    let serialized_graph = graph_builder.serialize_to_turtle()?;

    Ok(serialized_graph)
}
