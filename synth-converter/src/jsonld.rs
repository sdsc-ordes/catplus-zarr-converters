use sophia::jsonld::parser::JsonLdParser;
use sophia::inmem::dataset::LightDataset;
use sophia::turtle::serializer::trig::{TrigConfig, TrigSerializer};
use sophia::api::parser::QuadParser;
use sophia::api::prelude::QuadSource;
use sophia::api::prelude::QuadSerializer;
use sophia::api::dataset::Dataset;
use sophia::api::prelude::Stringifier;

// Here we create fake jsonld data and parse it into a dataset creating a QuadSource (easy way to stream quads)
// We then serialize the dataset into a string using the TrigSerializer 
// The output is the TriG representation of the dataset, human readable
// in comments there is a JsonSerializer attempt, but the output is difficult to manipulate
pub fn manip_jsonld() -> Result<(), Box<dyn std::error::Error>> {

    let jsonld_data = r#"
    {
        "@context": {
            "name": "http://schema.org/name",
            "homepage": "http://schema.org/url"
        },
        "@id": "http://example.org/alice",
        "name": "Alice",
        "homepage": "http://example.org/alice/home"
    }
    "#;
    let jsonld_parser = JsonLdParser::new();
    let quad_source = jsonld_parser.parse_str(jsonld_data);
    let mut dataset_2 = LightDataset::new();
    quad_source.add_to_dataset(&mut dataset_2);

   

    // // Use the JsonLdSerializer from sophia_jsonld to serialize the dataset

    let mut stringifier = TrigSerializer::new_stringifier_with_config(TrigConfig::new().with_pretty(true));
    let trig = stringifier.serialize_quads(dataset_2.quads())?;
    println!("{}", trig.as_str());

    // let mut serializer = JsonLdSerializer::new_jsonifier();
    // let jsonld_string =  serializer.serialize_dataset(&dataset_2);
    // what to do with this output?? does not go into serde_json so not good JSON? 

    Ok(())
}