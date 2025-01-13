use std::{array, path::PathBuf};
use std::fs;
use serde_json::{Value,Map};
use std::io::BufWriter;
use std::collections::HashMap;

use anyhow::{Context, Result};
//use clap::Parser;
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use zarr_converter::manage_store::create_store;
use zarr_converter::chunk::{chunk, add_chunk};
use zarr_converter::metadata_add::collect_metadata;
use zarr_converter::manage_arrays::{create_array, retrieve_ndarray};
use zarr_converter::manage_groups::{add_root_group, create_group};
use zarr_converter::extractor::extract_and_separate;
use zarr_converter::agilent::agilent_get_data;
use zarr_converter::utils::find_middle_divisor;

// #[derive(Parser, Debug)]
// struct Args {
//     /// Path to the input JSON file: relative or absolute.
//     input_file: String,

//     /// Path to the output RDF file.
//     array_path: String,
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    //let args = Args::parse();

    // ZARR: CREATE STORE
    let store_path: PathBuf = "../zarr-files/hierarchy.zarr".into();
    let mut store = create_store(&store_path)?;
    add_root_group(&mut store)?;    

    // ZARR: CREATE GROUP & its METADATA
    let group_path = "/agilent";
    let json_metadata = serde_json::json!({"Group name": "the best"}).as_object().unwrap().clone();
    create_group(&mut store, group_path, json_metadata)?;

     
    // Validate input file
    //let input_path = Path::new(&args.input_file);
    let input_file = "data/agilent-example.json";
    let file_content = fs::read_to_string(input_file)?;
    let json: Value = serde_json::from_str(&file_content)?;

    let mut data_map = Map::new();
    let mut metadata_map =  Map::new();
    
    // Seperate and extract data from metadata 
    extract_and_separate(&json, &mut data_map, &mut metadata_map)?;

    // // Write data to "example-pretty.json"
    // let data_file = File::create("data/pretty-example.json")?;
    // let writer = BufWriter::new(data_file);
    // serde_json::to_writer_pretty(writer, &json)?;
    // println!("Data written to pretty-example.json");

    // // Write data to "data.json"
    // let data_file = File::create("data/data-example.json")?;
    // let writer = BufWriter::new(data_file);
    // serde_json::to_writer_pretty(writer, &data_map)?;
    // println!("Data written to data.json");

    // // Write data to "metadata.json"
    // let metadata_file = File::create("data/metadata-example.json")?;
    // let writer = BufWriter::new(metadata_file);
    // serde_json::to_writer_pretty(writer, &metadata_map)?;
    // println!("Data written to metadata.json");


    // AGILENT DATA CUBES
    let mut data_cubes = HashMap::new();
    agilent_get_data(&data_map, &mut data_cubes)?;
    
    // ZARR: ARRAYS 
    // - Create new Zarr array 
    // - Define ideal chunk shape
    // - Chunk data cube into the Zarr array 

    for (data_cube_name, data_cube) in data_cubes.iter(){
        
        println!("{}", "----------------------");
        println!("{}", data_cube_name);
        println!("{:?}", data_cube.shape());

        // Calculate ideal chunk shape for that data cube
        let chunk_shape= find_middle_divisor(data_cube.shape());
        println!("Chunk Shape: {:?}", chunk_shape);
        
        // Zarr: CREATE ARRAY & its METADATA
        let array_path = format!("/agilent/_{}", data_cube_name) ;
        let array_shape= data_cube.shape().to_vec().iter().map(|&x| x as u64).collect();
        let dimension_names = vec!["y", "x"];
        let chunking_vec: Vec<u64> = vec![chunk_shape.try_into().unwrap(), 2];
        let json_metadata = serde_json::json!({"Zarr V3": "is great"}).as_object().unwrap().clone();
        create_array(&mut store, &array_path, array_shape, chunking_vec, dimension_names, json_metadata)?;
        
        // Zarr: CHUNK DATA CUBE into ARRAY
        println!("Created Array");
        let chunking_usize:[usize;2] = [chunk_shape, 2];
        chunk(&data_cube, chunking_usize, &store, &array_path)?; 
        
        // Zarr: RETRIEVE Full Zarr ARRAY to NDARRAY and check shape
        println!("Retrieving array");
        let array_ndarray = retrieve_ndarray(&mut store, &array_path)?;
        println!("{:?}", array_ndarray.shape());
    }
    // let test_array = data_cubes.get("data_cube_0").unwrap();
    // let chunk_shape= find_middle_divisor(test_array.shape());
    
    // // // CONSOLIDATE METADATA
    // let metadata = collect_metadata(&mut store)?;
    // let metadata_json = serde_json::to_string_pretty(&metadata)?;
    // println!("{}", metadata_json);

    // // // // CHUNKING
    // let chunk_indices = [0, 0];
    // let chunk_elements = vec![0.2, 0.3, 1.5, 1.3];
    // add_chunk(&mut store, array_path, chunk_indices, chunk_elements)?;
    // // // @TO-DO: Array type to be figured out
    // // // let array_indices = [1, 1];  
    // // // let array_subset = ndarray::array![[-1.1, -1.2], [-2.1, -2.2]];
    // // // add_array_subset(&mut store, array_path, array_indices, array_subset)?;

    return Ok(());
}