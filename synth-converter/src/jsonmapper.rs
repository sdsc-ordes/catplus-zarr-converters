use serde_json::{Value, Map};
use std::collections::HashMap;
use std::io::{self, Read};
use csv::ReaderBuilder;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

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

// Function to generate a random identifier string
fn generate_random_id() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect()
}

// Function to process the JSON object recursively, handling all levels of nesting
pub fn process_json_value(value: &mut Value, mapping: &HashMap<String, String>) {
    match value {
        Value::Object(obj) => process_json_object(obj, mapping),
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                process_json_value(item, mapping);
            }
        }
        _ => {}
    }
}

// Function to process JSON objects, modify them, and ensure new keys stay at the appropriate level
fn process_json_object(
    json_obj: &mut Map<String, Value>,
    mapping: &HashMap<String, String>
) {
    let mut new_entries = Map::new();

    for (key, value) in json_obj.clone() {
        if let Some(mapped_url) = mapping.get(&key) {
            let mut mapped_value = json_obj.remove(&key).unwrap();

            if key.starts_with("has") {
                // If the value is an array, create identifiers and insert them at the current level
                if let Value::Array(arr) = &mut mapped_value {
                    let mut identifiers = Vec::new();
                    for item in arr.drain(..) {
                        if let Value::Object(obj) = item {
                            let random_id = generate_random_id();
                            identifiers.push(Value::String(random_id.clone()));
                            new_entries.insert(random_id, Value::Object(obj));
                        }
                    }
                    json_obj.insert(mapped_url.clone(), Value::Array(identifiers));
                } else {
                    // Generate a new identifier for non-array values and add at the current level
                    let random_id = generate_random_id();
                    new_entries.insert(random_id.clone(), mapped_value.clone());
                    json_obj.insert(mapped_url.clone(), Value::String(random_id));
                }
            } else {
                json_obj.insert(mapped_url.clone(), mapped_value);
            }
        } else {
            // Recursively process nested objects or arrays
            process_json_value(&mut json_obj.get_mut(&key).unwrap(), mapping);
        }
    }

    // Add new entries to the JSON object at the current level
    for (key, value) in new_entries {
        json_obj.insert(key, value);
    }
}


