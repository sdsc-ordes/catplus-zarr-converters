use clap::Parser;
use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};
use synth_converter::convert::json_to_rdf; // Use the unified conversion function.

/// Converts CAT+ Synthesis JSON input into RDF formats.
///
/// This tool expects Synthesis data similar to example/1-Synth.json
/// of a batch with actions. This data is then transformed to RDF and
/// serialized as Turtle (ttl) or JSON-LD (jsonld).
#[derive(Parser, Debug)]
struct Args {
    /// Path to the input JSON file: relative or absolute
    input_file: String,

    /// Path to the output RDF file
    output_file: String,

    /// Output format: "ttl" (Turtle) or "jsonld" (JSON-LD)
    #[arg(short, long, default_value = "ttl")]
    format: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Validate input file
    let input_path = Path::new(&args.input_file);
    if !input_path.exists() {
        eprintln!("Error: Input file '{}' does not exist.", args.input_file);
        return Err(io::Error::new(io::ErrorKind::NotFound, "Input file not found").into());
    }
    if !input_path.is_file() {
        eprintln!("Error: '{}' is not a valid file.", args.input_file);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Input is not a file").into());
    }

    // Read input file
    let mut input_content = String::new();
    File::open(input_path)?.read_to_string(&mut input_content)?;

    // Use unified conversion function
    match json_to_rdf(&input_content, &args.format) {
        Ok(serialized_graph) => {
            println!("Conversion successful!");

            // Write to output file
            let output_path = Path::new(&args.output_file);
            let mut output = File::create(output_path)?;
            output.write_all(serialized_graph.as_bytes())?;
            println!("Processed content written to '{}'", output_path.display());
            Ok(())
        }
        Err(err) => {
            eprintln!("Error during conversion: {}", err);
            Err(err)
        }
    }
}
