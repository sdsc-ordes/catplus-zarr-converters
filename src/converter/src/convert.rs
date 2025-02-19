use anyhow::{Context, Result};
use catplus_common::graph::{graph_builder::GraphBuilder, insert_into::InsertIntoGraph};
use serde::{de::DeserializeOwned, Deserialize};

// Derive Deserialize and ValueEnum
#[derive(Deserialize, Debug, clap::ValueEnum, Clone)]
pub enum RdfFormat {
    Turtle,
    Jsonld,
}

/// Parses JSON and serializes the RDF graph to the specified format.
///
/// This function can handle any struct that implements `serde::DeserializeOwned` and your `InsertIntoGraph` trait.
///
/// # Arguments
/// - `input_content`: The JSON input as a string.
/// - `format`: The desired serialization format.
///
/// # Returns
/// A `Result` containing the serialized graph as a string or an error.
pub fn json_to_rdf<T>(input_content: &str, format: &RdfFormat) -> Result<String>
where
    T: DeserializeOwned + InsertIntoGraph, // Trait bounds
{
    let data: T = parse_json(input_content).context("Failed to parse JSON input")?;

    let mut graph_builder = GraphBuilder::new();
    graph_builder.insert(&data).context("Failed to build RDF graph")?;

    let serialized_graph = match format {
        RdfFormat::Jsonld => {
            graph_builder.serialize_to_jsonld().context("Failed to serialize to JSON-LD")?
        }
        RdfFormat::Turtle => {
            graph_builder.serialize_to_turtle().context("Failed to serialize to Turtle")?
        }
    };

    Ok(serialized_graph)
}

/// Parses a JSON string into a struct of type T.
fn parse_json<T>(json_data: &str) -> Result<T>
where
    T: DeserializeOwned, // Trait bound
{
    serde_json::from_str(json_data).map_err(|e| anyhow::Error::new(e))
}
