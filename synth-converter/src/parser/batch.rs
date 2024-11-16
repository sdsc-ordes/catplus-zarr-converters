use serde::{Deserialize, Serialize};
use std::clone::Clone;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Batch {
    #[serde(rename = "batchID")]
    pub batch_id: String,
    pub actions: Vec<Action>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Action {
    pub name: String,
    pub speed: Option<Measurement>,
    #[serde(rename = "TemperatureTumbleStirrer")]
    pub temperature_tumble_stirrer: Option<Measurement>,
    #[serde(rename = "TemperatureShaker")]
    pub temperature_shaker: Option<Measurement>,
    #[serde(rename = "RotationTumbleStirrer")]
    pub rotation_tumble_stirrer: Option<Measurement>,
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    pub endingtime: Option<String>,
    pub method: Option<String>,
    pub equipment: Option<String>,
    pub equipment_local_name: Option<String>,
    pub container: Option<String>,
    #[serde(rename = "containerBarcode")]
    pub container_barcode: Option<String>,
    pub vacuum: Option<Measurement>,
    pub pressure: Option<Measurement>,
    #[serde(rename = "dispenseState")]
    pub dispense_state: Option<String>,
    #[serde(rename = "dispenseType")]
    pub dispense_type: Option<String>,
    #[serde(rename = "containersPositionsAndQuantities")]
    pub containers_positions_and_quantities: Option<Vec<ContainerPosition>>,
    pub sample: Option<Sample>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Measurement {
    pub value: serde_json::Value,
    pub unit: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContainerPosition {
    pub position: String,
    pub quantity: Measurement,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sample {
    #[serde(rename = "expectedDatum")]
    pub expected_datum: Measurement,
    pub unit: Option<String>,
    pub container: String,
    #[serde(rename = "containerBarcode")]
    pub container_barcode: String,
    #[serde(rename = "vialId")]
    pub vial_id: String,
    #[serde(rename = "vialType")]
    pub vial_type: String,
    pub role: String,
    pub sample: Vec<SampleDetail>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SampleDetail {
    #[serde(rename = "ID")]
    pub id: String,
    pub role: String,
    #[serde(rename = "internalBarCode")]
    pub internal_bar_code: String,
    #[serde(rename = "expectedDatum")]
    pub expected_datum: Option<Measurement>,
    #[serde(rename = "measuredQuantity")]
    pub measured_quantity: Measurement,
    #[serde(rename = "physicalState")]
    pub physical_state: String,
    pub concentration: Option<Measurement>,
    pub purity: Option<String>,
    pub chemical: Chemical,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chemical {
    #[serde(rename = "ID")]
    pub id: String,
    pub name: String,
    #[serde(rename = "CASNumber")]
    pub cas_umber: String,
    #[serde(rename = "molecularMass")]
    pub molecular_mass: String,
    pub smiles: String,
}
