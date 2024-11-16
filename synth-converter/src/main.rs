use std::fs;
mod parser;
use sophia::api::prelude::*;
use sophia::api::ns::Namespace;
use sophia::inmem::graph::LightGraph;
use sophia::turtle::parser::turtle;
use sophia_turtle::serializer::turtle::TurtleSerializer;
use parser::parser::typed_example;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    // Grab JSON file
    let file_path = "data/test.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");

    match typed_example(&contents) {
        Ok(person) => {
            println!("Parsed person: {:?}", person);
        }
        Err(err) => {
            eprintln!("Error parsing JSON: {}", err);
        }
    } 
    // Loading a graph
    let example = r#"
        @prefix : <http://example.org/>.
        @prefix foaf: <http://xmlns.com/foaf/0.1/>.
        :alice foaf:name "Alice";
               foaf:mbox <mailto:alice@work.example> .
        :bob foaf:name "Bob".
    "#;
    let mut graph: LightGraph = turtle::parse_str(example).collect_triples()?;
    
    // Mutating the graph
    let ex = Namespace::new("http://example.org/")?;
    let foaf = Namespace::new("http://xmlns.com/foaf/0.1/")?;
    graph.insert(
        ex.get("bob")?,
        foaf.get("knows")?,
        ex.get("alice")?,
    )?;
    
    // Serializing the graph
    let mut nt_stringifier = TurtleSerializer::new_stringifier();
    let example2 = nt_stringifier.serialize_graph(&graph)?.as_str().to_string();
    println!("The resulting graph:\n{}", example2);

    Ok(())
}
