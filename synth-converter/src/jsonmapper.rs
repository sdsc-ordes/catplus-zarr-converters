use serde_json::{Value, Map};
use sophia::inmem::dataset::LightDataset;
use std::collections::HashMap;
use std::io::{self};
use csv::ReaderBuilder;

use oxrdf::{NamedNodeRef, SubjectRef};

mod graphinsert;
use graphinsert::insert_triple;

// Function to read and parse CSV into a HashMap, extracting only specific columns
pub fn read_csv_to_mapping(file_path: &str) -> io::Result<HashMap<String, String>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;
    let mut mapping = HashMap::new();

    // Get the headers to find the correct columns
    let headers = rdr.headers()?.clone();
    let name_col_index = headers.iter().position(|h| h == "Name of property/object");
    let link_col_index = headers.iter().position(|h| h == "Link for ontology");

    // Check if both columns exist
    if let (Some(name_idx), Some(link_idx)) = (name_col_index, link_col_index) {
        for result in rdr.records() {
            let record = result?;
            let key = record.get(name_idx).unwrap_or("").to_string();
            let value = record.get(link_idx).unwrap_or("").to_string();
            if !key.is_empty() && !value.is_empty() {
                mapping.insert(key, value);
            }
        }
    } else {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Required columns not found in CSV"));
    }

    Ok(mapping)
}

pub fn add_json_to_graph<'graph>(
    graph: &'graph mut oxrdf::Graph,
    json_obj: &mut Value,
    mapping: &HashMap<String, String>
) -> Result<&'graph mut oxrdf::Graph , Box<dyn std::error::Error>>
{
    // TO DO : How can we switch back to the subject being the Action after defining the hasSample ? 
    // TO DO: Does this accomodate nested JSON ? Probably not? 
    for (key, value) in json_obj.as_object().unwrap() {
        let triple_subject;
        if key == "action_name"{
            // value will be something like AddAction, StirAction etc. 
            let triple_subject = mapping.get(&value.to_string()).unwrap();
        } else if key == "hasSample" {
            let triple_subject = mapping.get("Sample");
        } else if key =="hasChemical"{
            let triple_subject = mapping.get("Chemical");
        }
        let triple_predicate = NamedNodeRef::new(mapping.get(&key).unwrap()).unwrap();
        let triple_object = NamedNodeRef::new(&value.to_string()).unwrap();
        insert_triple(graph, triple_subject, triple_predicate, triple_object);
    }    
    Ok(graph)
}



