use zarrs::array::Array;
use zarrs::storage::ReadableWritableListableStorage;
use ndarray::{ArrayBase, Dim, IxDynImpl, ArrayViewD};
use ndarray::s;


pub fn chunk(data: &ArrayBase<ndarray::OwnedRepr<f32>, Dim<IxDynImpl>>, 
    chunk_shape: [usize; 2],
    store: &ReadableWritableListableStorage, 
    array_path: &str) -> Result<(), Box<dyn std::error::Error>> {

    let shape = data.shape();
    for i in (0..shape[0]).step_by(chunk_shape[0]) {
        for j in (0..shape[1]).step_by(chunk_shape[1]) {
            let row_end = (i + chunk_shape[0]).min(shape[0]);
            let col_end = (j + chunk_shape[1]).min(shape[1]);
            let chunk_extraction: ArrayViewD<f32> = data.slice(s![i..row_end, j..col_end]).into_dyn();
            let chunk_elements: Vec<f32> = chunk_extraction.iter().cloned().collect();
            let expected_size = (row_end - i) * (col_end - j);
            if chunk_elements.len() != expected_size {
                return Err(format!(
                    "Chunk size mismatch: expected {}, got {}",
                    expected_size,
                    chunk_elements.len()
                )
                .into());
            }
            let chunk_indices = [
                (i / chunk_shape[0]) as u64,
                (j / chunk_shape[1]) as u64,
            ];
            add_chunk(&store, array_path, chunk_indices, chunk_elements)?;
        }
    }
    println!("Done chunking");
    return Ok(());
}

pub fn add_chunk(store: &ReadableWritableListableStorage, array_path: &str, chunk_indices: [u64; 2], chunk_elements: Vec<f32>)-> Result<(), Box<dyn std::error::Error>>{
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