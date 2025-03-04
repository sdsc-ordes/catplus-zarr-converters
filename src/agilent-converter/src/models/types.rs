#[rustfmt::skip]
// The structure follows the input data as descibed in the
// https://github.com/sdsc-ordes/cat-plus-ontology
use catplus_common::{
    graph::{
        insert_into::{InsertIntoGraph, Link},
        namespaces::{alloproc, alloqual, allores, cat, obo, purl, qudt, schema},
    },
    models::enums::{ActionName, Unit},
};
use anyhow;
use serde::{Deserialize, Serialize};
use sophia::{
    api::ns::{rdf, xsd},
    inmem::graph::LightGraph,
};
use sophia_api::{
    graph::MutableGraph,
    term::{SimpleTerm, Term},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sample;
impl InsertIntoGraph for Sample {};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Peak;
impl InsertIntoGraph for Peak {};
