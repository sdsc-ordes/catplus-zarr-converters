use crate::{
    graph::{
        namespaces::*,
        utils::{generate_bnode_term, generate_uri_term},
    },
    parser::actions::{
        Action, ActionName, Batch, Chemical, ContainerInfo, ContainerPosition, Observation, Sample,
        SampleItem,
    },
};
use sophia::{
    api::{
        graph::MutableGraph,
        ns::xsd,
        serializer::{Stringifier, TripleSerializer},
    },
    inmem::graph::LightGraph,
};
use sophia_api::{ns::NsTerm, term::SimpleTerm};
use sophia_turtle::serializer::turtle::{TurtleConfig, TurtleSerializer};

pub struct GraphBuilder {
    graph: LightGraph,
}

impl GraphBuilder {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            graph: LightGraph::new(),
        })
    }

    fn add_date_time_to_graph(
        &mut self,
        subject: &SimpleTerm,
        predicate: &NsTerm<'_>,
        date_time: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let object = date_time * xsd::dateTime;
        self.graph.insert(subject, predicate, &object)?;
        Ok(())
    }

    fn add_container_info_to_graph(
        &mut self,
        subject: &SimpleTerm,
        container_info: &ContainerInfo,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.graph.insert(
            subject,
            &CAT.get("containerID")?,
            container_info.container_id.as_str(),
        )?;
        self.graph.insert(
            subject,
            &CAT.get("containerBarcode")?,
            container_info.container_barcode.as_str(),
        )?;
        Ok(())
    }

    fn insert_observation_to_graph(
        &mut self,
        subject: &SimpleTerm,
        property_term: &NsTerm<'_>,
        observation: &Observation,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let observation_term = generate_bnode_term();
        self.graph
            .insert(subject, property_term, &observation_term)?;
        self.graph.insert(
            &observation_term,
            &QUDT.get("unit")?,
            observation.unit.as_str(),
        )?;
        self.graph
            .insert(&observation_term, &QUDT.get("value")?, observation.value)?;
        Ok(())
    }

    fn insert_container_position_to_graph(
        &mut self,
        subject: &SimpleTerm,
        container_position: &ContainerPosition,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let container_position_term = generate_bnode_term();
        self.graph.insert(
            subject,
            &CAT.get("hasContainerPositionAndQuantity")?,
            &container_position_term,
        )?;
        self.graph.insert(
            &container_position_term,
            &RDF.get("type")?,
            &CAT.get("ContainerPositionAndQuantity")?,
        )?;
        self.graph.insert(
            &container_position_term,
            &ALLORES.get("AFR_0002240")?,
            container_position.position.as_str(),
        )?;
        self.insert_observation_to_graph(
            &container_position_term,
            &QUDT.get("quantity")?,
            &container_position.quantity,
        )?;
        Ok(())
    }

    fn add_chemical_to_graph(
        &mut self,
        subject: &SimpleTerm,
        chemical: &Chemical,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let chemical_term: SimpleTerm = generate_uri_term()?;
        self.graph
            .insert(subject, &CAT.get("has_chemical")?, &chemical_term)?;
        self.graph
            .insert(&chemical_term, &RDF.get("type")?, &OBO.get("CHEBI_25367")?)?;
        self.graph.insert(
            &chemical_term,
            &PURL.get("identifier")?,
            chemical.chemical_id.as_str(),
        )?;
        self.graph.insert(
            &chemical_term,
            &CAT.get("chemicalName")?,
            chemical.chemical_name.as_str(),
        )?;
        self.graph.insert(
            &chemical_term,
            &CAT.get("casNumber")?,
            chemical.cas_number.as_str(),
        )?;
        self.graph.insert(
            &chemical_term,
            &ALLORES.get("AFR_0002295")?,
            chemical.smiles.as_str(),
        )?;
        let molecular_mass = chemical.molecular_mass.value.to_string();
        self.graph.insert(
            &chemical_term,
            &ALLORES.get("AFR_0002294")?,
            &*molecular_mass,
        )?;
        Ok(())
    }

    fn add_sample_item_to_graph(
        &mut self,
        subject: &SimpleTerm,
        sample_item: &SampleItem,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sample_item_term = generate_bnode_term();
        self.graph
            .insert(&sample_item_term, &RDF.get("type")?, &CAT.get("Sample")?)?;
        self.graph
            .insert(subject, &CAT.get("hasSample")?, &sample_item_term)?;
        self.graph.insert(
            &sample_item_term,
            &CAT.get("role")?,
            sample_item.role.as_str(),
        )?;
        if let Some(expected_datum) = &sample_item.expected_datum {
            self.insert_observation_to_graph(
                &sample_item_term,
                &CAT.get("expectedDatum")?,
                expected_datum,
            )?;
        }
        self.graph.insert(
            &sample_item_term,
            &CAT.get("role")?,
            sample_item.role.as_str(),
        )?;
        self.graph.insert(
            &sample_item_term,
            &PURL.get("identifier")?,
            sample_item.sample_id.as_str(),
        )?;
        self.graph.insert(
            &sample_item_term,
            &ALLOQUAL.get("AFQ_0000111")?,
            sample_item.physical_state.as_str(),
        )?;
        self.graph.insert(
            &sample_item_term,
            &CAT.get("internalBarCode")?,
            sample_item.internal_bar_code.as_str(),
        )?;
        self.add_chemical_to_graph(&sample_item_term, &sample_item.has_chemical)?;
        Ok(())
    }

    fn add_sample_to_graph(
        &mut self,
        subject: &SimpleTerm,
        sample: &Sample,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sample_term = generate_bnode_term();
        self.graph
            .insert(subject, &CAT.get("hasSample")?, &sample_term)?;
        self.graph
            .insert(&sample_term, &RDF.get("type")?, &CAT.get("Sample")?)?;
        self.add_container_info_to_graph(&sample_term, &sample.container)?;
        self.insert_observation_to_graph(
            &sample_term,
            &CAT.get("expectedDatum")?,
            &sample.expected_datum,
        )?;
        self.graph.insert(
            &sample_term,
            &CAT.get("vialShape")?,
            sample.vial_type.as_str(),
        )?;
        self.graph.insert(
            &sample_term,
            &ALLORES.get("AFR_0002464")?,
            sample.vial_id.as_str(),
        )?;
        self.graph
            .insert(&sample_term, &CAT.get("role")?, sample.role.as_str())?;
        for sample_item in &sample.has_sample {
            self.add_sample_item_to_graph(&sample_term, &sample_item)?;
        }
        Ok(())
    }

    fn add_action_type_to_graph(
        &mut self,
        subject: &SimpleTerm,
        action: &Action,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match action.action_name {
            ActionName::AddAction => {
                self.graph
                    .insert(subject, &RDF.get("type")?, &CAT.get("AddAction")?)?;
            }
            ActionName::setTemperatureAction => {
                self.graph.insert(
                    subject,
                    &RDF.get("type")?,
                    &CAT.get("setTemperatureAction")?,
                )?;
            }
            _ => {
                self.graph
                    .insert(subject, &RDF.get("type")?, &ALLORES.get("AFRE_0000001")?)?;
            }
        }
        Ok(())
    }

    fn add_action_to_graph(
        &mut self,
        subject: &SimpleTerm,
        action: &Action,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let action_term: SimpleTerm = generate_uri_term()?;
        self.graph
            .insert(&action_term, &CAT.get("hasBatch")?, subject)?;
        self.add_date_time_to_graph(
            &action_term,
            &ALLORES.get("AFX_0000622")?,
            action.start_time.as_str(),
        )?;
        self.add_date_time_to_graph(
            &action_term,
            &ALLORES.get("AFR_0002423")?,
            &action.ending_time.as_str(),
        )?;
        self.graph.insert(
            &action_term,
            &ALLORES.get("AFR_0001606")?,
            action.method_name.as_str(),
        )?;
        self.graph.insert(
            &action_term,
            &ALLORES.get("AFR_0001723")?,
            action.equipment_name.as_str(),
        )?;
        self.graph.insert(
            &action_term,
            &CAT.get("localEquipmentName")?,
            action.sub_equipment_name.as_str(),
        )?;
        if let Some(container_info) = &action.container_info {
            self.add_container_info_to_graph(&action_term, &container_info)?;
        }
        if let Some(temperature_shaker) = &action.temperature_shaker {
            self.insert_observation_to_graph(
                &action_term,
                &CAT.get("temperatureShakerShape")?,
                temperature_shaker,
            )?;
        }
        if let Some(temperature_tumble_stirrer) = &action.temperature_tumble_stirrer {
            self.insert_observation_to_graph(
                &action_term,
                &CAT.get("temperatureTumbleStirrerShape")?,
                temperature_tumble_stirrer,
            )?;
        }
        if let Some(speed_shaker) = &action.speed_shaker {
            self.insert_observation_to_graph(&action_term, &CAT.get("speedInRPM")?, speed_shaker)?;
        }
        if let Some(dispense_type) = &action.dispense_type {
            self.graph.insert(
                &action_term,
                &CAT.get("dispenseType")?,
                dispense_type.as_str(),
            )?;
        }
        if let Some(dispense_state) = &action.dispense_state {
            self.graph.insert(
                &action_term,
                &ALLOQUAL.get("AFQ_0000111")?,
                dispense_state.as_str(),
            )?;
        }
        if let Some(container_positions) = &action.has_container_position_and_quantity {
            for container_position in container_positions {
                self.insert_container_position_to_graph(&action_term, container_position)?;
            }
        }
        if let Some(sample) = &action.has_sample {
            self.add_sample_to_graph(&action_term, sample)?;
        }
        self.add_action_type_to_graph(&action_term, action)?;
        Ok(())
    }

    pub fn add_batch(&mut self, batch: &Batch) -> Result<(), Box<dyn std::error::Error>> {
        let batch_term = generate_bnode_term();
        self.graph
            .insert(&batch_term, RDF.get("type")?, &CAT.get("Batch")?)?;
        self.graph
            .insert(&batch_term, &SCHEMA.get("name")?, batch.batch_id.as_str())?;
        for action in &batch.actions {
            self.add_action_to_graph(&batch_term, action)?;
        }
        Ok(())
    }

    pub fn serialize_to_turtle(&self) -> Result<String, Box<dyn std::error::Error>> {
        let prefix_map = generate_prefix_map();
        let config = TurtleConfig::default()
            .with_pretty(true)
            .with_own_prefix_map(prefix_map);
        let mut serializer = TurtleSerializer::new_stringifier_with_config(config);
        serializer.serialize_graph(&self.graph)?;
        Ok(serializer.as_str().to_string())
    }
}
