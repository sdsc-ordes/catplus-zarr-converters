use sophia::{
    api::{
        dataset::{Dataset, MutableDataset},
        parser::QuadParser,
        prelude::{QuadSerializer, QuadSource, Stringifier},
    },
    inmem::dataset::LightDataset,
    iri::Iri,
    jsonld::parser::JsonLdParser,
    turtle::serializer::trig::{TrigConfig, TrigSerializer},
};

// Import common and define an alias;
mod common;
use common::utils::foo as fa;

mod jsonld;

// rio is also a good library of rdf in rust
// rdf tooling rust: https://github.com/oxigraph/oxigraph
// ideas :
// From the metadata 1. parse json into graph (triples) 2. do some conversion to get jsonld to put into zarr
// From ontology: 1. parse ontology into graph (triples?) 2. Matching system with the metadata terms incoming

// Next step :
// Find how to read in ontology into turtle and then search for a term (one of the prefLabel that is also in metadata)

// use rio_turtle::{TurtleParser};
// use rio_api::parser::TriplesParser;
// use rio_api::model::*;
use oxrdf::{vocab::rdf, NamedNodeRef};
use oxttl::TurtleParser;
use std::{fs::File, io::BufReader};

fn main() {
    fa();
    common::utils::foo();
    common::aliased_name_for_utils::foo();
}

// Here we read in a turtle file and search for a specific term
// file and search term are hardcoded for now
fn search_ttl() -> Result<(), Box<dyn std::error::Error>> {
    // // Path to your Turtle file, relative to the location of main.rs
    let file_path = "../data/ontology.ttl"; // Change this to your actual file path

    // Open the Turtle file for reading
    let file = File::open(file_path).expect("Failed to open Turtle file");
    let reader = BufReader::new(file);
    let search_item = "http://www.w3.org/2004/02/skos/core#prefLabel";
    let search_schema = NamedNodeRef::new(search_item).unwrap();
    let mut count = 0;
    for triple in TurtleParser::new().for_reader(reader) {
        //println!("{:?}", triple);
        let triple = triple.unwrap();
        //can be triple subject predicate or object
        if triple.predicate == search_schema {
            count += 1;
        }
    }
    println!(
        "Found {} triples with predicate rdf:type and object {}",
        count, search_item
    );
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
        Some(&graph_iri),
    )?;

    dataset.insert(
        "http://example.org/#Alice",
        "http://xmlns.com/foaf/0.1/knows",
        "http://example.org/#Bob",
        Some(&graph_iri),
    )?;

    let quads: Vec<_> = dataset.quads().collect();

    for quad in quads {
        println!("Here is a quad: ");
        println!("{:?}", quad);
    }
    Ok(())
}

// Here we create fake jsonld data and parse it into a dataset creating a QuadSource (easy way to stream quads)
// We then serialize the dataset into a string using the TrigSerializer
// The output is the TriG representation of the dataset, human readable
// in comments there is a JsonSerializer attempt, but the output is difficult to manipulate
fn manip_jsonld() -> Result<(), Box<dyn std::error::Error>> {
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
    let quad_source = jsonld_parser.parse_str(jsonld_data);
    let mut dataset_2 = LightDataset::new();
    quad_source.add_to_dataset(&mut dataset_2);

    // // Use the JsonLdSerializer from sophia_jsonld to serialize the dataset

    let mut stringifier =
        TrigSerializer::new_stringifier_with_config(TrigConfig::new().with_pretty(true));
    let trig = stringifier.serialize_quads(dataset_2.quads())?;
    println!("{}", trig.as_str());

    // let mut serializer = JsonLdSerializer::new_jsonifier();
    // let jsonld_string =  serializer.serialize_dataset(&dataset_2);
    // what to do with this output?? does not go into serde_json so not good JSON?

    Ok(())
}
