use crate::graph::namespaces::generate_prefix_map;
use sophia::api::serializer::{Stringifier, TripleSerializer};
use sophia::inmem::graph::LightGraph;
use sophia_turtle::serializer::turtle::{TurtleConfig, TurtleSerializer};


/// Serialize an RDF graph to Turtle format
///
/// # Parameters
/// - `graph`: A reference to the graph to be serialized.
///
/// # Returns
/// A `Result` containing the Turtle serialization as a `String`, or an error if serialization fails.
pub fn serialize_graph_to_turtle(
    graph: &LightGraph,
) -> Result<String, Box<dyn std::error::Error>> {

    let prefix_map = generate_prefix_map();

    let config = TurtleConfig::default()
        .with_pretty(true)
        .with_own_prefix_map(prefix_map);

    let mut serializer = TurtleSerializer::new_stringifier_with_config(config);
    serializer.serialize_graph(graph)?;

    Ok(serializer.as_str().to_string())
}
