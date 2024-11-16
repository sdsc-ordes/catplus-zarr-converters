use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}
