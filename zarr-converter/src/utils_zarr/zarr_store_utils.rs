use zarrs::storage::ReadableWritableListableStorage;
use zarrs::filesystem::FilesystemStore;
use std::path::PathBuf;
use std::sync::Arc;

pub fn create_store(store_path: &PathBuf )-> Result<ReadableWritableListableStorage, Box<dyn std::error::Error>>{
    let filesystem = FilesystemStore::new(&store_path).expect("Failed to create filesystem store");

    let store: ReadableWritableListableStorage = Arc::new(filesystem);
    Ok(store)
}