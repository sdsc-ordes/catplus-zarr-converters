use anyhow::{Context, Result};
use catplus_common::models::types::{Batch,CampaignWrapper};
use clap::Parser;
use serde::Deserialize;
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};
use synth_converter::convert::json_to_rdf;

// Derive Deserialize and ValueEnum
#[derive(Deserialize, Debug, clap::ValueEnum, Clone)]
enum InputType {
    Synth,
    HCI,
}

/// Converts CAT+ Synthesis JSON input into RDF formats.
///
/// This tool expects Synthesis data similar to example/1-Synth.json
/// of a batch with actions. This data is then transformed to RDF and
/// serialized as Turtle (ttl) or JSON-LD (jsonld).
#[derive(Parser, Debug)]
struct Args {

    /// Type of input data: "Synth" or "HCI".
    #[arg(value_enum)] // Use value_enum for the enum
    input_type: InputType,

    /// Path to the input JSON file.
    input_file: String,

    /// Path to the output RDF file.
    output_file: String,

    /// Output format: "ttl" (Turtle) or "jsonld" (JSON-LD)
    #[arg(short, long, default_value = "ttl")]
    format: String,
}

fn main() -> Result<()> {
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

    // Unified conversion function with type selection
    let serialized_graph = match args.input_type {
        InputType::Synth => json_to_rdf::<Batch>(&input_content, &args.format),
        InputType::HCI => json_to_rdf::<CampaignWrapper>(&input_content, &args.format),
    }
    .with_context(|| format!("Failed to convert JSON to RDF format '{}'", args.format))?;

    println!("Conversion successful!");

    // Write to output file
    let output_path = Path::new(&args.output_file);
    let mut output = File::create(output_path)
        .with_context(|| format!("Failed to create output file '{}'", args.output_file))?;
    output
        .write_all(serialized_graph.as_bytes())
        .with_context(|| format!("Failed to write to output file '{}'", args.output_file))?;

    println!("Processed content written to '{}'", output_path.display());
    Ok(())
}
