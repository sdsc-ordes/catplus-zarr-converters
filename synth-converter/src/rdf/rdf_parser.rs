use sophia::inmem::graph::LightGraph;
use sophia_api::{parser::TripleParser, prelude::TripleSource};
use sophia_turtle::parser::turtle::TurtleParser;

/// Parse Turtle input to an RDF graph
///
/// # Parameters
/// - `turtle_input`: A Turtle input content.
///
/// # Returns
/// A `Result` containing an RDF graph, or an error if parsing fails.
pub fn parse_turtle_to_graph(turtle_input: &str) -> Result<LightGraph, Box<dyn std::error::Error>> {
    // Create a mutable LightGraph to store parsed triples
    let mut graph = LightGraph::new();

    // Parse the Turtle input and populate the graph
    TurtleParser::default()
        .parse_str(turtle_input)
        .add_to_graph(&mut graph)?;

    Ok(graph)
}
