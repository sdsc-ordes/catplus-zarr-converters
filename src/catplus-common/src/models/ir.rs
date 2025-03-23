use crate::{
    graph::{
        insert_into::{InsertIntoGraph, Link},
        namespaces::{alloproc, allocom, allohdf, alloqual, allores, cat, obo, purl, qudt, schema},
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "OPUSDataFile")]
pub struct OpusDataFile {
    #[serde(rename = "Data_Parameters_TR", default)]
    data_parameters_tr: Option<Parameters>,
    
    #[serde(rename = "TR__Peak", default)]
    tr_peak: Option<PeakReport>,
    
    #[serde(rename = "Data_Parameters_ScSm", default)]
    data_parameters_scsm: Option<Parameters>,
    
    #[serde(rename = "ScSm__Peak", default)]
    scsm_peak: Option<PeakReport>,
    
    #[serde(rename = "Data_Parameters_ScRf", default)]
    data_parameters_scrf: Option<Parameters>,
    
    #[serde(rename = "ScRf__Peak", default)]
    scrf_peak: Option<PeakReport>
}

#[derive(Debug, Deserialize, Serialize)]
struct Parameters {
    #[serde(rename = "parameter", default)]
    parameters: Vec<Parameter>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Parameter {
    #[serde(rename = "name", default)]
    name: String,
    #[serde(rename = "$text", default)]
    value: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "MainReport")]
struct PeakReport {
    #[serde(rename = "Header", default)]
    header: Option<Header>,
    #[serde(rename = "Matrix", default)]
    matrix: Option<Matrix>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Header {
    #[serde(rename = "Line", default)]
    lines: Vec<Line>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Line {
    #[serde(rename = "Legend", default)]
    legend: Option<String>,
    #[serde(rename = "Value", default)]
    value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Matrix {
    #[serde(rename = "Line", default)]
    lines: Vec<MatrixLine>,
}

#[derive(Debug, Deserialize, Serialize)]
struct MatrixLine {
    #[serde(rename = "Value", default)]
    values: Vec<String>,
}