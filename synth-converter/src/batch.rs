// The structure follows the input data as descibed in the
// https://github.com/sdsc-ordes/cat-plus-ontology see here for the expected Synth input data:
// https://github.com/sdsc-ordes/cat-plus-ontology/tree/96091fd2e75e03de8a4c4d66ad502b2db27998bd/json-file/1-Synth
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerInfo {
    #[serde(rename = "containerID")]
    pub container_id: String,
    pub container_barcode: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Observation {
    pub value: f64,
    pub unit: String,
    pub error_margin: Option<ErrorMargin>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMargin {
    pub value: f64,
    pub unit: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerPositionQuantityItem {
    #[serde(rename = "containerID")]
    pub container_id: String,
    pub position: String,
    pub quantity: Observation,
}
