
use zarrs::group::GroupBuilder;
use zarrs::storage::ReadableWritableListableStorage;


pub fn add_root_metadata(store: &mut ReadableWritableListableStorage){
    // // Write the root group metadata
    GroupBuilder::new().build(store.clone(), "/")?.store_metadata()?;

}
pub fn add_metadata(fmt: &str) -> Result<String> {
}