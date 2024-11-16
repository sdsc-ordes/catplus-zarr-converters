use serde_json::{Result, Value};

pub fn untyped_example(json_data: &str) -> Result<()> {
	let v: Value = serde_json::from_str(json_data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}
