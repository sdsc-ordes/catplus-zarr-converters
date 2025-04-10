use anyhow::{Context, Result};
use clap::Parser;
use std::{
    fs,
    fs::File,
    io::{stdin, stdout, BufReader, BufWriter, Read, Write},
    path::PathBuf,
};
use catplus_common::rdf::{
    rdf_parser::parse_turtle_to_graph,
    rdf_serializers::serialize_graph_to_turtle,
} ;
use validation::{
    core::*,
    engines::shacl_api::*,
};

// Validates an RDF file
// Only turtle format is supported
#[derive(Parser, Debug)]
struct Args {

    /// Path to the input RDF data.
    /// Defaults to stdin.
    #[arg(default_value = "-")]
    input: PathBuf,

    /// Path to the output validation report.
    /// Defaults to stdout
    #[arg(short, long, default_value = "-")]
    output: PathBuf,

    /// Path to the shapes file.
    /// If not provided, the default shapes of the validation engine will be used.
    #[arg(short, long, default_value=None)]
    shapes: Option<PathBuf>,

    /// Endpoint of the SHACL API server.
    #[arg(short, long)]
    endpoint: String,
}

fn main() -> Result<()> {
    let args = Args::parse();


    validate_graph(args.input, args.output, args.shapes, args.endpoint)?;

    Ok(())
}


// Get a reader based on input path, either from stdin or a file.
pub fn get_reader(path: &PathBuf) -> Result<Box<dyn Read>> {
    return match path.to_str().unwrap() {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        path => Ok(Box::new(BufReader::new(File::open(path)?))),
    }
}

// Get a writer based on input path, either to stdout or a file.
pub fn get_writer(path: &PathBuf) -> Result<Box<dyn Write>>{
    return match path.to_str().unwrap() {
        "-" => Ok(Box::new(BufWriter::new(stdout()))),
        path => Ok(Box::new(BufWriter::new(File::create(path)?))),
    };
}

fn validate_graph(
    input: PathBuf,
    output: PathBuf,
    shapes: Option<PathBuf>,
    endpoint: String,
) -> Result<()> {

    // Check if the endpoint is reachable
    let shacl_api = ShaclApiEndpoint::new(endpoint.clone());
    if !shacl_api.is_available() {
        return Err(anyhow::anyhow!("SHACL API is not available at {}", endpoint));
    }

    // Parse I/O paths
    let mut source = get_reader(&input)?;
    let mut sink = get_writer(&output)?;

    // Read whole files as strings
    let mut input_data = String::new();
    source
        .read_to_string(&mut input_data)
        .context("Failed to read input data")?;

    let shapes_data = shapes
        .map(|path| {
            fs::read_to_string(path)
                .expect("Failed to read shapes file")
        });

    // Parse into triple graphs
    let data_graph = parse_turtle_to_graph(&input_data)
        .context("Failed to parse input RDF data")?;

    let shapes_graph = shapes_data.map(|data| {
        parse_turtle_to_graph(&data)
            .expect("Failed to parse shapes data")
    });

    let report = shacl_api.validate(&data_graph, shapes_graph.as_ref()).unwrap();
    
    // Write the validation report to the output
    sink 
        .write_all(serialize_graph_to_turtle(&report.graph).unwrap().as_bytes())
        .context("Failed to write to output file")?;
    Ok(())
}
