use serde::{Deserialize, Serialize};
use std::clone::Clone;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub phones: Vec<String>,
}
