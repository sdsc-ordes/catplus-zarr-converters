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
    pub batch_id: String,
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
            (schema::name, &self.batch_id.as_simple()),
            (allohdf::HardLink, &self.link.as_ref().clone().map(|s| s.as_simple())),
            (cat::reactionType, &self.reaction_type.as_ref().clone().map(|s| s.as_simple())),
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
    pub method_name: String,
    pub equipment_name: String,
    pub sub_equipment_name: String,
    #[serde(flatten)]
    pub container_info: Option<ContainerInfo>,
    pub speed_shaker: Option<Observation>,
    pub has_container_position_and_quantity: Option<Vec<ContainerPositionQuantityItem>>,
    pub dispense_state: Option<String>,
    pub dispense_type: Option<String>,
    pub has_sample: Option<Sample>,
    pub speed_tumble_stirrer: Option<Observation>,
    pub temperature_tumble_stirrer: Option<Observation>,
    pub temperature_shaker: Option<Observation>,
    pub pressure_measurement: Option<Observation>,
}

impl InsertIntoGraph for Action {
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
            (cat::temperatureShakerShape, &self.temperature_shaker),
            (alloproc::AFP_0002677, &self.pressure_measurement),
            (cat::hasSample, &self.has_sample),
            (cat::hasContainerPositionAndQuantity, &self.has_container_position_and_quantity),
            (alloqual::AFQ_0000111, &self.dispense_state.as_ref().clone().map(|s| s.as_simple())),
            (cat::dispenseType, &self.dispense_type.as_ref().clone().map(|s| s.as_simple())),
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }

        // NOTE: for container_info, we attach triples directly to the action
        let _ = &self.container_info.insert_into(graph, iri.clone())?;

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerInfo {
    #[serde(rename = "containerID")]
    pub container_id: String,
    pub container_barcode: String,
}

impl InsertIntoGraph for ContainerInfo {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (prop, value) in [
            (&cat::containerID, &self.container_id.as_simple() as &dyn InsertIntoGraph),
            (&cat::containerBarcode, &self.container_barcode.as_simple()),
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
    pub container: ContainerInfo,
    #[serde(rename = "vialID")]
    pub vial_id: String,
    pub vial_type: String,
    pub role: String,
    pub expected_datum: Observation,
    pub has_sample: Vec<SampleItem>,
}

impl InsertIntoGraph for Sample {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (prop, value) in [
            (rdf::type_, &cat::Sample.as_simple() as &dyn InsertIntoGraph),
            (cat::role, &self.role.as_simple()),
            (cat::vialShape, &self.vial_type.as_simple()),
            (allores::AFR_0002464, &self.vial_id.as_simple()),
            (cat::expectedDatum, &self.expected_datum),
            (cat::hasSample, &self.has_sample),
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: prop.as_simple(), target_iri: None },
            )?;
        }

        // NOTE: for container_info, we attach triples directly to the sample
        let _ = &self.container.insert_into(graph, iri.clone())?;

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
            (cat::chemicalName, &self.chemical_name.as_simple()),
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
pub struct ContainerPositionQuantityItem {
    #[serde(rename = "containerID")]
    pub container_id: String,
    pub position: String,
    pub quantity: Observation,
}

impl InsertIntoGraph for ContainerPositionQuantityItem {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::ContainerPositionAndQuantity.as_simple() as &dyn InsertIntoGraph),
            (cat::containerID, &self.container_id.as_simple()),
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeakList {
    pub peak : Vec<Peak>,
}

impl InsertIntoGraph for PeakList {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::PeakList.as_simple() as &dyn InsertIntoGraph),
            (cat::Peak, &self.peak)
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
pub struct Peak{
    #[serde(rename="@index")]
    pub index: i64,
    #[serde(rename = "peak area")]
    pub peak_area: Measurement,
    #[serde(rename = "retention time")]
    pub retention_time: Measurement,
    #[serde(rename = "peakIdentifier")]
    pub peak_identifier: String,
    #[serde(rename = "peak start")]
    pub peak_start: Measurement,
    #[serde(rename = "peak end")]
    pub peak_end: Measurement,
    #[serde(rename = "peak height")]
    pub peak_height: Measurement,
    #[serde(rename = "relative peak height")]
    pub relative_peak_height: Measurement,
    #[serde(rename = "peak value at start")]
    pub peak_value_at_start: Measurement,
    #[serde(rename = "peak value at end")]
    pub peak_value_at_end: Measurement,
    #[serde(rename = "relative peak area")]
    pub relative_peak_area: Measurement,
}

impl InsertIntoGraph for Peak {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &allores::AFR_0000413.as_simple() as &dyn InsertIntoGraph),
            (allores::AFR_0001073, &self.peak_area),
            (allores::AFR_0001089, &self.retention_time),
            //(allores::AFR_0001164, &self.peak_identifier),
            (allores::AFR_0001178, &self.peak_start),
            (allores::AFR_0001180, &self.peak_end),
            (allores::AFR_0000948, &self.peak_height),
            (allores::AFR_0000949, &self.relative_peak_height),
            (allores::AFR_0001179, &self.peak_value_at_start),
            (allores::AFR_0001181, &self.peak_value_at_end),
            (allores::AFR_0001165, &self.relative_peak_area)
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
pub struct Measurement{
    pub value: f64,
    pub unit: Unit,
}

