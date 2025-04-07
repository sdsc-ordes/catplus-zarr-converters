use sophia_api::{
    prelude::*,
    term::{bnode_id::BnodeId, SimpleTerm},
};
use uuid::Uuid;

pub const EX_BASE: &str = "http://example.org/";

pub fn generate_bnode_term() -> SimpleTerm<'static> {
    let identifier = Uuid::new_v4().to_string();
    let bnode = BnodeId::new_unchecked(identifier);

    bnode.try_into_term().expect("Failed to convert BnodeId to SimpleTerm")
}

pub fn generate_iri_term() -> SimpleTerm<'static> {
    let identifier = Uuid::new_v4().to_string();
    let iri_string = format!("{}{}", EX_BASE, identifier);
    let iri = IriRef::new_unchecked(iri_string);

    iri.try_into_term().expect("Failed to convert Iri to SimpleTerm")
}
