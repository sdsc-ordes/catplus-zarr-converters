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


// if Optional: .as_ref().clone().map(|s| s.as_simple())
// else: .as_simple()

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LiquidChromatographyDocument {
    pub analyst: String,
    #[serde(rename = "AFR_0002375")]
    pub measurement_document: MeasurementDocument,
    #[serde(rename = "DeviceSystemDocument")]
    pub device_system_document: DeviceSystemDocument,
}

impl InsertIntoGraph for LiquidChromatographyDocument {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &allores::AFR_0002525.as_simple() as &dyn InsertIntoGraph),
            (allores::AFR_0001116, &self.analyst.as_simple()),
            (allores::AFR_0002374, &self.measurement_document),
            (allores::AFR_0002526, &self.device_system_document),
            ,
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MeasurementDocument {
    pub measurement_identifier: String,
    #[serde(rename = "ChromatographyColumnDocument")]
    pub chromatography_column_document: Optional<String>,
    #[serde(rename = "AFR_0002567")]
    pub device_control_document: DeviceDocument,
    pub sample_document: SampleDocument,
    pub injection_document: InjectionDocument,
    pub detection_type: String,
    pub chromatogram_data_cube: ChromatogramDataCube,
    pub three_dimensional_ultraviolet_spectrum_data_cube: ThreeDimensionalUltravioletSpectrumDataCube,
    pub three_three_dimensional_mass_spectrum_data_cube: ThreeDimensionalMassSpectrumDataCube,
    pub processed_data_document: ProcessedDataDocument,
}

impl InsertIntoGraph for MeasurementDocument {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &allores::AFR_0002374.as_simple() as &dyn InsertIntoGraph),
            (allores::AFR_0001121, &self.measurement_identifier.iri().as_simple()),
            (allores::AFR_0002607, &self.chromatography_column_document.as_ref().clone().map(|s| s.as_simple())),
            (allores::AFR_0002722, &self.device_document),
            (allores::AFR_0002083, &self.sample_document),
            (allores::AFR_0002529, &self.injection_document),
            (allores::AFR_0002534, &self.detection_type.as_simple()),
            (allores::AFR_0002550, &self.chromatogram_data_cube),
            (allores::AFR_0002551, &self.three_dimensional_ultraviolet_spectrum_data_cube),
            (allores::AFR_0002878, &self.three_three_dimensional_mass_spectrum_data_cube),
            (allores::AFR_0002659, &self.processed_data_document),
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceSystemDocument{
    #[serde(rename = "")]
    pub device_document: Vec<DeviceDocument>,
    pub asset_management_identifier: Optional<String>,
}

impl InsertIntoGraph for DeviceSystemDocument {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &allores::AFR_0002526.as_simple() as &dyn InsertIntoGraph),
            (allores::AFR_0002722, &self.device_document),
            (allores::AFR_0001976, &self.chromatography_column_document.as_ref().clone().map(|s| s.as_simple())),
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceDocument{
    pub device_identifier: String,
    pub device_type: String,
    pub product_manufacturer: String,
    pub equipment_serial_number: String,
    pub model_number: String,
    pub firmware_version: String,
    pub detection_type: String,
    pub index: Optional<Integer>,
}

impl InsertIntoGraph for DeviceDocument {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &allores::AFR_0002722.as_simple() as &dyn InsertIntoGraph),
            (allores::AFR_0002018, &self.device_identifier.iri().as_simple()),
            (allores::AFR_0002568, &self.device_type.as_simple()),
            (allores::AFR_0001258, &self.product_manufacturer.as_simple()),
            (allores::AFR_0001119, &self.equipment_serial_number.as_simple()),
            (obo::IAO_0000017, &self.model_number.as_simple()),
            (allores::AFR_0001259, &self.firmware_version.as_simple()),
            (allores::AFR_0002534, &self.detection_type.as_simple()),
            ( "http://purl.allotrope.org/ontologies/datacube-hdf-map#Index", &self.index.as_ref().clone().map(|s| s.as_simple()))
        ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessedDataDocument{
    #[serde(rename = "PeakList")]
    pub peak_list: PeakList,
} 

impl InsertIntoGraph for ProcessedDataDocument {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &allores::AFR_0002659.as_simple() as &dyn InsertIntoGraph),
            (allores::AFR_0000432, &self.peak_list),
            ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SampleDocument {
    pub sample_identifier: String,
    pub written_name: String,
}

impl InsertIntoGraph for SampleDocument {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &allores::AFR_0002083.as_simple() as &dyn InsertIntoGraph),
            (allores::AFR_0001118, &self.sample_identifier.iri().as_simple()),
            (obo::IAO_0000590, &self.written_name.as_simple())
            ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InjectionDocument {
    #[serde(rename = "AutosamplerInjectionVolumeSetting")]
    pub autosampler_injection: AutosamplerInjectionVolumeSetting,
    pub injection_identifier: String,
    pub injection_time: dateTime,
}

