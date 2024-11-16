mod parser;
use std::fs;
use crate::parser::person::Person;
use sophia::api::prelude::*;
use sophia::api::ns::Namespace;
use sophia::inmem::graph::LightGraph;
use sophia_turtle::serializer::turtle::TurtleSerializer;
use parser::parser::parse_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Grab JSON file
    let file_path = "data/test.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
    match parse_json(&contents) {
        Ok(person) => {
            let p: Person = person.clone();
            println!("Parsed person: {:?}", person);
            let mut g: LightGraph = LightGraph::new();
            let ex = Namespace::new("http://example.org/")?;
            let foaf = Namespace::new("http://xmlns.com/foaf/0.1/")?;
            let schema = Namespace::new("http://schema.org/")?;

            // Create a variable to store the transformed name (slug) for URI
            let person_name_slug = p.name.replace(" ", "_");
            
            // Create a variable to store the original name (with spaces) for insertion
            let person_name = p.name.clone();  // Use the original name
            
            // Use the transformed name for the URI
            let person_uri = ex.get(&person_name_slug)?;
            
            // Insert the original name into the graph (with spaces, "John Doe")
            g.insert(
                person_uri.clone(),
                foaf.get("name")?,
                person_name.as_str(),  // Insert the original name with spaces
            )?;

            g.insert(
                person_uri.clone(),
                foaf.get("age")?,
                p.age.to_string().as_str(),  // Assuming `p.age` is an integer or can be converted to a string
            )?;

            // Add phone numbers to the graph (use schema.org's telephone)
            for phone in &p.phones {
                g.insert(
                    person_uri.clone(),
                    schema.get("telephone")?,
                    phone.as_str(),  // Assuming `phone` is a String
                )?;
            }


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
