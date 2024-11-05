use sophia::inmem::dataset::LightDataset;
use sophia::api::dataset::MutableDataset;
use sophia::iri::Iri;
use sophia::api::dataset::Dataset;

// Here we create an example dataset of quads
// we show how to extract these quads and print them

pub fn create_quad_dataset() -> Result<(), Box<dyn std::error::Error>> {
    let mut dataset = LightDataset::new();

    // Define graph IRIs (named graphs)
    let graph_iri = Iri::new("http://example.org/graph1").unwrap();

    // Insert quads into the dataset
    dataset.insert(
        "http://example.org/#Alice",
        "http://xmlns.com/foaf/0.1/name",
        "Alice",
        Some(&graph_iri)
    )?;

    dataset.insert(
        "http://example.org/#Alice",
        "http://xmlns.com/foaf/0.1/knows",
        "http://example.org/#Bob",
        Some(&graph_iri)
    )?;

    let quads: Vec<_> = dataset.quads().collect();

    for quad in quads {
        println!("Here is a quad: ");
        println!("{:?}", quad);
    }
    Ok(())
}


