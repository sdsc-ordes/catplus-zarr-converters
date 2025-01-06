use std::path::PathBuf;
use zarrs::group::Group;

use crate::manage_store::create_store;
use crate::chunk::{add_chunk, erase_chunk};
use crate::metadata_add::{add_root_metadata, add_array_metadata, add_group_metadata, collect_metadata};
use crate::manage_arrays::{create_array, add_array_subset, retrieve_ndarray};
use crate::manage_groups::create_group;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // CREATE STORE
    let store_path: PathBuf = "../zarr-files/hierarchy.zarr".into();
    let store;
    store = create_store(&store_path);
    add_root_group(&mut store);    

    // CREATE ARRAY & its METADATA
    let array_path = "/array";
    let array_shape= vec![3,4];
    let dimension_names = ["y", "x"];
    let json_metadata = serde_json::json!({"Zarr V3": "is great"}).as_object().unwrap().clone();
    create_array(&mut store, array_path, array_shape);
    add_array_metadata(&mut store, array_path, json_metadata);

    // CREATE GROUP & its METADATA
    let group_path = "/new_group";
    let json_metadata = serde_json::json!({"Group name": "the best"}).as_object().unwrap().clone();
    create_group(&mut store, group_path);
    add_group_metadata(&mut store, group_path, json_metadata);

    
    // CONSOLIDATE METADATA
    let metadata = collect_metadata(&mut store)?;
    let metadata_json = serde_json::to_string_pretty(&metadata)?;
    println!("{}", metadata_json);

    // CHUNKING
    let chunk_indices = [0, 0];
    let chunk_elements = [0.2, 0.3, 1.2, 1.3];
    add_chunk(&mut store, array_path, chunk_indices, chunk_elements);
    let array_indices = [1, 1];  
    let array_subset = ndarray::array![[-1.1, -1.2], [-2.1, -2.2]];
    add_array_subset(&mut store, array_path, array_indices, array_subset);

    // Retrieve all array elements as an ndarray
    let array_ndarray;
    array_ndarray = retrieve_ndarray(array);

    return Ok(());
}