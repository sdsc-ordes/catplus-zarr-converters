use crate::rdf::rdf_serializers::{serialize_graph_to_jsonld, serialize_graph_to_turtle};
use anyhow::{Context, Result};
use sophia::inmem::graph::LightGraph;
use sophia_api::prelude::*;
use sophia_api::term::SimpleTerm;

use super::insert_into::InsertIntoGraph;

/// An RDF Graph
pub struct GraphBuilder {
    pub graph: LightGraph,
}

/// Builds an RDF graph of Synthesis data for the cat+ ontology.
///
/// The rust structure `actions` in /parser/actions is mapped to the cat+ ontology
///
/// # public methods:
/// * insert:  starts the process of building the graph from the input structure
/// * serialize_to_turtle: serializes the graph to a turtle output
impl GraphBuilder {
    pub fn new() -> Self {
        Self { graph: LightGraph::new() }
    }

    /// Inserts a new object into the graph as a collection of triples.
    pub fn insert(&mut self, other: &dyn InsertIntoGraph) -> Result<()> {
        other.insert_into(&mut self.graph, other.get_uri())?;

        Ok(())
    }

    /// Materializes blank nodes in the graph by replacing them with URIs.
    /// If a prefix is given, it will be used for all materialized blank nodes.
    /// Otherwise, the empty string is used as the prefix.
    pub fn materialize_blank_nodes(&mut self, prefix: Option<&str>) -> Result<()> {
        let mut new_graph = LightGraph::new();

        for triple in self.graph.triples_matching(Any, Any, Any) {
            let [subject, predicate, object] = triple?;

            // If the subject is a blank node, replace it with a URI
            let new_subject = match subject {
                SimpleTerm::BlankNode(s) => {
                    let new_iri = format!("{}{}", prefix.unwrap_or_default(), s.as_str());
                    new_iri.to_owned()
                },
                SimpleTerm::Iri(s) => s.as_str().to_owned(),
                _ => panic!("Unexpected subject type")
            };

            // If the object is a blank node, replace it with a URI
            // In any other case, we just clone it
            match object {
                SimpleTerm::BlankNode(o) => {
                    let new_o = format!("{}{}", prefix.unwrap_or_default(), o.as_str());
                    new_graph.insert(
                        new_subject.as_simple(),
                        predicate.clone(),
                        new_o.as_simple()
                    )?;
                },
                _ => {
                    new_graph.insert(
                        new_subject.as_simple(),
                        predicate.clone(),
                        object.clone()
                    )?;
                },
            };
        }
                    

        self.graph = new_graph;
        Ok(())
    }

    /// Get the turtle serialization of the RDF graph
    ///
    /// Assumes a new graph has been created and built.
    ///
    /// # Returns
    /// A `Result` containing the graph as Turtle serialization, or an error
    /// if the graph retrieval fails.
    pub fn serialize_to_turtle(&self) -> Result<String> {
        serialize_graph_to_turtle(&self.graph).context("Failed to serialize graph to Turtle")
    }

    /// Get the turtle serialization of the RDF graph
    ///
    /// Assumes a new graph has been created and built.
    ///
    /// # Returns
    ///  The `jsonld` serialization of the grap, or an error otherwise.
    /// if the graph retrieval fails.
    pub fn serialize_to_jsonld(&self) -> Result<String> {
        serialize_graph_to_jsonld(&self.graph).context("Failed to serialize graph to JSON-LD")
    }
}
