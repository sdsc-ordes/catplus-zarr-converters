
use zarrs::group::{self, GroupBuilder};
use zarrs::storage::ReadableWritableListableStorage;
use serde_json::{Value, Map};


pub fn add_root_group(store: &mut ReadableWritableListableStorage) -> Result<(), Box<dyn std::error::Error>>{
    // // Write the root group metadata
    GroupBuilder::new().build(store.clone(), "/")?.store_metadata()?;
    Ok(())
}

pub fn create_group(store: &mut ReadableWritableListableStorage, group_path: &str, metadata: Map<String, Value> )-> Result<(), Box<dyn std::error::Error>>{
    let _group = zarrs::group::GroupBuilder::new()
    .attributes(metadata)
    .build(store.clone(), group_path)?;

    _group.store_metadata()?;
    Ok(())
}