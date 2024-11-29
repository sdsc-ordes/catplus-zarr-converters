use clap::Parser;
use std::{
    fs::File,
    io::{Read, Write},
};
use synth_converter::convert::json_to_turtle;

/// A simple JSON to Turtle converter.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input JSON file
    input_file: String,

    /// Output Turtle file
    output_file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut input_content = String::new();
    File::open(&args.input_file)?.read_to_string(&mut input_content)?;

    match json_to_turtle(&input_content) {
        Ok(serialized_graph) => {
            println!("{}", serialized_graph);
            let mut output = File::create(&args.output_file)?;
            output.write_all(serialized_graph.as_bytes())?;
            println!("Processed content written to {}", args.output_file);
            Ok(())
        }
        Err(err) => {
            eprintln!("Error converting JSON to Turtle: {}", err);
            Err(err)
        }
    }
}
