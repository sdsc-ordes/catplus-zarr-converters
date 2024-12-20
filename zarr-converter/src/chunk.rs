use zarrs::array::{ArrayBuilder, DataType, FillValue, ZARR_NAN_F32};
use zarrs::array::codec::GzipCodec; // requires gzip feature
use zarrs::array_subset::ArraySubset;

// To Do : Create Array - Under construction
// pub fn create_array(store, path_in_store, array_shape, json_metadata)-> Result<Array>{
//     // Create a new V3 array using the array builder
//     let array = ArrayBuilder::new(
//         array_shape, // array shape
//         DataType::Float32,
//         vec![2, 2].try_into()?, // regular chunk shape (non-zero elements)
//         FillValue::from(ZARR_NAN_F32),
//     )
//     .bytes_to_bytes_codecs(vec![
//         Arc::new(GzipCodec::new(5)?),
//     ])
//     .dimension_names(["y", "x"].into())
//     .attributes(json_metadata.as_object().unwrap().clone())
//     .build(store.clone(), path_in_store)?;


//     // let array = ArrayBuilder::new(
//     //     vec![3, 4], // array shape
//     //     DataType::Float32,
//     //     vec![2, 2].try_into()?, // regular chunk shape (non-zero elements)
//     //     FillValue::from(ZARR_NAN_F32),
//     // )
//     // .bytes_to_bytes_codecs(vec![
//     //     Arc::new(GzipCodec::new(5)?),
//     // ])
//     // .dimension_names(["y", "x"].into())
//     // .attributes(serde_json::json!({"Zarr V3": "is great"}).as_object().unwrap().clone())
//     // .build(store.clone(), "/array")?; // /path/to/hierarchy.zarr/array
//     Ok(array)
// }

// To test
pub fn add_chunk(array: &mut Array, chunk_indices: &Array, chunk_elements: &Array)-> Result<Array>{
    array.store_chunk_elements::<f32>(
        &chunk_indices, 
        &chunk_elements
    )?;
    Ok(array)
}

//To test
pub fn add_array_subset(array: &mut Array, array_indices: &Array, array_subset: &Array)-> Result<Array>{
    array.store_array_subset_ndarray::<f32, _>(
        &array_indices,
        array_subset
    )?;
    Ok(array)
}

// To do : 
// array.erase_chunk(&[1, 1])?;
// // Retrieve all array elements as an ndarray
// let array_ndarray = array.retrieve_array_subset_ndarray::<f32>(&array.subset_all());
    