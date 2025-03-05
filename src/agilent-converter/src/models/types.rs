use serde::{Deserialize, Serialize};
use sophia::{
    api::ns::{rdf, xsd},
    inmem::graph::LightGraph,
};
use sophia_api::{
    graph::MutableGraph,
    term::{SimpleTerm, Term},
};

use catplus_common::models::types::{PeakList};
use catplus_common::models::enums::{Unit};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename = "AFR_0002525")]
pub struct LiquidChromatographyDocument {
    #[serde(rename = "AFR_0001116")]
    pub analyst: String,
    #[serde(rename = "AFR_0002374")]
    pub measurement_document: MeasurementDocument,
    #[serde(rename = "AFR_0002526")]
    pub device_system_document: DeviceSystemDocument,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename = "AFR_0002375")]
pub struct MeasurementDocument {
    #[serde(rename = "AFR_0001121")]
    pub measurement_identifier: String,
    #[serde(rename = "AFR_0002607")]
    pub chromatography_column_document: ChromatographyColumnDocument,
    #[serde(rename = "AFR_0002722")]
    pub device_control_document: DeviceDocument,
    #[serde(rename = "AFR_0002083")]
    pub sample_document: SampleDocument,
    #[serde(rename = "AFR_0002529")]
    pub injection_document: InjectionDocument,
    #[serde(rename = "AFR_0002534")]
    pub detection_type: String,
    #[serde(rename = "AFR_0002550")]
    pub chromatogram_data_cube: ChromatogramDataCube,
    #[serde(rename = "AFR_0002551")]
    pub three_dimensional_ultraviolet_spectrum_data_cube: ThreeDimensionalUltravioletSpectrumDataCube,
    #[serde(rename = "AFR_0002878")]
    pub three_three_dimensional_mass_spectrum_data_cube: ThreeDimensionalMassSpectrumDataCube,
    #[serde(rename = "hasProcessedDataDocument")]
    pub processed_data_document: ProcessedDataDocument,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceSystemDocument{
    #[serde(rename = "AFR_0002722")]
    pub device_document: List[DeviceDocument],
    #[serde(rename ="AFR_0001976")]
    pub asset_management_identifier: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename = "AFR_0002567")]
pub struct DeviceDocument{
    #[serde(rename = "AFR_0002018")]
    pub device_identifier: String,
    #[serde(rename = "AFR_0002568")]
    pub device_type: String,
    #[serde(rename = "AFR_0001258")]
    pub product_manufacturer: String,
    #[serde(rename = "AFR_0001119")]
    pub equipment_serial_number: String,
    #[serde(rename = "IAO_0000017")] //obo
    pub model_number: String,
    #[serde(rename = "AFR_0001259")]
    pub firmware_version: String,
    #[serde(rename = "AFR_0002534")]
    pub detection_type: String,
    #[serde(rename = "http://purl.allotrope.org/ontologies/datacube-hdf-map#Index")]
    pub index: Integer,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessedDataDocument{
    #[serde(rename = "AFR_0000432")]
    pub peak_list: PeakList,
} 

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SampleDocument {
    #[serde(rename = "AFR_0001118")] //allo-res
    pub sample_identifier: String,
    #[serde(rename = "IAO_0000590")] //obo
    pub written_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InjectionDocument {
    #[serde(rename = "AFR_0001267")]
    pub autosampler_injection: AutosamplerInjectionVolumeSetting,
    #[serde(rename = "AFR_0002535")]
    pub injection_identifier: String,
    #[serde(rename = "AFR_0002536")]
    pub injection_time: dateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChromatogramDataCube {
    #[serde(rename = "IAO_0000009")] //obo
    pub label: String,
    #[serde(rename = "DataSet")] //qb
    pub cube_structure: CubeStructure,
    #[serde(rename = "Dataset")] //allo-hdf
    pub data: Dataframe,
    #[serde(rename = "AFR_0000917")] //allo-hdf
    pub identifier: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThreeDimensionalUltravioletSpectrumDataCube {
    #[serde(rename = "IAO_0000009")] //obo
    pub label: String,
    #[serde(rename = "DataSet")] //qb
    pub cube-structure: CubeStructure,
    #[serde(rename = "Dataset")] //allo-hdf
    pub data: Dataframe,
    #[serde(rename = "AFR_0000917")] //allo-hdf
    pub identifier: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThreeDimensionalMassSpectrumDataCube {
    #[serde(rename = "IAO_0000009")] //obo
    pub label: String,
    #[serde(rename = "DataSet")] //qb
    pub cube-structure: CubeStructure,
    #[serde(rename = "Dataset")] //allo-hdf
    pub data: Dataframe,
    #[serde(rename = "AFR_0000917")] //allo-hdf
    pub identifier: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AutosamplerInjectionVolumeSetting {
    pub value: f64,
    pub unit: Unit,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CubeStructure {
    #[serde(rename = "measure")]
    pub measures: Measure,
    #[serde(rename = "dimension")]
    pub dimensions: Dimension,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename = "AFRL_0000157")]
pub struct Measure {
    #[serde(rename = "componentDataType")]
    pub component_data_type : X, // Waiting check from Robin
    #[serde(rename = "label")]
    pub concept: String,
    pub unit: Unit,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dimension {
    #[serde(rename = "componentDataType")]
    pub component_data_type : X, // Waiting check from Robin
    #[serde(rename = "label")]
    pub concept: String,
    pub unit: Unit,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dataframe {
    #[serde(rename = "measure")]
    pub measures: Measure,
    #[serde(rename = "dimension")]
    pub dimensions: Dimension,
}
