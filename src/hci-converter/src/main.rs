use anyhow::{Context, Result};
use clap::Parser;
use hci_converter::convert::json_to_rdf;
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

/// Converts CAT+ HCI JSON input into RDF formats.
///
/// This tool expects HCI data similar to example/1-HCI.json
/// of a batch with actions. This data is then transformed to RDF and
/// serialized as Turtle (ttl) or JSON-LD (jsonld).
#[derive(Parser, Debug)]
struct Args {
    /// Path to the input JSON file: relative or absolute.
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

    // Use unified conversion function
    let serialized_graph = json_to_rdf(&input_content, &args.format)
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
