use std::path::PathBuf;

use zarr_converter::manage_store::create_store;
use zarr_converter::chunk::add_chunk;
use zarr_converter::metadata_add::collect_metadata;
use zarr_converter::manage_arrays::{create_array, retrieve_ndarray};
use zarr_converter::manage_groups::{add_root_group, create_group};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // CREATE STORE
    let store_path: PathBuf = "../zarr-files/hierarchy.zarr".into();
    let mut store = create_store(&store_path)?;
    add_root_group(&mut store)?;    

    // CREATE GROUP & its METADATA
     let group_path = "/new_group";
     let json_metadata = serde_json::json!({"Group name": "the best"}).as_object().unwrap().clone();
     create_group(&mut store, group_path, json_metadata)?;

    // CREATE ARRAY & its METADATA
    let array_path = "/new_group/array_test";
    let array_shape= vec![5,7];
    let dimension_names = vec!["y", "x"];
    let json_metadata = serde_json::json!({"Zarr V3": "is great"}).as_object().unwrap().clone();
    create_array(&mut store, array_path, array_shape, dimension_names, json_metadata)?;
    
    // // CONSOLIDATE METADATA
    let metadata = collect_metadata(&mut store)?;
    let metadata_json = serde_json::to_string_pretty(&metadata)?;
    println!("{}", metadata_json);

    // // // CHUNKING
    let chunk_indices = [0, 0];
    let chunk_elements = vec![0.2, 0.3, 1.5, 1.3];
    add_chunk(&mut store, array_path, chunk_indices, chunk_elements)?;
    // // @TO-DO: Array type to be figured out
    // // let array_indices = [1, 1];  
    // // let array_subset = ndarray::array![[-1.1, -1.2], [-2.1, -2.2]];
    // // add_array_subset(&mut store, array_path, array_indices, array_subset)?;

    //Retrieve all array elements as an ndarray
    let array_ndarray = retrieve_ndarray(&mut store, array_path)?;
    println!("{}", array_ndarray);

    return Ok(());
}