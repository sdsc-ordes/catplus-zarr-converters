use crate::parser::actions::Batch;

pub fn parse_json(json_data: &str) -> serde_json::Result<Batch> {
    let batch: Batch = serde_json::from_str(json_data)?;
    Ok(batch)
}
