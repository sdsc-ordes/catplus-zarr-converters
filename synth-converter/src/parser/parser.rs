use crate::parser::person::Person;
use serde_json::Result;

pub fn typed_example(json_data: &str) -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let person: Person = serde_json::from_str(json_data)?;

    // Do things just like with any other Rust data structure.
    println!("{:?}", person);

    Ok(())
}
