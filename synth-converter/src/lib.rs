pub mod convert {
    use crate::{graph::graph_builder::GraphBuilder, parser::parser::parse_json};
    use std::error::Error;

    pub fn json_to_turtle(input_content: &str) -> Result<String, Box<dyn Error>> {
        let batch = parse_json(input_content)?;
        let mut graph_builder = GraphBuilder::new()?;
        graph_builder.add_batch(&batch)?;
        let serialized_graph = graph_builder.serialize_to_turtle()?;
        Ok(serialized_graph)
    }
}

mod graph;
mod parser;
