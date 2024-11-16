use crate::parser::batch::Batch;

pub fn parse_json(json_data: &str) -> serde_json::Result<Batch> {
    // Parse the JSON string into a `Batch` object
    let batch: Batch = serde_json::from_str(json_data)?;

    // Log the parsed person object for debugging
    //println!("{:?}", batch);

    // Return the `batch` object
    Ok(batch)
}
