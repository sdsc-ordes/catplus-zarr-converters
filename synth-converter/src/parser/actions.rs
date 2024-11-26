use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Batch {
    pub batchID: String,
    pub Actions: Vec<Action>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Action {
    pub actionName: ActionName,
    pub startTime: String,
    pub endingTime: String,
    pub methodName: String,
    pub equipmentName: String,
    pub subEquipmentName: String,
    #[serde(flatten)]
    pub containerInfo: Option<ContainerInfo>,
    pub speedShaker: Option<Observation>,
    #[serde(flatten)]
    pub hasContainerPositionAndQuantity: Option<Vec<ContainerPosition>>,
    pub dispenseState: Option<String>,
    pub dispenseType: Option<String>,
    pub hasSample: Option<Sample>,
    pub speedTumbleStirrer: Option<Observation>,
    pub temperatureTumbleStirrer: Option<Observation>,
    pub temperatureShaker: Option<Observation>,
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
#[allow(non_snake_case)]
pub struct ContainerInfo {
    pub containerID: String,
    pub containerBarcode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Observation {
    pub value: f64,
    pub unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Sample {
    #[serde(flatten)]
    pub container: ContainerInfo,
    pub vialID: String,
    pub vialType: String,
    pub role: String,
    pub expectedDatum: Observation,
    pub hasSample: Vec<SampleItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct SampleItem {
    pub sampleID: String,
    pub role: String,
    pub internalBarCode: String,
    pub expectedDatum: Option<Observation>,
    pub physicalState: String,
    pub hasChemical: Chemical,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Chemical {
    pub chemicalID: String,
    pub chemicalName: String,
    pub CASNumber: String,
    pub molecularMass: Observation,
    pub smiles: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerPosition {
    pub position: String,
    pub quantity: Observation,
}
