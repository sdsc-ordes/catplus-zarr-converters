use zarrs::storage::ReadableWritableListableStorage;
use zarrs::filesystem::FilesystemStore; // requires filesystem feature
use std::path::PathBuf;

pub fn create_store(store_path: &PathBuf )-> Result<ReadableWritableListableStorage>{
    let filesystem = FilesystemStore::new(&store_path).expect("Failed to create filesystem store");

    let store: ReadableWritableListableStorage =
        Arc::new(filesystem);
    Ok(store)
}