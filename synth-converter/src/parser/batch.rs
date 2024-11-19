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
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct FiltrateAction {
    pub startTime: String,
    pub method_name: String,
    pub equipment_name: String,
    pub sub_equipment_name: String,
    pub containerID: String,
    pub containerBarcode: String,
}
