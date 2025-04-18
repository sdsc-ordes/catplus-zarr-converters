use crate::{
    graph::{
        insert_into::{InsertIntoGraph, Link},
        namespaces::{alloprop, alloqual, allores, cat},
    },
    models::{
        core::{Chemical, Observation, Plate, Well},
        enums::ActionName,
    },
};
use anyhow;
use serde::{Deserialize, Serialize};
use sophia::{
    api::ns::{rdf, xsd},
    inmem::graph::LightGraph,
};
use sophia_api::term::{SimpleTerm, Term};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "Batch")]
pub struct BravoBatch {
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<BravoAction>>,
}

impl InsertIntoGraph for BravoBatch {
    fn insert_into(&self, graph: &mut LightGraph, _iri: SimpleTerm) -> anyhow::Result<()> {
        if let Some(actions) = &self.actions {
            for action in actions {
                let action_uri = action.get_uri();
                action.insert_into(graph, action_uri)?;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BravoAction {
    pub action_name: ActionName,
    pub start_time: String,
    pub ending_time: String,
    pub method_name: Option<String>,
    pub equipment_name: String,
    pub sub_equipment_name: Option<String>,
    pub speed_shaker: Option<Observation>,
    pub at_well: Option<BravoWell>,
    pub dispense_state: Option<String>,
    pub dispense_type: Option<String>,
    pub has_sample: Option<BravoSample>,
    pub temperature: Option<Observation>,
    pub volume_evaporation_final: Option<Observation>,
    pub has_solvent: Option<Solvent>,
    #[serde(rename = "SPMEprocess")]
    pub spme_process: Option<bool>,
    pub has_cartridge: Option<Cartridge>,
    pub start_duration: Option<Observation>,
    pub ending_duration: Option<Observation>,
    pub order: Option<String>,
}

impl InsertIntoGraph for BravoAction {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &self.action_name.iri().as_simple() as &dyn InsertIntoGraph),
            (allores::AFX_0000622, &(self.start_time.as_str() * xsd::dateTime).as_simple()),
            (allores::AFR_0002423, &(self.ending_time.as_str() * xsd::dateTime).as_simple()),
            (allores::AFR_0001606, &self.method_name.as_ref().clone().map(|s| s.as_simple())),
            (allores::AFR_0001723, &self.equipment_name.as_simple()),
            (cat::startDuration, &self.start_duration),
            (cat::endingDuration, &self.ending_duration),
            (
                cat::subEquipmentName,
                &self.sub_equipment_name.as_ref().clone().map(|s| s.as_simple()),
            ),
            // TODO: serialize to xsd::boolean and not string
            (
                cat::isSpmeProcess,
                &self
                    .spme_process
                    .as_ref()
                    .clone()
                    .as_ref()
                    .map(|s| s.to_string())
                    .as_ref()
                    .map(|s| s.as_str().as_simple()),
            ),
            (cat::speedInRPM, &self.speed_shaker),
            (cat::volumeEvaporationFinal, &self.volume_evaporation_final),
            (alloprop::AFX_0000060, &self.temperature),
            (cat::hasSample, &self.has_sample),
            (cat::hasSolvent, &self.has_solvent),
            (cat::hasWell, &self.at_well),
            (cat::hasCartridge, &self.has_cartridge),
            (cat::order, &self.order.as_ref().clone().map(|s| s.as_simple())),
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cartridge {
    pub cartridge_name: String,
    pub cartridge_composition: String,
}

impl InsertIntoGraph for Cartridge {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (prop, value) in [
            (rdf::type_, &cat::Cartridge.as_simple() as &dyn InsertIntoGraph),
            (cat::cartridgeName, &self.cartridge_name.as_simple()),
            (cat::cartridgeComposition, &self.cartridge_composition.as_simple()),
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: prop.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Solvent {
    pub has_chemical: Chemical,
    pub volume: Observation,
}

impl InsertIntoGraph for Solvent {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (prop, value) in [
            (rdf::type_, &cat::Solvent.as_simple() as &dyn InsertIntoGraph),
            (cat::hasChemical, &self.has_chemical),
            (cat::volume, &self.volume),
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: prop.as_simple(), target_iri: None },
            )?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BravoWell {
    #[serde(flatten)]
    pub has_plate: Plate,
    pub position: String,
    pub product_identification: ProductIdentification,
}

impl InsertIntoGraph for BravoWell {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::Well.as_simple() as &dyn InsertIntoGraph),
            (cat::hasPlate, &self.has_plate),
            (allores::AFR_0002240, &self.position.as_simple()),
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductIdentification {
    #[serde(rename = "sampleID")]
    pub sample_id: String,
    pub peak_identifier: String,
}

impl InsertIntoGraph for ProductIdentification {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::Product.as_simple() as &dyn InsertIntoGraph),
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BravoSample {
    #[serde(flatten)]
    pub has_well: Well,
}

impl InsertIntoGraph for BravoSample {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (prop, value) in [
            (rdf::type_, &cat::Sample.as_simple() as &dyn InsertIntoGraph),
            (cat::hasWell, &self.has_well),
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: prop.as_simple(), target_iri: None },
            )?;
        }

        Ok(())
    }
}
