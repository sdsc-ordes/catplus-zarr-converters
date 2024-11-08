use sophia::inmem::dataset::LightDataset;
use sophia::api::dataset::MutableDataset;
use sophia::iri::Iri;
use oxrdf::{Graph, Term, TripleRef};
use sophia::jsonld;

use std::io::{stdout, BufWriter};
use sophia::jsonld::{serializer::JsonLdSerializer, JsonLdOptions};
use sophia::api::prelude::*;
use sophia::api::source::StreamError::{SinkError, SourceError};

use std::fs::File;

pub fn serialize_to_jsonld(graph: &Graph, output_file: File) -> Result<(), Box<dyn std::error::Error>> {
    let convert_quads = convert_graph_to_quads(graph);
    match convert_quads {
        Ok(quads) => {
            let quads = quads;
        }
        Err(e) => {
            // Handle the error, printing it or taking other action
            eprintln!("Error converting graph to quads: {}", e);
        }
    }
    convert_quads_to_jsonld(&quads, &output_file);
    Ok(())
}


fn convert_graph_to_quads(graph: &Graph) -> Result<LightDataset, Box<dyn std::error::Error>> {
    let mut dataset = LightDataset::new();
    let graph_iri = Iri::new("http://example.org/graph1").unwrap();
    
    // TO DO : still understanding how to iterate through the graph and get the subject, predicate, object
    for triple in graph.iter() {
        let subject_str = match triple.subject() {
            Term::NamedNode(node) => node.as_str().to_string(),
            Term::BlankNode(bnode) => bnode.as_str().to_string(),
            _ => String::new(),
        };

        let predicate_str = match triple.predicate() {
            Term::NamedNode(node) => node.as_str().to_string(),
            _ => String::new(),
        };

        let object_str = match triple.object() {
            Term::NamedNode(node) => node.as_str().to_string(),
            Term::BlankNode(bnode) => bnode.as_str().to_string(),
            Term::Literal(literal) => literal.value().to_string(),
            _ => String::new(),
        };

        dataset.insert(subject_str, predicate_str, object_str, Some(graph_iri.clone()))?;

    }

    Ok(dataset)
}



// inspired from sophia example: https://github.com/pchampin/sophia_rs/blob/main/sophia/examples/serialize.rs

fn serialize_quads<Q: QuadSource, S: QuadSerializer>(
    quad_source: Q,
    mut ser: S,
) -> Result<(), String> {
    match ser.serialize_quads(quad_source) {
        Ok(_) => Ok(()),
        Err(SourceError(e)) => Err(format!("Error while parsing input: {e}")),
        Err(SinkError(e)) => Err(format!("Error while serializing quads: {e}")),
    }
}

fn convert_quads_to_jsonld(quad_source: &LightDataset, output_file: File) -> Result<(), Box<dyn std::error::Error>> {
    let out = BufWriter::new(output_file);
    serialize_quads(
        quad_source.quads(),
        JsonLdSerializer::new_with_options(
            out, 
            JsonLdOptions::new().with_spaces(2)
        ),
    );
    Ok(())
}


// use sophia::jsonld::JsonLdSerializer;
// use sophia::inmem::graph::LightGraph;
// use sophia::api::serializer::TripleSerializer;
// use std::io;

// pub fn jsonld_transform(graph: &LightGraph) -> Result<(), Box<dyn std::error::Error>> {
//     // Create JSON-LD serializer
//     let mut serializer = JsonLdSerializer::new(io::stdout());

//     // Serialize the graph as JSON-LD and write it to stdout
//     serializer.serialize_graph(&graph)?;

//     Ok(())
// }