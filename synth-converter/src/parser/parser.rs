use crate::parser::actions::Batch;

#[allow(dead_code)]
/// Parses a JSON string into a `Batch` struct.
///
/// The input JSON should conform to the structure defined in the `Batch` struct.
/// An example input file, `1-Synth.json`, is available in the `example` directory.
///
/// # Arguments
/// * `json_data` - A JSON string to be parsed.
///
/// # Returns
/// A `Result` containing the parsed `Batch` struct or a `serde_json::Error`
/// if parsing fails.
pub fn parse_json(json_data: &str) -> serde_json::Result<Batch> {
    let batch: Batch = serde_json::from_str(json_data)?;

    Ok(batch)
}
