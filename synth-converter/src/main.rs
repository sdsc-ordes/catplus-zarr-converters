use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        std::process::exit(1);
    }
    let input_file = &args[1];
    let output_file = &args[2];

    let mut input_content = String::new();
    File::open(input_file)?.read_to_string(&mut input_content)?;

    let processed_content = input_content.to_uppercase();

    let mut output = File::create(output_file)?;
    output.write_all(processed_content.as_bytes())?;

    println!("Processed content written to {}", output_file);
    Ok(())
}

