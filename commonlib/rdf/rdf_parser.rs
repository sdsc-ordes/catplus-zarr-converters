use anyhow::Result;
use sophia::inmem::graph::LightGraph;
use sophia_api::{parser::TripleParser, prelude::TripleSource};
use sophia_turtle::parser::turtle::TurtleParser;

/// Parses a Turtle string into an RDF graph.
///
/// # Parameters
/// - `turtle_input`: The Turtle content as a string slice.
///
/// # Returns
/// - `Result<LightGraph>`: The parsed RDF graph on success, or an error on failure.
pub fn parse_turtle_to_graph(turtle_input: &str) -> Result<LightGraph> {
    // Initialize an RDF graph
    let mut graph = LightGraph::new();

    // Parse the Turtle input and populate the graph
    TurtleParser::default()
        .parse_str(turtle_input)
        .add_to_graph(&mut graph)
        .map_err(|e| anyhow::anyhow!("Failed to parse Turtle input: {}", e))?;

    Ok(graph)
}