impl InsertIntoGraph for InjectionDocument {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::InjectionDocument.as_simple() as &dyn InsertIntoGraph),
            (allores::AFR_0001267, &self.autosampler_injection),
            (allores::AFR_0002535, &self.injection_identifier.iri().as_simple()),
            (allores::AFR_0002536, &(self.injection_time.as_str() * xsd::dateTime).as_simple())
            ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChromatogramDataCube {
    pub label: String,
    #[serde(rename = "CubeStructure")] 
    pub cube_structure: CubeStructure,
    #[serde(rename = "Dataframe")]
    pub data: Dataframe,
    pub identifier: String
}

impl InsertIntoGraph for ChromatogramDataCube {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::ChromatogramDataCube.as_simple() as &dyn InsertIntoGraph),
            (obo::IAO_0000009, &self.label.as_simple()),
            (qb::DataSet, &self.cube_structure),
            (allohdf::Dataset, &self.data),
            (allores::AFR_0000917, &self.identifier.iri().as_simple())
            ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThreeDimensionalUltravioletSpectrumDataCube {
    pub label: String,
    #[serde(rename = "CubeStructure")] 
    pub cube-structure: CubeStructure,
    #[serde(rename = "Dataframe")] 
    pub data: Dataframe,
    #[serde(rename = "AFR_0000917")]
    pub identifier: String
}

impl InsertIntoGraph for ThreeDimensionalUltravioletSpectrumDataCube {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::ThreeDimensionalUltravioletSpectrumDataCube.as_simple() as &dyn InsertIntoGraph),
            (obo::IAO_0000009, &self.label.as_simple()),
            (qb::DataSet, &self.cube_structure),
            (allohdf::Dataset, &self.data),
            (allores::AFR_0000917, &self.identifier.iri().as_simple())
            ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThreeDimensionalMassSpectrumDataCube {
    pub label: String,
    #[serde(rename = "CubeStructure")] 
    pub cube-structure: CubeStructure,
    #[serde(rename = "Dataframe")] 
    pub data: Dataframe,
    pub identifier: String,
}

impl InsertIntoGraph for ThreeDimensionalMassSpectrumDataCube {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::ThreeDimensionalMassSpectrumDataCube.as_simple() as &dyn InsertIntoGraph),
            (obo::IAO_0000009, &self.label.as_simple()),
            (qb::DataSet, &self.cube_structure),
            (allohdf::Dataset, &self.data),
            (allores::AFR_0000917, &self.identifier.iri().as_simple())
            ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AutosamplerInjectionVolumeSetting {
    pub value: f64,
    pub unit: Unit,
}

impl InsertIntoGraph for AutosamplerInjectionVolumeSetting {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::AutosamplerInjectionVolumeSetting.as_simple() as &dyn InsertIntoGraph),
            (qudt::value, &self.value.as_simple()),
            (qudt::unit, &self.unit.iri().as_simple())
            ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CubeStructure {
    #[serde(rename = "AFRL_0000157")]
    pub measures: Measure,
    #[serde(rename = "Dimension")]
    pub dimensions: Dimension,
}

impl InsertIntoGraph for CubeStructure {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::CubeStructure.as_simple() as &dyn InsertIntoGraph),
            (cat::measure, &self.measures),
            (cat::dimension, &self.dimensions)
            ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Measure {
    pub component_data_type : String, //subject to change 
    pub concept: String,
    pub unit: Unit,
}

impl InsertIntoGraph for Measure {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &allorole::AFRL_0000157.as_simple() as &dyn InsertIntoGraph),
            (allodc::componentDataType,  &self.component_data_type.as_simple()),
            (rdfs:label, &self.concept.as_simple()),
            (qudt::unit, &self.unit.iri().as_simple())
            ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dimension {
    pub component_data_type : String, //subject to change 
    pub concept: String,
    pub unit: Unit,
}

impl InsertIntoGraph for Dimension {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::Dimension.as_simple() as &dyn InsertIntoGraph),
            (allodc::componentDataType,  &self.component_data_type.as_simple()),
            (rdfs:label, &self.concept.as_simple()),
            (qudt::unit, &self.unit.iri().as_simple())
            ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dataframe {
    #[serde(rename = "AFRL_0000157")]
    pub measures: Measure,
    #[serde(rename = "Dimension")]
    pub dimensions: Dimension,
}

impl InsertIntoGraph for Dataframe {
    fn insert_into(&self, graph: &mut LightGraph, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::Dataframe.as_simple() as &dyn InsertIntoGraph),
            (cat::measure, &self.measures),
            (cat::dimension, &self.dimensions)
            ] {
            value.attach_into(
                graph,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}
