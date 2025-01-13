use serde_json::{Map, Value};
use std::collections::HashMap;
use ndarray::{ArrayD, ArrayBase, Dim, IxDynImpl, Array2, stack, Axis};

use crate::array_builder::{convert_to_2d_ndarray, build_2d_ndarray};

fn add_ultraviolet_cube(data_cubes_extracted: &mut HashMap<String, ArrayD<f32>>, i:usize, data_cube: &Value) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(variables) = data_cube
    .get("three-dimensional ultraviolet spectrum data cube")
    .and_then(|cube| cube.get("data")) {
        let measures = variables["measures"][0].as_array().ok_or("Expected 'data' to be an array")?;
        let ndarray = convert_to_2d_ndarray(measures)?;
        // QUESTION: What to do with the dimensions? Put in a metadata ? Have metadata & dimensions in attributes? 
        println!("NDarray: {:?}", ndarray.shape());
        data_cubes_extracted.insert(format!("data_cube_{}", i), ndarray);
    }
    return Ok(());
}



fn add_data_cube(data_cubes_extracted: &mut HashMap<String, ArrayD<f32>>, i:usize, data_cube: &Value) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(check) = data_cube["chromatogram data cube"]["data"]["measures"].as_array() {
        let measures= data_cube["chromatogram data cube"]["data"]["measures"][0].as_array().ok_or("Expected 'data' to be an array")?;
        let dimensions= data_cube["chromatogram data cube"]["data"]["dimensions"][0].as_array().ok_or("Expected 'data' to be an array")?;
        let ndarray = build_2d_ndarray(measures, dimensions)?;
        println!("NDarray: {:?}", ndarray.shape());
        data_cubes_extracted.insert(format!("data_cube_{}", i), ndarray);
    }
    return Ok(());    
}

pub fn agilent_get_data(data_map: &Map<String, Value>, data_cubes_extracted: &mut HashMap<String, ArrayD<f32>>) -> Result< (), Box<dyn std::error::Error>>  {
    let data_cubes  = data_map["liquid chromatography aggregate document"]["liquid chromatography document"]["data"][0]["measurement aggregate document"]["measurement document"]["data"].as_array().ok_or("Expected 'data' to be an array")?;
    println!("Data cubes {}", data_cubes.len());
    for (i, data_cube) in data_cubes.iter().enumerate() {
        println!("{}", "-----------");
        println!("Processing {}", i);
        add_data_cube(data_cubes_extracted, i, data_cube)?;
        add_ultraviolet_cube(data_cubes_extracted, i, data_cube)?;
        //add_mass_cube(data_cubes_extracted, i, data_cube)?;
    }
    return Ok(());
}


// fn extract_from_mass(variables: &Value) -> Result<(Vec<Value>, Vec<Vec<Value>>), Box<dyn std::error::Error>> {
//     let mut measures: Vec<Value> = Vec::new();
//     let mut dimensions: Vec<Vec<Value>> = vec![Vec::new(), Vec::new()];

//     for point in variables.as_array().ok_or("Expected 'data' to be an array")? {
//         let measure_array = point["measures"][0]
//             .as_array()
//             .ok_or("Expected 'measures[0]' to be an array")?;
//         measures.push(measure_array.iter().cloned().collect());

//         let time_value = point["time"].clone();
//         dimensions[0].push(time_value);

//         let dimension_array = point["dimensions"][0]
//             .as_array()
//             .ok_or("Expected 'dimensions[0]' to be an array")?;
//         dimensions[1].extend(dimension_array.iter().cloned());
//     }

//     Ok((measures, dimensions))
// }

// fn add_mass_cube(data_cubes_extracted: &mut HashMap<String, ArrayD<f32>>, i:usize, data_cube: &Value) -> Result<(), Box<dyn std::error::Error>> {
//     if let Some(variables) = data_cube
//     .get("three-dimensional mass spectrum data cube")
//     .and_then(|cube| cube.get("data")) {
//         let(measures, dimensions) = extract_from_mass(variables)?;
//         // QUESTION: What to do with the dimensions? Put in a metadata ? Have metadata & dimensions in attributes? 
//         println!("{:?}", measures.len());
//         let ndarray = convert_to_2d_ndarray(&measures)?;
//         println!("NDarray: {:?}", ndarray.shape());
//         data_cubes_extracted.insert(format!("data_cube_{}", i), ndarray);
//     }
//     return Ok(());
// }

