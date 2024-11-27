use crate::graph::namespaces::EX;
use sophia_api::{
    prelude::*,
    term::{bnode_id::BnodeId, SimpleTerm},
};
use uuid::Uuid;

pub fn generate_bnode_term() -> SimpleTerm<'static> {
    let identifier = Uuid::new_v4().to_string();
    let bnode = BnodeId::new_unchecked(identifier);
    bnode
        .try_into_term()
        .expect("Failed to convert BnodeId to SimpleTerm")
}

pub fn generate_uri_term() -> Result<SimpleTerm<'static>, Box<dyn std::error::Error>> {
    let identifier = Uuid::new_v4().to_string();
    let node_uri = EX.get(&identifier)?;
    node_uri
        .try_into_term()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>) // Convert error type
}
