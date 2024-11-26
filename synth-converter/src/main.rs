mod parser;
use parser::parser::parse_json;
use serde_json;
use std::env;
use std::fs::File;
use std::io::{Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        std::process::exit(1);
    }
    let input_file = &args[1];
    let output_file = &args[2];

    let mut input_content = String::new();
    File::open(input_file)?.read_to_string(&mut input_content)?;

    match parse_json(&input_content) {
        Ok(batch) => {
            let mut output = File::create(output_file)?;
            let serialized_batch = serde_json::to_string(&batch)?;
            output.write_all(serialized_batch.as_bytes())?;
            println!("Processed content written to {}", output_file);
            Ok(())
        }
        Err(err) => {
            eprintln!("Error parsing JSON: {}", err);
            Err(Box::new(err))
        }
    }
}


