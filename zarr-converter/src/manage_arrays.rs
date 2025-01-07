use zarrs::array::{Array, ArrayBuilder, DataType, FillValue, ZARR_NAN_F32};
use zarrs::array::codec::GzipCodec; // requires gzip feature
use zarrs::storage::ReadableWritableListableStorage;
use std::array;
use std::sync::Arc;
use serde_json::{Value, Map};

pub fn create_array(store: &mut ReadableWritableListableStorage, 
    array_path: &str, array_shape: Vec<u64>, 
    dimension_names: Vec<&str>,
    metadata: Map<String, Value>)-> Result<(), Box<dyn std::error::Error>>{
    let _array = ArrayBuilder::new(
        array_shape, // array shape
        DataType::Float32,
        vec![2, 2].try_into()?, // regular chunk shape (non-zero elements)
        FillValue::from(ZARR_NAN_F32),
    )
    .bytes_to_bytes_codecs(vec![
        Arc::new(GzipCodec::new(5)?),
    ])
    .dimension_names(dimension_names.into())
    .attributes(metadata)
    .build(store.clone(), array_path)?;
    _array.store_metadata()?;
    println!("{}", array_path);
    Ok(())
}

// @TO-DO: Array type to be figured out
// pub fn add_array_subset(store: &mut ReadableWritableListableStorage, array_path: &str, array_indices: [u64; 2], array_subset: ndarray::ArrayBase<<f64>, <[usize; 2]>>)-> Result<(), Box<dyn std::error::Error>>{
//     let _array = Array::open(store.clone(), array_path)?;
//     _array.store_array_subset_ndarray::<f32, _>(
//         &array_indices,
//         array_subset
//     )?;
//     Ok(())
// }

pub fn retrieve_ndarray(store: &mut ReadableWritableListableStorage, array_path: &str)-> Result<ndarray::ArrayD<f32>, Box<dyn std::error::Error>>{
    let _array = Array::open(store.clone(), array_path)?;
    let array_ndarray = _array.retrieve_array_subset_ndarray::<f32>(&_array.subset_all())?;
    Ok(array_ndarray)
}