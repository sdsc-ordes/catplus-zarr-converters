
use zarrs::array::{ArrayBuilder, DataType, FillValue, ZARR_NAN_F32};
use zarrs::array::codec::GzipCodec; // requires gzip feature
use zarrs::array_subset::ArraySubset;
use std::path::PathBuf;

use std::sync::Arc;

use zarr_converter::manage_store::create_store;
use zarr_converter::metadata_add::add_root_metadata;
use zarr_converter::chunk::{create_array, add_chunk, add_array_subset};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let store_path: PathBuf = "../zarr-files/hierarchy.zarr".into();
    let store;
    store = create_store(&store_path);
    add_root_metadata(&mut store);    

    let array;
    let path_in_store = "/array";
    let array_shape= vec![3,4];
    let json_metadata = serde_json::json!({"Zarr V3": "is great"});
    array = create_array(store, path_in_store, array_shape, json_metadata);
    
    // Store the array metadata
    array.store_metadata()?;
    // Consolidate metadata (get all metadata in a store)
    //To do write a function

    // CHUNKING
    let chunk_indices = [0, 0];
    let chunk_elements = [0.2, 0.3, 1.2, 1.3]
    array = add_chunk(array, chunk_indices, chunk)
    let array_indices = [1, 1];  
    let array_subset = ndarray::array![[-1.1, -1.2], [-2.1, -2.2]];
    array = add_array_subset(array, array_indices, array_subset)

    return Ok(());
}