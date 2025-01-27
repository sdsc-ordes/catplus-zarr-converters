use std::{fmt, intrinsics::mir::mir};

// The structure follows the input data as descibed in the
// https://github.com/sdsc-ordes/cat-plus-ontology see here for the expected Synth input data:
// https://github.com/sdsc-ordes/cat-plus-ontology/tree/96091fd2e75e03de8a4c4d66ad502b2db27998bd/json-file/1-Synth
use crate::graph::{
    namespaces::{alloproc, alloqual, allores, cat, obo, purl, qudt, schema},
    utils::generate_bnode_term,
};
use anyhow;
use serde::{Deserialize, Serialize};
use sophia::{
    api::{
        graph::MutableGraph,
        ns::{rdf, xsd},
    },
    inmem::graph::LightGraph,
};
use sophia_api::{
    ns::NsTerm,
    term::{FromTerm, SimpleTerm, Term},
};

fn to_graph_box<T: ToGraph + 'static>(item: T) -> Box<dyn ToGraph> {
    Box::new(item)
}

pub fn link_node<N>(source_uri: SimpleTerm, predicate: SimpleTerm, node: N) -> Vec<[SimpleTerm; 3]>
where
    N: ToGraph,
{
    let node_uri = node.get_uri();
    let mut triples = vec![[source_uri.clone(), predicate.clone(), node_uri.clone()]];
    triples.append(&mut node.to_triples(node_uri.clone()));

    triples
}

/// Convert a struct into an RDF graph.
pub trait ToGraph {
    /// Represent the struct as a collection of triples.
    ///
    /// # Arguments
    /// - `subject`: The URI to use for the struct being converted.
    ///
    /// # Returns
    /// A collection of triples.
    fn to_triples<'a, 'b, 'c>(&'c self, subject: SimpleTerm<'a>) -> Vec<[SimpleTerm<'b>; 3]>
    where
        'c: 'b,
        'a: 'b;

    /// Convert the struct to a graph.
    ///
    /// # Arguments
    /// - `subject`: The URI to use for the struct being converted.
    ///
    /// # Returns
    /// The graph representation of the struct.
    fn to_graph(&self, subject: SimpleTerm) -> anyhow::Result<LightGraph> {
        let mut graph = LightGraph::new();
        let triples = self.to_triples(subject);
        for triple in triples {
            graph.insert(&triple[0], &triple[1], &triple[2])?;
        }
        return Ok(graph);
    }

    /// Get the URI for the struct.
    ///
    /// The default implementation generates a random blank node URI.
    ///
    /// # Returns
    /// The URI for the struct.
    fn get_uri(&self) -> SimpleTerm<'static> {
        generate_bnode_term()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Batch {
    #[serde(rename = "batchID")]
    pub batch_id: String,
    #[serde(rename = "Actions")]
    pub actions: Vec<Action>,
    pub batch_name: Option<String>,
    #[serde(rename = "ReactionType")]
    pub reaction_type: Option<String>,
    #[serde(rename = "OptimizationType")]
    pub optimization_type: Option<String>,
    #[serde(rename = "Link")]
    pub link: Option<String>,
}

