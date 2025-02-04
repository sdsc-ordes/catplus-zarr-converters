use crate::{
    graph::{
        namespaces::{
            alloproc, alloqual, allores, cat, obo, purl, qudt, schema,
            unit::{ToNsTerm, Unit},
        },
        utils::generate_bnode_term,
    },
    models::{
        Action, ActionName, Batch, Chemical, ContainerInfo, ContainerPositionQuantityItem,
        ErrorMargin, Observation, Sample, SampleItem,
    },
    rdf::rdf_serializers::{serialize_graph_to_jsonld, serialize_graph_to_turtle},
};
use anyhow::{Context, Result};
use sophia::{
    api::{
        graph::{Graph, MutableGraph},
        ns::{rdf, xsd},
    },
    inmem::graph::LightGraph,
};
use sophia_api::{ns::NsTerm, term::SimpleTerm, triple::Triple};

/// An RDF Graph
pub struct GraphBuilder {
    pub graph: LightGraph,
}

/// Builds an RDF graph of Synthesis data for the cat+ ontology.
///
/// The rust structure `actions` in /parser/actions is mapped to the cat+ ontology
///
/// # public methods:
/// * insert_a_batch:  starts the process of building the graph from the input structure
/// * serialize_to_turtle: serializes the graph to a turtle output
impl GraphBuilder {
    pub fn new() -> Self {
        Self {
            graph: LightGraph::new(),
        }
    }

    pub fn add_graph(&mut self, other: &LightGraph) -> Result<()> {
        for t in other.triples() {
            let t = t?;
            let spo = t.spo();
            self.graph.insert(spo[0], spo[1], spo[2])?;
        }
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
