#[rustfmt::skip]
// The structure follows the input data as descibed in the
// https://github.com/sdsc-ordes/catplus-ontology see here for the expected Synth input data:
// https://github.com/sdsc-ordes/catplus-ontology/tree/96091fd2e75e03de8a4c4d66ad502b2db27998bd/json-file/1-Synth
use crate::{
    graph::{
        insert_into::{InsertIntoGraph, Link},
        namespaces::{alloqual, allores, cat, obo, purl, qudt, schema},
    },
    models::enums::Unit,
};
use anyhow;
use serde::{Deserialize, Serialize};
use sophia::{api::ns::rdf, inmem::graph::LightGraph};
use sophia_api::term::{SimpleTerm, Term};

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
            (cat::hasPlate, &self.has_plate),
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
    pub quantity: Observation,
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
