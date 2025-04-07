use crate::{
    graph::{
        insert_into::{InsertIntoGraph, Link},
        namespaces::{alloproc, alloqual, allores, cat, purl},
    },
    models::{
        core::{Observation, Plate, Sample, Well},
        enums::ActionName,
    },
};
use anyhow;
use serde::{Deserialize, Serialize};
use sophia::{
    api::ns::{rdf, xsd},
    inmem::graph::LightGraph,
};
use sophia_api::{
    graph::MutableGraph,
    term::{SimpleTerm, Term},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "Batch")]
pub struct SynthBatch {
    #[serde(rename = "batchID")]
    pub batch_id: String,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<SynthAction>>,
}

impl InsertIntoGraph for SynthBatch {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::Batch.as_simple() as &dyn InsertIntoGraph),
            (purl::identifier, &self.batch_id.as_simple()),
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }

        // NOTE: for actions, the direction is reversed (action hasbatch batch)
        if let Some(actions) = &self.actions {
            for action in actions {
                let action_uri = action.get_uri();
                graph.insert(&action_uri, cat::hasBatch.as_simple(), iri.clone())?;
                action.insert_into(graph, action_uri)?;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "Action")]
pub struct SynthAction {
    pub action_name: ActionName,
    pub start_time: String,
    pub ending_time: String,
    pub method_name: String,
    pub equipment_name: String,
    pub sub_equipment_name: String,
    #[serde(flatten)]
    pub has_plate: Option<Plate>,
    pub speed_shaker: Option<Observation>,
    pub has_well: Option<Vec<Well>>,
    pub dispense_state: Option<String>,
    pub dispense_type: Option<String>,
    pub has_sample: Option<Sample>,
    pub speed_tumble_stirrer: Option<Observation>,
    pub temperature_tumble_stirrer: Option<Observation>,
    pub temperature_shaker: Option<Observation>,
    pub pressure_measurement: Option<Observation>,
    pub vacuum: Option<Observation>,
}

impl InsertIntoGraph for SynthAction {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &self.action_name.iri().as_simple() as &dyn InsertIntoGraph),
            (allores::AFX_0000622, &(self.start_time.as_str() * xsd::dateTime).as_simple()),
            (allores::AFR_0002423, &(self.ending_time.as_str() * xsd::dateTime).as_simple()),
            (allores::AFR_0001606, &self.method_name.as_simple()),
            (allores::AFR_0001723, &self.equipment_name.as_simple()),
            (cat::subEquipmentName, &self.sub_equipment_name.as_simple()),
            (cat::speedInRPM, &self.speed_shaker),
            (cat::temperatureTumbleStirrerShape, &self.temperature_tumble_stirrer),
            (cat::speedTumbleStirrerShape, &self.speed_tumble_stirrer),
            (cat::vacuum, &self.vacuum),
            (cat::temperatureShakerShape, &self.temperature_shaker),
            (alloproc::AFP_0002677, &self.pressure_measurement),
            (cat::hasSample, &self.has_sample),
            (cat::hasWell, &self.has_well),
            (cat::hasPlate, &self.has_plate),
            (alloqual::AFQ_0000111, &self.dispense_state.as_ref().clone().map(|s| s.as_simple())),
            (cat::dispenseType, &self.dispense_type.as_ref().clone().map(|s| s.as_simple())),
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }

        Ok(())
    }
}
