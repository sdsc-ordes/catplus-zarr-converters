use sophia::api::graph::MutableGraph;
use sophia::api::ns::Namespace;
use sophia::api::serializer::TripleSerializer;
use sophia::inmem::graph::FastGraph;
use sophia_turtle::serializer::turtle::TurtleSerializer;
use crate::parser::batch::Batch;

pub struct GraphBuilder {
    graph: FastGraph,
    ex: Namespace<String>,
    allores: Namespace<String>,
    schema: Namespace<String>,
}

impl GraphBuilder {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            graph: FastGraph::new(),
            ex: Namespace::<String>::new("http://example.org/".to_string())?,
            allores: Namespace::<String>::new("http://purl.allotrope.org/ontologies/result#".to_string())?,
            schema: Namespace::<String>::new("https://schema.org/".to_string())?,
        })
    }

    pub fn add_batch(&mut self, batch: &Batch) -> Result<(), Box<dyn std::error::Error>> {
        let batch_uri = self.ex.get(&batch.batch_id)?;

        self.graph.insert(
            &batch_uri,
            &self.allores.get("AFR_0001120")?,
            batch.batch_id.as_str(),
        )?;

        for action in &batch.actions {
            let action_uri = self.ex.get(&action.name)?;
            self.graph.insert(
                &batch_uri,
                &self.allores.get("AFRE_0000001")?,
                &action_uri,
            )?;

            let action_predicates = vec![
                (Some(action.name.as_str()), self.schema.get("name")?),
            ];
    
            for (field, predicate) in action_predicates {
                if let Some(value) = field {
                    self.graph.insert(&action_uri, &predicate, value)?;
                }
            } 
        }

        Ok(())
    }

    pub fn serialize_to_turtle(&self) -> Result<String, Box<dyn std::error::Error>> {
        use sophia::api::serializer::Stringifier; // Import the required trait
    
        let mut serializer = TurtleSerializer::new_stringifier();
        serializer.serialize_graph(&self.graph)?;
        Ok(serializer.as_str().to_string())
    }
    
}
