use serde_json::{Map, Value};
use std::collections::HashMap;
use ndarray::ArrayD;

use crate::array_builder::{convert_to_2d_ndarray, build_2d_ndarray};

fn add_ultraviolet_cube(data_cubes_extracted: &mut HashMap<String, ArrayD<f32>>, i:usize, data_cube: &Value) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(variables) = data_cube
    .get("three-dimensional ultraviolet spectrum data cube")
    .and_then(|cube| cube.get("data")) {
        let measures = variables["measures"][0].as_array().ok_or("Expected 'data' to be an array")?;
        let ndarray = convert_to_2d_ndarray(measures)?;
        // QUESTION: What to do with the dimensions? Put in a metadata ? Have metadata & dimensions in attributes? 
        //println!("NDarray: {:?}", ndarray.shape());
        data_cubes_extracted.insert(format!("data_cube_{}", i), ndarray);
    }
    return Ok(());
}


fn add_data_cube(data_cubes_extracted: &mut HashMap<String, ArrayD<f32>>, i:usize, data_cube: &Value) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(_check) = data_cube["chromatogram data cube"]["data"]["measures"].as_array() {
        let measures= data_cube["chromatogram data cube"]["data"]["measures"][0].as_array().ok_or("Expected 'data' to be an array")?;
        let dimensions= data_cube["chromatogram data cube"]["data"]["dimensions"][0].as_array().ok_or("Expected 'data' to be an array")?;
        let ndarray = build_2d_ndarray(measures, dimensions)?;
        data_cubes_extracted.insert(format!("data_cube_{}", i), ndarray);
    }
    return Ok(());    
}

pub fn agilent_get_data(data_map: &Map<String, Value>, data_cubes_extracted: &mut HashMap<String, ArrayD<f32>>) -> Result< (), Box<dyn std::error::Error>>  {
    let data_cubes  = data_map["liquid chromatography aggregate document"]["liquid chromatography document"]["data"][0]["measurement aggregate document"]["measurement document"]["data"].as_array().ok_or("Expected 'data' to be an array")?;
    for (i, data_cube) in data_cubes.iter().enumerate() {
        add_data_cube(data_cubes_extracted, i, data_cube)?;
        //add_ultraviolet_cube(data_cubes_extracted, i, data_cube)?;
        //add_mass_cube(data_cubes_extracted, i, data_cube)?;
    }
    return Ok(());
}
