use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Batch {
    pub batchID: String,
    #[serde(rename = "action")]
    pub actions: Vec<Action>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case, non_camel_case_types)]
#[serde(tag = "action_name")]
pub enum Action {
    filtrateAction(FiltrateAction),
    setTemperatureAction(SetTemperatureAction),
    shakeAction(ShakeAction),
    setVacuumAction(SetVacuumAction),
    setPressureAction(SetPressureAction),
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct BaseAction {
    pub startTime: String,
    pub method_name: Option<String>,
    pub equipment_name: String,
    pub sub_equipment_name: String,
    pub containerID: String,
    pub containerBarcode: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct FiltrateAction {
    #[serde(flatten)]
    pub base: BaseAction,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct SetTemperatureAction {
    #[serde(flatten)]
    pub base: BaseAction,
    pub speed: Option<Measurement>,
    pub TemperatureTumbleStirrer: Measurement,
    pub TemperatureShaker: Measurement,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ShakeAction {
    #[serde(flatten)]
    pub base: BaseAction,
    pub RotationTumbleStirrer: Measurement,
    pub TemperatureTumbleStirrer: Measurement,
    pub TemperatureShaker: Measurement,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct SetVacuumAction {
    #[serde(flatten)]
    pub base: BaseAction,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct SetPressureAction {
    #[serde(flatten)]
    pub base: BaseAction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurement {
    pub value: f64,
    pub unit: String,
}
