#[rustfmt::skip]
// The structure follows the input data as descibed in the
// https://github.com/sdsc-ordes/catplus-ontology see here for the expected Synth input data:
// https://github.com/sdsc-ordes/catplus-ontology/tree/96091fd2e75e03de8a4c4d66ad502b2db27998bd/json-file/1-Synth
use crate::{
    graph::{
        insert_into::{InsertIntoGraph, Link},
        namespaces::{alloproc, allocom, allohdf, alloqual, allores, cat, obo, purl, qudt, schema},
    },
    models::enums::{ActionName, Unit},
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
pub struct Campaign {
    pub campaign_name: String,
    pub description: String,
    #[serde(rename = "objective")]
    pub generic_objective: String,
    pub campaign_class: String,
    #[serde(rename = "type")]
    pub campaign_type: String,
    pub reference: String,
    pub has_objective: Option<Objective>,
    pub has_batch: Batch,
    pub has_chemical: Option<Vec<Chemical>>,
}

impl InsertIntoGraph for Campaign {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::Campaign.as_simple() as &dyn InsertIntoGraph),
            (schema::name, &self.campaign_name.as_simple()),
            (schema::description, &self.description.as_simple()),
            (cat::genericObjective, &self.generic_objective.as_simple()),
            (cat::campaignClass, &self.campaign_class.as_simple()),
            (cat::campaignType, &self.campaign_type.as_simple()),
            (allores::AFR_0002764, &self.reference.as_simple()),
            (cat::hasObjective, &self.has_objective),
            (cat::hasBatch, &self.has_batch),
            (cat::hasChemical, &self.has_chemical),
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
pub struct Objective {
    pub criteria: String,
    pub condition: String,
    pub description: String,
    pub objective_name: String,
}

impl InsertIntoGraph for Objective {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &obo::IAO_0000005.as_simple()),
            (schema::name, &self.objective_name.as_simple()),
            (schema::description, &self.description.as_simple()),
            (cat::criteria, &self.criteria.as_simple()),
            (allocom::AFC_0000090, &self.condition.as_simple()),
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct CampaignWrapper {
    #[serde(rename = "hasCampaign")]
    pub has_campaign: Campaign,
}
impl InsertIntoGraph for CampaignWrapper {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        self.has_campaign.insert_into(graph, iri)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Batch {
    #[serde(rename = "batchID")]
    pub batch_id: Option<String>,
    #[serde(rename = "Actions")]
    pub actions: Option<Vec<Action>>,
    pub batch_name: Option<String>,
    pub reaction_type: Option<String>,
    pub reaction_name: Option<String>,
    pub optimization_type: Option<String>,
    pub link: Option<String>,
}

impl InsertIntoGraph for Batch {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::Batch.as_simple() as &dyn InsertIntoGraph),
            (purl::identifier, &self.batch_id.as_ref().clone().map(|s| s.as_simple())),
            (schema::name, &self.batch_name.as_ref().clone().map(|s| s.as_simple())),
            (allohdf::HardLink, &self.link.as_ref().clone().map(|s| s.as_simple())),
            (cat::reactionType, &self.reaction_type.as_ref().clone().map(|s| s.as_simple())),
            (cat::reactionName, &self.reaction_name.as_ref().clone().map(|s| s.as_simple())),
            (
                cat::optimizationType,
                &self.optimization_type.as_ref().clone().map(|s| s.as_simple()),
            ),
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
pub struct Action {
    pub action_name: ActionName,
    pub start_time: String,
    pub ending_time: String,
    pub method_name: Option<String>,
    pub equipment_name: String,
    pub sub_equipment_name: Option<String>,
    #[serde(flatten)]
    pub has_plate: Option<Plate>,
    pub speed_shaker: Option<Observation>,
    pub has_well: Option<Vec<Well>>,
    pub at_well: Option<Well>,
    pub dispense_state: Option<String>,
    pub dispense_type: Option<String>,
    pub has_sample: Option<Sample>,
    pub speed_tumble_stirrer: Option<Observation>,
    pub temperature_tumble_stirrer: Option<Observation>,
    pub temperature_shaker: Option<Observation>,
    pub temperature: Option<Observation>,
    pub pressure_measurement: Option<Observation>,
    pub vacuum: Option<Observation>,
    pub volume_evaporation_final: Option<Observation>,
    pub has_solvent: Option<Solvent>,
    #[serde(rename = "SPMEprocess")]
    pub spme_process: Option<bool>,
    pub has_cartridge: Option<Cartridge>,
    pub start_duration: Option<Observation>,
    pub ending_duration: Option<Observation>,
    pub order: Option<String>,
}

impl InsertIntoGraph for Action {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &self.action_name.iri().as_simple() as &dyn InsertIntoGraph),
            (allores::AFX_0000622, &(self.start_time.as_str() * xsd::dateTime).as_simple()),
            (allores::AFR_0002423, &(self.ending_time.as_str() * xsd::dateTime).as_simple()),
            (allores::AFR_0001606, &self.method_name.as_ref().clone().map(|s| s.as_simple())),
            (allores::AFR_0001723, &self.equipment_name.as_simple()),
            (cat::subEquipmentName, &self.sub_equipment_name.as_ref().clone().map(|s| s.as_simple())),
            // TODO: serialize to xsd::boolean and not string
            (cat::isSpmeProcess, &self.spme_process.as_ref().clone().as_ref().map(|s| s.to_string()).as_ref().map(|s| s.as_str().as_simple())),
            (cat::speedInRPM, &self.speed_shaker),
            (cat::volumeEvaporationFinal, &self.volume_evaporation_final),
            (cat::temperatureTumbleStirrerShape, &self.temperature_tumble_stirrer),
            (cat::speedTumbleStirrerShape, &self.speed_tumble_stirrer),
            (cat::vacuum, &self.vacuum),
            (cat::temperatureShakerShape, &self.temperature_shaker),
            (cat::temperatureInDegC, &self.temperature),
            (alloproc::AFP_0002677, &self.pressure_measurement),
            (cat::hasSample, &self.has_sample),
            (cat::hasSolvent, &self.has_solvent),
            (cat::hasWell, &self.has_well),
            (cat::hasWell, &self.at_well),
            (cat::hasCartridge, &self.has_cartridge),
            (cat::order, &self.order.as_ref().clone().map(|s| s.as_simple())),
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
pub struct Plate {
    #[serde(rename = "containerID")]
    pub container_id: String,
    pub container_barcode: Option<String>,
}

impl InsertIntoGraph for Plate {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (prop, value) in [
            (rdf::type_, &cat::Plate.as_simple() as &dyn InsertIntoGraph),
            (cat::containerID, &self.container_id.as_simple() as &dyn InsertIntoGraph),
            (
                cat::containerBarcode,
                &self.container_barcode.as_ref().clone().map(|s| s.as_simple()),
            ),
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
pub struct Observation {
    pub value: f64,
    pub unit: Unit,
    pub error_margin: Option<ErrorMargin>,
}

/// Implementation for concrete [Observation].
impl InsertIntoGraph for Observation {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (prop, value) in [
            (rdf::type_, &cat::Observation.as_simple() as &dyn InsertIntoGraph),
            (qudt::unit, &self.unit.iri().as_simple() as &dyn InsertIntoGraph),
            (qudt::value, &self.value.as_simple()),
            (cat::errorMargin, &self.error_margin),
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
pub struct ErrorMargin {
    pub value: f64,
    pub unit: Unit,
}

/// Implementation for concrete [Observation].
impl InsertIntoGraph for ErrorMargin {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (prop, value) in [
            (rdf::type_, &cat::errorMargin.as_simple() as &dyn InsertIntoGraph),
            (qudt::unit, &self.unit.iri().as_simple() as &dyn InsertIntoGraph),
            (qudt::value, &self.value.as_simple()),
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
pub struct Sample {
    #[serde(flatten)]
    pub has_plate: Plate,
    #[serde(rename = "vialID")]
    pub vial_id: Option<String>,
    pub vial_type: Option<String>,
    pub role: Option<String>,
    pub expected_datum: Option<Observation>,
    pub has_sample: Option<Vec<SampleItem>>,
}

impl InsertIntoGraph for Sample {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (prop, value) in [
            (rdf::type_, &cat::Sample.as_simple() as &dyn InsertIntoGraph),
            (cat::hasPlate, &self.has_plate),
            (cat::role, &self.role.as_ref().clone().map(|s| s.as_simple())),
            (cat::vialShape, &self.vial_type.as_ref().clone().map(|s| s.as_simple())),
            (allores::AFR_0002464, &self.vial_id.as_ref().clone().map(|s| s.as_simple())),
            (cat::expectedDatum, &self.expected_datum),
            (cat::hasSample, &self.has_sample),
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
pub struct SampleItem {
    #[serde(rename = "sampleID")]
    pub sample_id: String,
    pub role: String,
    pub internal_bar_code: String,
    pub expected_datum: Option<Observation>,
    pub measured_quantity: Option<Observation>,
    pub concentration: Option<Observation>,
    pub physical_state: String,
    pub has_chemical: Chemical,
}

impl InsertIntoGraph for SampleItem {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (prop, value) in [
            (rdf::type_, &cat::Sample.as_simple() as &dyn InsertIntoGraph),
            (purl::identifier, &self.sample_id.as_simple()),
            (cat::role, &self.role.as_simple()),
            (cat::internalBarCode, &self.internal_bar_code.as_simple()),
            (alloqual::AFQ_0000111, &self.physical_state.as_simple()),
            (cat::expectedDatum, &self.expected_datum),
            (cat::measuredQuantity, &self.measured_quantity),
            (allores::AFR_0002036, &self.concentration),
            (cat::hasChemical, &self.has_chemical),
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
pub struct Chemical {
    #[serde(rename = "chemicalID")]
    pub chemical_id: String,
    pub chemical_name: String,
    #[serde(rename = "CASNumber")]
    pub cas_number: Option<String>,
    pub molecular_mass: Observation,
    pub smiles: String,
    pub swiss_cat_number: Option<String>,
    #[serde(rename = "Inchi")]
    pub inchi: String,
    pub keywords: Option<String>,
    pub molecular_formula: String,
    pub density: Option<Observation>,
}

impl InsertIntoGraph for Chemical {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (prop, value) in [
            (rdf::type_, &obo::CHEBI_25367.as_simple() as &dyn InsertIntoGraph),
            (purl::identifier, &self.chemical_id.as_simple()),
            (allores::AFR_0002292, &self.chemical_name.as_simple()),
            (allores::AFR_0001952, &self.molecular_formula.as_simple()),
            (allores::AFR_0002295, &self.smiles.as_simple()),
            (allores::AFR_0002294, &self.molecular_mass),
            (allores::AFR_0002296, &self.inchi.as_simple()),
            (cat::casNumber, &self.cas_number.as_ref().clone().map(|s| s.as_simple())),
            (cat::swissCatNumber, &self.swiss_cat_number.as_ref().clone().map(|s| s.as_simple())),
            (schema::keywords, &self.keywords.as_ref().clone().map(|s| s.as_simple())),
            (obo::PATO_0001019, &self.density),
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
pub struct Well {
    #[serde(flatten)]
    pub has_plate: Plate,
    pub position: String,
    pub quantity: Option<Observation>,
}

impl InsertIntoGraph for Well {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::Well.as_simple() as &dyn InsertIntoGraph),
            (cat::hasPlate, &self.has_plate),
            (allores::AFR_0002240, &self.position.as_simple()),
            (qudt::quantity, &self.quantity),
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sophia::iri::IriRef;
    use sophia_api::term::Term;

    use crate::{
        graph::{graph_builder::GraphBuilder, insert_into::InsertIntoGraph},
        models::{ErrorMargin, Observation},
    };

    #[test]
    fn test_observation_to_triples() -> anyhow::Result<()> {
        let observation = Observation {
            value: 42.0,
            unit: Unit::DegC,
            error_margin: Some(ErrorMargin { value: 0.5, unit: Unit::DegC }),
        };

        let mut b = GraphBuilder::new();
        let i = IriRef::new_unchecked("http://test.com/my-obersvation");
        observation.insert_into(&mut b.graph, i.as_simple())?;
        println!("Graph\n{}", b.serialize_to_turtle().unwrap());

        Ok(())
    }
}
