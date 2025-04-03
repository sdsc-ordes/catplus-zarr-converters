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
    data_parameters_tr: Option<DataParameters>,
    
    #[serde(rename = "TR__Peak", default)]
    tr_peak: Option<MainReport>,
    
    #[serde(rename = "Data_Parameters_ScSm", default)]
    data_parameters_scsm: Option<DataParameters>,
    
    #[serde(rename = "ScSm__Peak", default)]
    scsm_peak: Option<MainReport>,
    
    #[serde(rename = "Data_Parameters_ScRf", default)]
    data_parameters_scrf: Option<DataParameters>,
    
    #[serde(rename = "ScRf__Peak", default)]
    scrf_peak: Option<MainReport>,
    
    #[serde(rename = "Acquisition_Parameters_Rf", default)]
    acquisition_parameters_rf: Option<AcquisitionParameters>,
    
    #[serde(rename = "FT_-_Parameters_Rf", default)]
    ft_parameters_rf: Option<FtParameters>,
    
    #[serde(rename = "Optic_Parameters_Rf", default)]
    optic_parameters_rf: Option<OpticParameters>,
    
    #[serde(rename = "Optic_Parameters_", default)]
    optic_parameters: Option<OpticParameters>,
    
    #[serde(rename = "FT_-_Parameters_", default)]
    ft_parameters: Option<FtParameters>,
    
    #[serde(rename = "Acquisition_Parameters_", default)]
    acquisition_parameters: Option<AcquisitionParameters>,
    
    #[serde(rename = "Instrument_Parameters_Rf", default)]
    instrument_parameters_rf: Option<InstrumentParameters>,
    
    #[serde(rename = "Instrument_Parameters_", default)]
    instrument_parameters: Option<InstrumentParameters>,
    
    #[serde(rename = "Sample_Parameters_", default)]
    sample_parameters: Option<SampleParameters>,
}

#[derive(Debug, Deserialize, Serialize)]
struct DataParameters {
    #[serde(rename = "DPF", default)]
    dpf: Option<u32>,
    #[serde(rename = "NPT", default)]
    npt: Option<u32>,
    #[serde(rename = "FXV", default)]
    fxv: Option<f64>,
    #[serde(rename = "LXV", default)]
    lxv: Option<f64>,
    #[serde(rename = "CSF", default)]
    csf: Option<u32>,
    #[serde(rename = "MXY", default)]
    mxy: Option<f64>,
    #[serde(rename = "MNY", default)]
    mny: Option<f64>,
    #[serde(rename = "DAT", default)]
    dat: Option<String>,
    #[serde(rename = "TIM", default)]
    tim: Option<String>,
    #[serde(rename = "DXU", default)]
    dxu: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct MainReport {
    #[serde(rename = "MainReport", default)]
    name: Option<String>,
    #[serde(rename = "Header", default)]
    header: Option<Header>,
    #[serde(rename = "Matrix", default)]
    matrix: Option<Matrix>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Header {
    #[serde(rename = "Line", default)]
    lines: Vec<HeaderLine>,
}

#[derive(Debug, Deserialize, Serialize)]
struct HeaderLine {
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

#[derive(Debug, Deserialize, Serialize)]
struct AcquisitionParameters {
    #[serde(rename = "PLF", default)]
    plf: Option<String>,
    #[serde(rename = "RES", default)]
    res: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
struct FtParameters {
    #[serde(rename = "APF", default)]
    apf: Option<String>,
    #[serde(rename = "HFQ", default)]
    hfq: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
struct OpticParameters {
    #[serde(rename = "APT", default)]
    apt: Option<String>,
    #[serde(rename = "BMS", default)]
    bms: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct InstrumentParameters {
    #[serde(rename = "HFL", default)]
    hfl: Option<f64>,
    #[serde(rename = "LFL", default)]
    lfl: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SampleParameters {
    #[serde(rename = "CNM", default)]
    cnm: Option<String>,
    #[serde(rename = "SNM", default)]
    snm: Option<String>,
}