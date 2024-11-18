use std::collections::HashMap;
use sophia::api::graph::MutableGraph;
use sophia::api::ns::Namespace;
use sophia::api::serializer::TripleSerializer;
use sophia::inmem::graph::LightGraph;
use sophia_api::term::bnode_id::BnodeId;
use sophia_turtle::serializer::turtle::TurtleSerializer;
use crate::parser::batch::Batch;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref RDF: Namespace<&'static str> = Namespace::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#").unwrap();
    pub static ref CAT: Namespace<&'static str> = Namespace::new("http://example.org/cat#").unwrap();
    pub static ref SCHEMA: Namespace<&'static str> = Namespace::new("https://schema.org/").unwrap();
    pub static ref ALLORES: Namespace<&'static str> = Namespace::new("http://purl.allotrope.org/ontologies/result#").unwrap();
    pub static ref EX: Namespace<&'static str> = Namespace::new("http://example.org/").unwrap();
}

pub struct GraphBuilder {
    action_counter: HashMap<String, usize>,
    graph: LightGraph,
}

impl GraphBuilder {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            action_counter: HashMap::new(),
            graph: LightGraph::new(),
        })
    }

    fn get_action_uri(&mut self, action_name: &str)  -> String {
        // Increment the counter for the given action name
        let count = self.action_counter.entry(action_name.to_string()).or_insert(0);
        *count += 1;

        // Generate a unique URI
        format!("{}_{}", action_name, *count)
    }

    fn get_action_iri_name(&self, action_name: &str) -> String {
        let mapped_class = match action_name {
            "add" => "AddAction",
            "set_temperature" => "setTemperatureAction",
            _ => "AFRE_0000001",
        };
        mapped_class.to_string()
    }

    pub fn add_batch(&mut self, batch: &Batch) -> Result<(), Box<dyn std::error::Error>> {
        let batch_uri = EX.get(&batch.batch_id)?.clone();
        let rdf_type = RDF.get("type")?;

        self.graph.insert(
            &batch_uri,
            &ALLORES.get("AFR_0001120")?,
            batch.batch_id.as_str(),
        )?;

        self.graph.insert(
            &batch_uri,
            &rdf_type,
            &CAT.get("Batch")?,
        )?;

        for action in &batch.actions {
            let unique_action_name = self.get_action_uri(&action.name).clone();

            let action_bnode = BnodeId::new_unchecked(&*unique_action_name);
            self.graph.insert(
                &batch_uri,
                &ALLORES.get("AFRE_0000001")?,
                &action_bnode,
            )?;

            let action_iri_name = self.get_action_iri_name(&action.name);
            let mapped_action = CAT.get(&action_iri_name)?.clone();
            self.graph
                .insert(
                    &action_bnode,
                    &rdf_type,
                    &mapped_action,
                )
                .unwrap();
                let schema_name = SCHEMA.get("name")?;
                let cat_local_name = CAT.get("localEquipmentName")?;
                let cat_barcode = CAT.get("containerBarcode")?;
                let cat_dispense_type = CAT.get("dispenseType")?;
                let action_predicates = vec![
                    (Some(action.name.as_str()), &schema_name),
                    (
                        action.equipment_local_name.as_ref().map(|x| x.as_str()),
                        &cat_local_name,
                    ),
                    (
                        action.container_barcode.as_ref().map(|x| x.as_str()),
                        &cat_barcode,
                    ),
                    (
                        action.dispense_type.as_ref().map(|x| x.as_str()),
                        &cat_dispense_type,
                    ),
                ];
                for (field, predicate) in action_predicates {
                    if let Some(value) = field {
                        // Use `&*predicate` to convert `&NsTerm<'_>` to `&&NsTerm<'_>`
                        self.graph.insert(&action_bnode, &*predicate, value)?;
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
