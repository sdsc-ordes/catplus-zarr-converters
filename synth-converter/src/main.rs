mod parser;
use std::fs;
use crate::parser::batch::Batch;
use sophia::api::prelude::*;
use sophia::api::ns::Namespace;
use sophia::inmem::graph::LightGraph;
use sophia_turtle::serializer::turtle::TurtleSerializer;
use parser::parser::parse_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Grab JSON file
    let file_path = "data/synth.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
    match parse_json(&contents) {
        Ok(batch) => {
            let b: Batch = batch.clone();
            //println!("Parsed batch: {:?}", batch);

            // Create namespaces
            let mut g: LightGraph = LightGraph::new();
            let ex = Namespace::new("http://example.org/")?;
            let allores = Namespace::new("http://purl.allotrope.org/ontologies/result")?;

            // Add batch-level triple
            let batch_uri = ex.get(&b.batch_id)?;
            g.insert(
                batch_uri.clone(),
                allores.get("AFR_0001120")?,
                b.batch_id.as_str(),
            )?;

            // Uncomment and fix serialization if needed
            let mut nt_stringifier = TurtleSerializer::new_stringifier();
            let example2 = nt_stringifier.serialize_graph(&g)?.as_str().to_string();
            println!("The resulting graph:\n{}", example2);  

            Ok(())      
        }
        Err(err) => {
            eprintln!("Error parsing JSON: {}", err);
            return Err(Box::new(err));  // Return error as Result
        }
    } 
}
