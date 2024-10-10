// use sophia::inmem::graph::FastGraph;   
// use sophia_jsonld::serializer::JsonLdSerializer;      
// use sophia::sophia_api::prelude::QuadSerializer;

use sophia::inmem::dataset::LightDataset;
use sophia::iri::Iri;
use sophia_jsonld::serializer::JsonLdSerializer; 
use sophia::sophia_api::prelude::QuadSerializer;

// rio is also a good library of rdf in rust 
// rdf tooling rust: https://github.com/oxigraph/oxigraph
// idea A: 1. parse json into graph (triples) 2. convert graph to ttl 3. ttl to quads 4. quads to jsonld to put into zarr
// idea B: 1. parse json into graph (quads)


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a dataset for quads (in-memory quad store)
    let mut dataset = LightDataset::new();

    // Define graph IRIs (named graphs)
    let graph_iri = Iri::new("http://example.org/graph1").unwrap();

    // Insert quads into the dataset
    dataset.insert(
        &"http://example.org/#Alice",
        &"http://xmlns.com/foaf/0.1/name",
        &"Alice",
        &graph_iri
    )?;

    dataset.insert(
        &"http://example.org/#Alice",
        &"http://xmlns.com/foaf/0.1/knows",
        &"http://example.org/#Bob",
        &graph_iri
    )?;

    // Use the JsonLdSerializer from sophia_jsonld to serialize the dataset
    let mut serializer = JsonLdSerializer::new_stringifier();
    
    // Serialize the dataset to JSON-LD
    let jsonld_string = serializer.serialize_quads(&dataset)?;

    // Output the JSON-LD to the console
    println!("{}", jsonld_string);

    Ok(())
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let mut graph = FastGraph::new();

//     graph.insert(
//         &"http://example.org/#Alice", 
//         &"http://xmlns.com/foaf/0.1/name", 
//         &"Alice",                      
//     )?;
    
//     graph.insert(
//         &"http://example.org/#Alice",   
//         &"http://xmlns.com/foaf/0.1/knows", 
//         &"http://example.org/#Bob",     
//     )?;

//     //let mut serializer = JsonLdSerializer::new_stringifier();
//     let mut serializer = JsonLdSerializer::new_stringifier();
    
//     let jsonld_string = serializer.serialize_quads(&graph)?;

//     println!("{}", jsonld_string);
    
//     Ok(())
// }