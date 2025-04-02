/// Interface for validating an RDF graph.

use sophia_api::prelude::*;
use sophia::inmem::graph::LightGraph;
use std::error::Error;


pub mod shacl_api;

/// A SHACL validation report for an RDF graph.
#[derive(Clone, Debug)]
pub struct ValidationReport {
    pub conforms: bool,
    pub graph: LightGraph,
    
    //TODO: Summary of severity counts.
    //pub summary: HashMap<Severity, u32>,
}

impl ValidationReport {
    pub fn new(conforms: bool, graph: LightGraph) -> Self {
        ValidationReport { conforms, graph }
    }

    pub fn from_graph(graph: LightGraph) -> Self {
        
        // NOTE: Only looks at value of the first sh:Conforms triple
        // not found -> not conform
        let conforms = graph
            .triples_matching(
                Any,
                ["http://www.w3.org/ns/shacl#conforms".as_simple()],
                Any,
            ).map(|t| t.map_or(false, |t| t[2].lexical_form().unwrap() == "true")).next().unwrap_or(false);

                   
        ValidationReport { conforms, graph }
    }
}

/// Interface for a SHACL validation engine.
pub trait ShaclEngine {
    fn is_available(&self) -> bool;
    fn validate(&self, data: &LightGraph, shapes: Option<&LightGraph>) -> Result<ValidationReport, Box<dyn Error>>;
    
    // TODO: SHACL inference
    // fn infer(&self, data: &LightGraph, rules: Option<&LightGraph>) -> Result<LightGraph, Box<dyn Error>>;
}

