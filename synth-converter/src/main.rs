// src/main.rs

use std::{
    env,
    fs::File,
    io::{Read, Write},
    process,
};
use synth_converter::convert::json_to_turtle;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        process::exit(1);
    }
    let input_file = &args[1];
    let output_file = &args[2];

    let mut input_content = String::new();
    File::open(input_file)?.read_to_string(&mut input_content)?;

    match json_to_turtle(&input_content) {
        Ok(serialized_graph) => {
            println!(
                "{}",
                serialized_graph
            );
            let mut output = File::create(output_file)?;
            output.write_all(serialized_graph.as_bytes())?;
            println!("Processed content written to {}", output_file);
            Ok(())
        }
        Err(err) => {
            eprintln!("Error converting JSON to Turtle: {}", err);
            Err(err)
        }
    }
}
