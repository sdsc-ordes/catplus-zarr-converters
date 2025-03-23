// use serde::{Deserialize, Serialize};
// use serde_xml_rs::{from_str, to_string};

use anyhow::{Context, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "OPUSDataFile")]
struct OpusDataFile {
    #[serde(rename = "Data_Parameters_TR", default)]
    data_parameters_tr: Option<Parameters>,
    
    #[serde(rename = "TR__Peak", default)]
    tr_peak: Option<PeakReport>,
    
    #[serde(rename = "Data_Parameters_ScSm", default)]
    data_parameters_scsm: Option<Parameters>,
    
    #[serde(rename = "ScSm__Peak", default)]
    scsm_peak: Option<PeakReport>,
    
    #[serde(rename = "Data_Parameters_ScRf", default)]
    data_parameters_scrf: Option<Parameters>,
    
    #[serde(rename = "ScRf__Peak", default)]
    scrf_peak: Option<PeakReport>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Parameters {
    #[serde(rename = "parameter", default)]
    parameters: Vec<Parameter>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Parameter {
    #[serde(rename = "name", default)]
    name: String,
    #[serde(rename = "$text", default)]
    value: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "MainReport")]
struct PeakReport {
    #[serde(rename = "Header", default)]
    header: Option<Header>,
    #[serde(rename = "Matrix", default)]
    matrix: Option<Matrix>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Header {
    #[serde(rename = "Line", default)]
    lines: Vec<Line>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Line {
    #[serde(rename = "Legend", default)]
    legend: Option<String>,
    #[serde(rename = "Value", default)]
    value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Matrix {
    #[serde(rename = "Line", default)]
    lines: Vec<MatrixLine>,
}

#[derive(Debug, Deserialize, Serialize)]
struct MatrixLine {
    #[serde(rename = "Value", default)]
    values: Vec<String>,
}

#[derive(Parser, Debug)]
struct Args {

    /// Path to the input JSON file.
    input_file: String,

}

fn main() -> Result<()>{
    let args = Args::parse();

    // Validate input file
    let input_path = Path::new(&args.input_file);
    if !input_path.exists() {
        anyhow::bail!("Input file '{}' does not exist.", args.input_file);
    }
    if !input_path.is_file() {
        anyhow::bail!("'{}' is not a valid file.", args.input_file);
    }

    // Read input file
    let mut input_content = String::new();
    File::open(input_path)
        .with_context(|| format!("Failed to open input file '{}'", args.input_file))?
        .read_to_string(&mut input_content)
        .with_context(|| format!("Failed to read input file '{}'", args.input_file))?;
    
    let opus_data: OpusDataFile = serde_xml_rs::from_str(&input_content).expect("Failed to parse XML");
    println!("{:?}", opus_data);
    Ok(())
}


// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// struct Item {
//     name: String,
//     source: String,
// }

// fn main() {
//     let src = r#"<Item><name>Banana</name><source>Store</source></Item>"#;
//     let should_be = Item {
//         name: "Banana".to_string(),
//         source: "Store".to_string(),
//     };

//     let item: Item = from_str(src).unwrap();
//     assert_eq!(item, should_be);

//     let reserialized_item = to_string(&item).unwrap();
//     //assert_eq!(src, reserialized_item);
// }
