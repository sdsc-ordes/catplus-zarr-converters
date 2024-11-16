use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

pub fn typed_example(json_data: &str) -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let p: Person = serde_json::from_str(json_data)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}