impl InsertIntoGraph for Measurement {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (prop, value) in [
            (rdf::type_, &cat::Measurement.as_simple() as &dyn InsertIntoGraph),
            (qudt::unit, &self.unit.iri().as_simple() as &dyn InsertIntoGraph),
            (qudt::value, &self.value.as_simple())
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: prop.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct PeakArea {
//     pub measurement: Measurement,
// }
// impl InsertIntoGraph for PeakArea {
//     fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
//         for (prop, value) in [
//             (rdf::type_, &cat::PeakArea.as_simple() as &dyn InsertIntoGraph),
//             (cat::Measurement, &self.measurement)
//         ] {
//             value.attach_into(
//                 graph,
//                 Link { source_iri: iri.clone(), pred: prop.as_simple(), target_iri: None },
//             )?;
//         }
//         Ok(())
//     }
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct RetentionTime {
//     pub measurement: Measurement,
// }
// impl InsertIntoGraph for RetentionTime {
//     fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
//         for (prop, value) in [
//             (rdf::type_, &cat::RetentionTime.as_simple() as &dyn InsertIntoGraph),
//             (cat::Measurement, &self.measurement)
//         ] {
//             value.attach_into(
//                 graph,
//                 Link { source_iri: iri.clone(), pred: prop.as_simple(), target_iri: None },
//             )?;
//         }
//         Ok(())
//     }
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct PeakStart {
//     pub measurement: Measurement,
// }
// impl InsertIntoGraph for PeakStart {
//     fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
//         for (prop, value) in [
//             (rdf::type_, &cat::PeakStart.as_simple() as &dyn InsertIntoGraph),
//             (cat::Measurement, &self.measurement)
//         ] {
//             value.attach_into(
//                 graph,
//                 Link { source_iri: iri.clone(), pred: prop.as_simple(), target_iri: None },
//             )?;
//         }
//         Ok(())
//     }
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct PeakEnd {
//     pub measurement: Measurement,
// }
// impl InsertIntoGraph for PeakEnd {
//     fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
//         for (prop, value) in [
//             (rdf::type_, &cat::PeakEnd.as_simple() as &dyn InsertIntoGraph),
//             (cat::Measurement, &self.measurement)
//         ] {
//             value.attach_into(
//                 graph,
//                 Link { source_iri: iri.clone(), pred: prop.as_simple(), target_iri: None },
//             )?;
//         }
//         Ok(())
//     }
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct RelativePeakHeight {
//     pub measurement: Measurement,
// }
// impl InsertIntoGraph for RelativePeakHeight {
//     fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
//         for (prop, value) in [
//             (rdf::type_, &cat::RelativePeakHeight.as_simple() as &dyn InsertIntoGraph),
//             (cat::Measurement, &self.measurement)
//         ] {
//             value.attach_into(
//                 graph,
//                 Link { source_iri: iri.clone(), pred: prop.as_simple(), target_iri: None },
//             )?;
//         }
//         Ok(())
//     }
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct PeakHeight {
//     pub measurement: Measurement,
// }
// impl InsertIntoGraph for PeakHeight {
//     fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
//         for (prop, value) in [
//             (rdf::type_, &cat::PeakHeight.as_simple() as &dyn InsertIntoGraph),
//             (cat::Measurement, &self.measurement)
//         ] {
//             value.attach_into(
//                 graph,
//                 Link { source_iri: iri.clone(), pred: prop.as_simple(), target_iri: None },
//             )?;
//         }
//         Ok(())
//     }
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct RelativePeakArea {
//     pub measurement: Measurement,
// }
// impl InsertIntoGraph for RelativePeakArea {
//     fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
//         for (prop, value) in [
//             (rdf::type_, &cat::RelativePeakArea.as_simple() as &dyn InsertIntoGraph),
//             (cat::Measurement, &self.measurement)
//         ] {
//             value.attach_into(
//                 graph,
//                 Link { source_iri: iri.clone(), pred: prop.as_simple(), target_iri: None },
//             )?;
//         }
//         Ok(())
//     }
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct PeakValueAtStart {
//     pub measurement: Measurement,
// }
// impl InsertIntoGraph for PeakValueAtStart {
//     fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
//         for (prop, value) in [
//             (rdf::type_, &cat::PeakValueAtStart.as_simple() as &dyn InsertIntoGraph),
//             (cat::Measurement, &self.measurement)
//         ] {
//             value.attach_into(
//                 graph,
//                 Link { source_iri: iri.clone(), pred: prop.as_simple(), target_iri: None },
//             )?;
//         }
//         Ok(())
//     }
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct PeakValueAtEnd {
//     pub measurement: Measurement,
// }
// impl InsertIntoGraph for PeakValueAtEnd {
//     fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
//         for (prop, value) in [
//             (rdf::type_, &cat::PeakValueAtEnd.as_simple() as &dyn InsertIntoGraph),
//             (cat::Measurement, &self.measurement)
//         ] {
//             value.attach_into(
//                 graph,
//                 Link { source_iri: iri.clone(), pred: prop.as_simple(), target_iri: None },
//             )?;
//         }
//         Ok(())
//     }
// }

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

