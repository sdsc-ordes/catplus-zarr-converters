use crate::{graph::graph_builder::GraphBuilder, parser::parser::parse_json};
use std::error::Error;

/// Parse JSON and serialize the RDF graph to the specified format
///
/// The input JSON should conform to the structure defined in the `Batch` struct.
/// An example input file, `1-Synth.json`, is available in the `example` directory.
///
/// # Arguments
/// - `input_content`: The JSON input as a string.
/// - `fmt`: The desired serialization format ("turtle" or "jsonld").
///           If unspecified or empty, defaults to "turtle".
///
/// # Returns
/// A `Result` containing the serialized graph as a string or an error if the process fails.
pub fn json_to_rdf(input_content: &str, fmt: &str) -> Result<String, Box<dyn Error>> {
    let batch = parse_json(input_content)?;

    let mut graph_builder = GraphBuilder::new()?;
    graph_builder.insert_a_batch(&batch)?;

    let serialized_graph = match fmt {
        "jsonld" => graph_builder.serialize_to_jsonld()?,
        _ => graph_builder.serialize_to_turtle()?, // Default to Turtle
    };

    Ok(serialized_graph)
}
