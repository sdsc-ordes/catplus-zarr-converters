use std::fs::File;
use std::io::BufReader;
use oxrdf::{NamedNodeRef, vocab::rdf};
use oxttl::TurtleParser;

// Here we read in a turtle file and search for a specific term
// file and search term are hardcoded for now
pub fn search_ttl()  -> Result<(), Box<dyn std::error::Error>> {
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
        if triple.predicate == search_schema{
            count += 1;
        }
    }
    println!("Found {} triples with predicate rdf:type and object {}", count, search_item);
    Ok(())
} 