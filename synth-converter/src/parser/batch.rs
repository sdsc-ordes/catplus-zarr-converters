use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Batch {
    pub batchID: String,
    pub actions: Vec<Action>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case, non_camel_case_types)]
#[serde(tag = "action_name")]
pub enum Action {
    filtrateAction(FiltrateAction),
    setTemperatureAction(SetTemperatureAction),
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct BaseAction {
    pub startTime: String,
    pub method_name: String,
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
    pub TemperatureTumbleStirrer: Measurement,
    pub TemperatureShaker: Measurement,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measurement {
    value: f64,
    unit: String,
}
