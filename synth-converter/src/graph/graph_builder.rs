use sophia::api::graph::MutableGraph;
use sophia::api::ns::Namespace;
use sophia::api::serializer::TripleSerializer;
use sophia::inmem::graph::LightGraph;
use sophia_turtle::serializer::turtle::TurtleSerializer;
use crate::parser::batch::Batch;
use crate::parser::batch::Action;
use sophia_api::term::bnode_id::BnodeId;
use sophia_api::ns::NsTerm;
use crate::parser::batch::BaseAction;
use crate::parser::batch::Measurement;
use crate::graph::utils::generate_unique_identifier;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref RDF: Namespace<&'static str> = Namespace::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#").unwrap();
    pub static ref CAT: Namespace<&'static str> = Namespace::new("http://example.org/cat#").unwrap();
    pub static ref SCHEMA: Namespace<&'static str> = Namespace::new("https://schema.org/").unwrap();
    pub static ref ALLORES: Namespace<&'static str> = Namespace::new("http://purl.allotrope.org/ontologies/result#").unwrap();
    pub static ref EX: Namespace<&'static str> = Namespace::new("http://example.org/").unwrap();
    pub static ref QUDT: Namespace<&'static str> = Namespace::new("http://qudt.org/schema/qudt/").unwrap();
}

pub struct GraphBuilder {
    graph: LightGraph,
}

impl GraphBuilder {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            graph: LightGraph::new(),
        })
    }

    fn add_base_action_to_graph(
        &mut self,
        action_uri: &NsTerm<'_>,
        base: &BaseAction,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.graph.insert(action_uri, &ALLORES.get("AFX_0000622")?, base.startTime.as_str())?;
        self.graph.insert(action_uri, &ALLORES.get("AFR_0001606")?, base.method_name.as_str())?;
        self.graph.insert(action_uri, &ALLORES.get("AFR_0001723")?, base.equipment_name.as_str())?;
        self.graph.insert(action_uri, &CAT.get("localEquipmentName")?, base.sub_equipment_name.as_str())?;
        self.graph.insert(action_uri, &CAT.get("containerID")?, base.containerID.as_str())?;
        self.graph.insert(action_uri, &CAT.get("containerBarcode")?, base.containerBarcode.as_str())?;
        Ok(())
    }

    fn insert_measurement_to_graph(
        &mut self,
        action_uri: &NsTerm<'_>,
        property_uri: &NsTerm<'_>, // Property URI (e.g., "TemperatureTumbleStirrer")
        measurement: &Measurement,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let bnode_id = generate_unique_identifier().clone();
        let action_bnode = BnodeId::new_unchecked(bnode_id);
        self.graph.insert(action_uri, property_uri, &action_bnode)?;
        self.graph.insert(&action_bnode, &QUDT.get("unit")?, measurement.unit.as_str())?;
        self.graph.insert(&action_bnode, &QUDT.get("value")?, measurement.value)?;
        Ok(())
    }

    fn add_action_to_graph(
        &mut self,
        batch_uri: &NsTerm<'_>,
        action: &Action,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let action_id = generate_unique_identifier();
        let action_uri = EX.get(&action_id)?;
        self.graph.insert(&action_uri, &CAT.get("hasBatch")?, batch_uri)?;
        match action {
            Action::setTemperatureAction(action) => {
                self.graph.insert(&action_uri, &RDF.get("type")?, &CAT.get("setTemperatureAction")?)?;
                self.add_base_action_to_graph(&action_uri, &action.base)?;
                self.insert_measurement_to_graph(
                    &action_uri,
                    &CAT.get("temperatureShakerShape")?,
                    &action.TemperatureTumbleStirrer,
                )?;
                self.insert_measurement_to_graph(
                    &action_uri,
                    &CAT.get("temperatureTumbleStirrerShape")?,
                    &action.TemperatureShaker,
                )?;
            }
            Action::filtrateAction(action) => {
                self.graph.insert(&action_uri, &ALLORES.get("type")?, &CAT.get("AFRE_0000001")?)?;
                self.add_base_action_to_graph(&action_uri, &action.base)?;
            }
        }
        Ok(())
    }

    pub fn add_batch(&mut self, batch: &Batch) -> Result<(), Box<dyn std::error::Error>> {
        let batch_id = generate_unique_identifier();
        let batch_uri = EX.get(&batch_id)?;
        let rdf_type = RDF.get("type")?;

        self.graph.insert(
            &batch_uri,
            &rdf_type,
            &CAT.get("Batch")?,
        )?;
        self.graph.insert(
            &batch_uri,
            &SCHEMA.get("name")?,
            batch.batchID.as_str(),
        )?;

        for action in &batch.actions {
            self.add_action_to_graph(&batch_uri, action)?;
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
