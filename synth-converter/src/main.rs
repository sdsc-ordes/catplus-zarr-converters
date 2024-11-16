// "Import" anything public in the parser module
pub mod parser;

fn main() {
    println!("Hello, world!");

    // Parse the JSON
    let result = parser::untyped_example();

    // Handle errors from the parser if any
    match result {
        Ok(_result) => (),
        Err(error) => print!("{}", error),
    }
}
