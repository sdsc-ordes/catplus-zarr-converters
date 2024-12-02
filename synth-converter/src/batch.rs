use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Batch {
    #[serde(rename = "batchID")]
    pub batch_id: String,
    #[serde(rename = "Actions")]
    pub actions: Vec<Action>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    #[serde(rename = "actionName")]
    pub action_name: ActionName,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endingTime")]
    pub ending_time: String,
    #[serde(rename = "methodName")]
    pub method_name: String,
    #[serde(rename = "equipmentName")]
    pub equipment_name: String,
    #[serde(rename = "subEquipmentName")]
    pub sub_equipment_name: String,
    #[serde(flatten)]
    pub container_info: Option<ContainerInfo>,
    #[serde(rename = "speedShaker")]
    pub speed_shaker: Option<Observation>,
    #[serde(flatten)]
    #[serde(rename = "hasContainerPositionAndQuantity")]
    pub has_container_position_and_quantity: Option<Vec<ContainerPosition>>,
    #[serde(rename = "dispenseState")]
    pub dispense_state: Option<String>,
    #[serde(rename = "dispenseType")]
    pub dispense_type: Option<String>,
    #[serde(rename = "hasSample")]
    pub has_sample: Option<Sample>,
    #[serde(rename = "speedTumbleStirrer")]
    pub speed_tumble_stirrer: Option<Observation>,
    #[serde(rename = "temperatureTumbleStirrer")]
    pub temperature_tumble_stirrer: Option<Observation>,
    #[serde(rename = "temperatureShaker")]
    pub temperature_shaker: Option<Observation>,
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
pub struct ContainerInfo {
    #[serde(rename = "containerID")]
    pub container_id: String,
    #[serde(rename = "containerBarcode")]
    pub container_barcode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Observation {
    pub value: f64,
    pub unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sample {
    #[serde(flatten)]
    pub container: ContainerInfo,
    #[serde(rename = "vialID")]
    pub vial_id: String,
    #[serde(rename = "vialType")]
    pub vial_type: String,
    pub role: String,
    #[serde(rename = "expectedDatum")]
    pub expected_datum: Observation,
    #[serde(rename = "hasSample")]
    pub has_sample: Vec<SampleItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SampleItem {
    #[serde(rename = "sampleID")]
    pub sample_id: String,
    pub role: String,
    #[serde(rename = "internalBarCode")]
    pub internal_bar_code: String,
    #[serde(rename = "expectedDatum")]
    pub expected_datum: Option<Observation>,
    #[serde(rename = "physicalState")]
    pub physical_state: String,
    #[serde(rename = "hasChemical")]
    pub has_chemical: Chemical,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chemical {
    #[serde(rename = "chemicalID")]
    pub chemical_id: String,
    #[serde(rename = "chemicalName")]
    pub chemical_name: String,
    #[serde(rename = "CASNumber")]
    pub cas_number: String,
    #[serde(rename = "molecularMass")]
    pub molecular_mass: Observation,
    pub smiles: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerPosition {
    pub position: String,
    pub quantity: Observation,
}
