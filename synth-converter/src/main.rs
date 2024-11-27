mod graph;
mod parser;
use graph::graph_builder::GraphBuilder;
use parser::parser::parse_json;
use std::{
    env,
    fs::File,
    io::{Read, Write},
};

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
            let mut graph_builder = GraphBuilder::new()?;
            graph_builder.add_batch(&batch)?;
            let serialized_graph = graph_builder.serialize_to_turtle()?;
            println!(
                "The resulting graph in Turtle format:\n{}",
                serialized_graph
            );
            let mut output = File::create(output_file)?;
            output.write_all(serialized_graph.as_bytes())?;
            println!("Processed content written to {}", output_file);
            Ok(())
        }
        Err(err) => {
            eprintln!("Error parsing JSON: {}", err);
            Err(Box::new(err))
        }
    }
}
