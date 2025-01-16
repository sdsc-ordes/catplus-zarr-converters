use zarrs::array::{Array, ArrayBuilder, DataType, FillValue, ZARR_NAN_F32};
use zarrs::array::codec::GzipCodec; 
use zarrs::storage::ReadableWritableListableStorage;
use std::sync::Arc;
use serde_json::{Value, Map};

pub fn create_array(store: &mut ReadableWritableListableStorage, 
    array_path: &str, array_shape: Vec<u64>, 
    chunking_shape: Vec<u64>,
    dimension_names: Vec<&str>,
    metadata: Map<String, Value>)-> Result<(), Box<dyn std::error::Error>>{
    let _array = ArrayBuilder::new(
        array_shape,
        DataType::Float32,
        chunking_shape.try_into()?,
        FillValue::from(ZARR_NAN_F32),
    )
    .bytes_to_bytes_codecs(vec![
        Arc::new(GzipCodec::new(5)?),
    ])
    .dimension_names(dimension_names.into())
    .attributes(metadata)
    .build(store.clone(), array_path)?;
    _array.store_metadata()?;
    Ok(())
}

pub fn retrieve_ndarray(store: &mut ReadableWritableListableStorage, array_path: &str)-> Result<ndarray::ArrayD<f32>, Box<dyn std::error::Error>>{
    let _array = Array::open(store.clone(), array_path)?;
    let array_ndarray = _array.retrieve_array_subset_ndarray::<f32>(&_array.subset_all())?;
    Ok(array_ndarray)
}