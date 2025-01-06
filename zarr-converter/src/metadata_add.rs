
use zarrs::group::{Group, GroupBuilder};
use zarrs::storage::ReadableWritableListableStorage;

use serde_json::{json, Value};
use std::collections::HashMap;


fn collect_metadata(store: &mut ReadableWritableListableStorage) -> Result<Value, Box<dyn std::error::Error>> {
    // Iterate over the entire store and collect metadata for all groups and arrays

    let group = Group::open(store, "/")?;
    let mut metadata = serde_json::Map::new();

    // Collect metadata for the group itself
    let group_attrs: serde_json::Map<std::string::String, Value> = group.attributes().clone();
    metadata.insert("group_attributes".to_string(), json!(group_attrs));

    // Collect metadata for arrays within the group
    let mut arrays_metadata = serde_json::Map::new();
    for array_name in group.array_names() {
        let array = group.open_array(&array_name)?;
        let array_attrs: HashMap<String, String> = array.attributes().read()?;
        
        arrays_metadata.insert(array_name.clone(), json!({
            "array_attributes": array_attrs,
            "shape": array.shape(),
            "dtype": format!("{:?}", array.dtype())
        }));
    }
    metadata.insert("arrays".to_string(), json!(arrays_metadata));

    // Recursively collect metadata for subgroups
    let mut groups_metadata = HashMap::new();
    for subgroup_name in group.group_names() {
        let subgroup = group.open_group(&subgroup_name)?;
        groups_metadata.insert(subgroup_name.clone(), collect_metadata(&subgroup)?);
    }
    metadata.insert("subgroups".to_string(), json!(groups_metadata));

    Ok(json!(metadata))
}