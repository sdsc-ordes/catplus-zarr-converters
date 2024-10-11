// use sophia::inmem::graph::FastGraph;  

use serde_json::json;


use sophia::api::dataset::MutableDataset;
use sophia::inmem::dataset::LightDataset;
use sophia::iri::Iri;
use sophia::jsonld::serializer::JsonLdSerializer; 
use sophia::api::prelude::QuadSerializer;
use sophia::api::prelude::Stringifier;
use sophia::jsonld::parser::JsonLdQuadSource;
use sophia::jsonld::parser::JsonLdParser;
use sophia::api::dataset::Dataset;
use sophia::api::prelude::QuadSource;
use sophia::api::parser::QuadParser;
use sophia::turtle::serializer::trig::{TrigConfig, TrigSerializer};

// rio is also a good library of rdf in rust 
// rdf tooling rust: https://github.com/oxigraph/oxigraph
// ideas : 
// From the metadata 1. parse json into graph (triples) 2. do some conversion to get jsonld to put into zarr
// From ontology: 1. parse ontology into graph (triples?) 2. Matching system with the metadata terms incoming


// Next step :
// Find how to read in ontology into turtle and then search for a term (one of the prefLabel that is also in metadata)
fn main() -> Result<(), Box<dyn std::error::Error>> {

    
    Ok(())
}

// Here we create an example dataset of quads
// we show how to extract these quads and print them

fn create_quad_dataset() -> Result<(), Box<dyn std::error::Error>> {
    let mut dataset = LightDataset::new();

    // Define graph IRIs (named graphs)
    let graph_iri = Iri::new("http://example.org/graph1").unwrap();

    // Insert quads into the dataset
    dataset.insert(
        "http://example.org/#Alice",
        "http://xmlns.com/foaf/0.1/name",
        "Alice",
        Some(&graph_iri)
    )?;

    dataset.insert(
        "http://example.org/#Alice",
        "http://xmlns.com/foaf/0.1/knows",
        "http://example.org/#Bob",
        Some(&graph_iri)
    )?;

    let mut quads: Vec<_> = dataset.quads().collect();

    for quad in quads {
        println!("Here is a quad: ");
        println!("{:?}", quad);
    }
}

// Here we create fake jsonld data and parse it into a dataset creating a QuadSource (easy way to stream quads)
// We then serialize the dataset into a string using the TrigSerializer 
// The output is the TriG representation of the dataset, human readable
// in comments there is a JsonSerializer attempt, but the output is difficult to manipulate
fn manipulate_jsonld() -> Result<(), Box<dyn std::error::Error>> {

    let jsonld_data = r#"
    {
        "@context": {
            "name": "http://schema.org/name",
            "homepage": "http://schema.org/url"
        },
        "@id": "http://example.org/alice",
        "name": "Alice",
        "homepage": "http://example.org/alice/home"
    }
    "#;
    let jsonld_parser = JsonLdParser::new();
    let mut quad_source = jsonld_parser.parse_str(jsonld_data);
    let mut dataset_2 = LightDataset::new();
    quad_source.add_to_dataset(&mut dataset_2);

   

    // // Use the JsonLdSerializer from sophia_jsonld to serialize the dataset

    let mut stringifier = TrigSerializer::new_stringifier_with_config(TrigConfig::new().with_pretty(true));
    let trig = stringifier.serialize_quads(dataset.quads())?;
    println!("{}", trig.as_str());

    // let mut serializer = JsonLdSerializer::new_jsonifier();
    // let jsonld_string =  serializer.serialize_dataset(&dataset_2);
    // what to do with this output?? does not go into serde_json so not good JSON? 

    Ok(())
}
