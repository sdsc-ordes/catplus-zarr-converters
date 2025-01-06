use zarrs::array::Array;
use zarrs::storage::ReadableWritableListableStorage;

pub fn add_chunk(store: &mut ReadableWritableListableStorage, array_path: &str, chunk_indices: [u64; 2], chunk_elements: Vec<f32>)-> Result<(), Box<dyn std::error::Error>>{
    let _array = Array::open(store.clone(), array_path)?;
    _array.store_chunk_elements::<f32>(
        &chunk_indices, 
        &chunk_elements
    )?;
    Ok(())
}

pub fn erase_chunk(store: &mut ReadableWritableListableStorage, array_path: &str, chunk_indices: [u64; 2])-> Result<(), Box<dyn std::error::Error>>{
    let _array = Array::open(store.clone(), array_path)?;
    _array.erase_chunk(&chunk_indices)?;
    Ok(())
}