impl ToGraph for Batch {
    fn to_triples(&self, subject: SimpleTerm) -> Vec<[SimpleTerm; 3]> {
        let mut triples: Vec<[SimpleTerm; 3]> =
            [(&rdf::type_, &cat::Batch), (&schema::name, &self::batch_id)]
                .into_iter()
                .map(|(predicate, object)| {
                    [subject.clone(), predicate.as_simple(), object.as_simple()]
                })
                .collect();

        for action in &self.actions {
            let action_subject = action.get_uri();
            triples.push([action_subject, cat::hasBatch.as_simple(), subject.clone()]);

            triples.append(&mut action.to_triples(action_subject));
        }

        triples
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

impl ToGraph for Action {
    fn to_triples<'a, 'b, 'c>(&'c self, subject: SimpleTerm<'a>) -> Vec<[SimpleTerm<'b>; 3]>
    where
        'c: 'b,
        'a: 'b,
    {
        let subject: SimpleTerm = generate_bnode_term();

        // Data properties.
        let mut data_properties = vec![
            (&allores::AFX_0000622, &self.start_time),
            (&allores::AFR_0002423, &self.ending_time),
            (&allores::AFR_0001606, &self.method_name),
            (&allores::AFR_0001723, &self.equipment_name),
            (&cat::subEquipmentName, &self.sub_equipment_name),
            (&rdf::type_, &self.action_name.to_string()),
        ];

        // Optional data properties.
        if let Some(dispense_type) = &self.dispense_type {
            data_properties.push((&cat::dispenseType, dispense_type));
        }

        // Object properties
        let mut object_properties: Vec<(NsTerm, Option<Box<dyn ToGraph>>)> = vec![
            (
                cat::temperatureShakerShape,
                self.temperature_shaker.map(to_graph_box),
            ),
            (
                cat::temperatureTumbleStirrerShape,
                self.temperature_tumble_stirrer.map(to_graph_box),
            ),
            (cat::speedInRPM, self.speed_shaker.map(to_graph_box)),
            (
                cat::speedTumbleStirrerShape,
                self.speed_tumble_stirrer.map(to_graph_box),
            ),
            (
                alloproc::AFP_0002677,
                self.pressure_measurement.map(to_graph_box),
            ),
            (cat::hasSample, self.has_sample.map(to_graph_box)),
        ];

        // Multivalued object properties.
        if let Some(container_pos) = &self.has_container_position_and_quantity {
            for container_item in container_pos {
                object_properties.push((
                    cat::hasContainerPositionAndQuantity,
                    Some(to_graph_box(*container_item)),
                ))
            }
        }

        // Generate triples.
        let mut triples: Vec<[SimpleTerm; 3]> = data_properties
            .into_iter()
            .map(|(predicate, object)| [subject.clone(), predicate.as_simple(), object.as_simple()])
            .collect();

        for (pred, object) in object_properties {
            if let Some(obj) = object {
                triples.append(&mut link_node(
                    subject.clone(),
                    pred.as_simple(),
                    obj.into(),
                ));
            }
        }

        // NOTE: for container_info, we attach triples directly to the action
        if let Some(container_info) = &self.container_info {
            triples.append(&mut container_info.to_triples(subject.clone()));
        };

        triples
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case, non_camel_case_types)]
pub enum ActionName {
    AddAction,
    setTemperatureAction,
    filtrateAction,
    shakeAction,
    setVacuumAction,
    setPressureAction,
}

impl fmt::Display for ActionName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let term = match self {
            Self::AddAction => cat::AddAction,
            Self::setTemperatureAction => cat::SetTemperatureAction,
            Self::setPressureAction => cat::SetPressureAction,
            Self::shakeAction => cat::ShakeAction,
            Self::setVacuumAction => cat::SetVacuumAction,
            Self::filtrateAction => cat::FiltrateAction,
        };

        write!(f, "{}", term.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerInfo {
    #[serde(rename = "containerID")]
    pub container_id: String,
    pub container_barcode: String,
}

impl ToGraph for ContainerInfo {
    fn to_triples<'a, 'b, 'c>(&'c self, subject: SimpleTerm<'a>) -> Vec<[SimpleTerm<'b>; 3]>
    where
        'c: 'b,
        'a: 'b,
    {
        let data_properties = [
            (&cat::containerID, &self.container_id.as_simple()),
            (&cat::containerBarcode, &self.container_barcode.as_simple()),
        ];

        let triples = data_properties
            .into_iter()
            .map(|(predicate, object)| [subject.clone(), predicate.as_simple(), object.to_owned()])
            .collect();

        triples
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Observation {
    pub value: f64,
    pub unit: String,
    pub error_margin: Option<ErrorMargin>,
}

impl ToGraph for Observation {
    fn to_triples<'a, 'b, 'c>(&'c self, subject: SimpleTerm<'a>) -> Vec<[SimpleTerm<'b>; 3]>
    where
        'c: 'b,
        'a: 'b,
    {
        let data_properties = [
            (&rdf::type_, &cat::Observation.as_simple()),
            (&qudt::unit, &self.unit.as_simple()),
            (&qudt::value, &self.value.as_simple()),
        ];

        let object_properties = [(cat::errorMargin, &self.error_margin)];

        let mut triples: Vec<[SimpleTerm; 3]> = data_properties
            .into_iter()
            .map(|(predicate, object)| [subject.clone(), predicate.as_simple(), object.as_simple()])
            .collect();

        for (pred, object) in object_properties {
            if let Some(obj) = object {
                triples.append(&mut link_node(subject.clone(), pred.as_simple(), *obj));
            }
        }

        triples
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMargin {
    pub value: f64,
    pub unit: String,
}

impl ToGraph for ErrorMargin {
    fn to_triples<'a, 'b, 'c>(&'c self, subject: SimpleTerm<'a>) -> Vec<[SimpleTerm<'b>; 3]>
    where
        'c: 'b,
        'a: 'b,
    {
        let data_properties = [
            (&qudt::unit, self.unit.as_simple()),
            (&qudt::value, self.value.as_simple()),
        ];

        let triples = data_properties
            .into_iter()
            .map(|(predicate, object)| [subject.clone(), predicate.as_simple(), object])
            .collect();

        triples
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

impl ToGraph for Sample {
    fn to_triples<'a, 'b, 'c>(&'c self, subject: SimpleTerm<'a>) -> Vec<[SimpleTerm<'b>; 3]>
    where
        'c: 'b,
        'a: 'b,
    {
        let data_properties = [
            (&rdf::type_, cat::Sample.as_simple()),
            (&cat::role, self.role.as_simple()),
            (&cat::vialShape, self.vial_type.as_simple()),
            (&allores::AFR_0002464, self.vial_type.as_simple()),
        ];

        let mut object_properties: Vec<(NsTerm, Box<dyn ToGraph>)> =
            vec![(cat::expectedDatum, to_graph_box(self.expected_datum))];

        // Multivalued object properties.
        for item in &self.has_sample {
            object_properties.push((cat::hasSample, to_graph_box(*item)))
        }

        // Generate triples
        let mut triples: Vec<[SimpleTerm; 3]> = data_properties
            .into_iter()
            .map(|(predicate, object)| [subject.clone(), predicate.as_simple(), object])
            .collect();

        for (pred, obj) in object_properties {
            triples.append(&mut link_node(
                subject.clone(),
                pred.as_simple(),
                obj.into(),
            ));
        }

        // NOTE: for container_info, we attach triples directly to the Sample
        triples.append(&mut self.container.to_triples(subject.clone()));

        triples
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

impl ToGraph for SampleItem {
    fn to_triples<'a, 'b, 'c>(&'c self, subject: SimpleTerm<'a>) -> Vec<[SimpleTerm<'b>; 3]>
    where
        'c: 'b,
        'a: 'b,
    {
        let data_properties: Vec<(NsTerm, Option<SimpleTerm>)> = vec![
            (rdf::type_, cat::Sample.as_simple()),
            (purl::identifier, self.sample_id.as_simple()),
            (cat::role, self.role.as_simple()),
            (cat::internalBarCode, self.internal_bar_code.as_simple()),
            (alloqual::AFQ_0000111, self.physical_state.as_simple()),
        ]
        .into_iter()
        .map(|(p, o)| (p, Some(o)))
        .collect();

        let object_properties: Vec<(NsTerm, Option<Box<dyn ToGraph>>)> = vec![
            (cat::expectedDatum, self.expected_datum.map(to_graph_box)),
            (
                cat::measuredQuantity,
                self.measured_quantity.map(to_graph_box),
            ),
            (allores::AFR_0002036, self.concentration.map(to_graph_box)),
            (cat::hasChemical, Some(to_graph_box(self.has_chemical))),
        ];

        let mut triples: Vec<[SimpleTerm<'b>; 3]> = data_properties
            .into_iter()
            .filter(|(_, o)| o.is_some())
            .map(|(p, o)| (p, o.unwrap()))
            .map(|(predicate, object)| [subject.clone(), predicate.as_simple(), object])
            .collect();

        for (pred, object) in object_properties {
            if let Some(obj) = object {
                triples.append(&mut link_node(
                    subject.clone(),
                    pred.as_simple(),
                    obj.into(),
                ));
            }
        }

        triples
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

impl ToGraph for Chemical {
    fn to_triples<'a, 'b, 'c>(&'c self, subject: SimpleTerm<'a>) -> Vec<[SimpleTerm<'b>; 3]>
    where
        'c: 'b,
        'a: 'b,
    {
        let mut data_properties: Vec<(NsTerm, Option<SimpleTerm>)> = vec![
            (rdf::type_, obo::CHEBI_25367.as_simple()),
            (purl::identifier, self.chemical_id.as_simple()),
            (cat::chemicalName, self.chemical_name.as_simple()),
            (allores::AFR_0001952, self.molecular_formula.as_simple()),
            (allores::AFR_0002295, self.smiles.as_simple()),
            (allores::AFR_0002294, self.molecular_mass.value.as_simple()),
            (allores::AFR_0002296, self.inchi.as_simple()),
        ]
        .into_iter()
        .map(|(p, o)| (p, Some(o)))
        .collect();

        // Optional data properties.
        data_properties.append(&mut vec![
            (cat::casNumber, self.cas_number.map(|o| o.as_simple())),
            (
                cat::swissCatNumber,
                self.swiss_cat_number.map(|o| o.as_simple()),
            ),
            (schema::keywords, self.keywords.map(|o| o.as_simple())),
        ]);

        let object_properties: Vec<(NsTerm, Option<Box<dyn ToGraph>>)> =
            vec![(obo::PATO_0001019, self.density.map(to_graph_box))];

        // Generate triples
        let mut triples: Vec<[SimpleTerm<'b>; 3]> = data_properties
            .into_iter()
            .map(|(p, o)| [subject.clone(), p.as_simple(), o.unwrap()])
            .collect();

        for (pred, object) in object_properties {
            if let Some(obj) = object {
                triples.append(&mut link_node(
                    subject.clone(),
                    pred.as_simple(),
                    obj.into(),
                ));
            }
        }

        triples
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerPositionQuantityItem {
    #[serde(rename = "containerID")]
    pub container_id: String,
    pub position: String,
    pub quantity: Observation,
}

impl ToGraph for ContainerPositionQuantityItem {
    fn to_triples<'a, 'b, 'c>(&'c self, subject: SimpleTerm<'a>) -> Vec<[SimpleTerm<'b>; 3]>
    where
        'c: 'b,
        'a: 'b,
    {
        let mut data_properties = vec![
            (&rdf::type_, cat::ContainerPositionAndQuantity.as_simple()),
            (&cat::containerID, self.container_id.as_simple()),
            (&allores::AFR_0002240, self.position.as_simple()),
        ];

        let mut object_project = vec![(qudt::quantity, to_graph_box(self.quantity))];

        // Generate triples
        let mut triples: Vec<[SimpleTerm<'b>; 3]> = data_properties
            .into_iter()
            .map(|(predicate, object)| [subject.clone(), predicate.as_simple(), object])
            .collect();

        for (pred, object) in object_project {
            triples.append(&mut link_node(
                subject.clone(),
                pred.as_simple(),
                object.into(),
            ));
        }

        triples
    }
}
