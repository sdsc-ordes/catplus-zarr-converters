use crate::{
    graph::{
        graph_builder::GraphBuilder,
        insert_into::{InsertIntoGraph, Link},
        namespaces::{allodc, allores, allorole, cat, obo, qb, qudt},
    },
    models::{core::PeakList, enums::Unit},
};

use serde::{Deserialize, Serialize};
use sophia::api::ns::{rdf, rdfs, xsd};
use sophia_api::term::{SimpleTerm, Term};

#[derive(Deserialize)]
pub struct LiquidChromatographyAggregateDocumentWrapper {
    #[serde(rename = "liquid chromatography aggregate document")]
    pub liquid_chromatography_aggregate_document: LiquidChromatographyAggregateDocument,
}

impl InsertIntoGraph for LiquidChromatographyAggregateDocumentWrapper {
    fn insert_into(&self, builder: &mut GraphBuilder, iri: SimpleTerm) -> anyhow::Result<()> {
        self.liquid_chromatography_aggregate_document.insert_into(builder, iri)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LiquidChromatographyAggregateDocument {
    #[serde(rename = "liquid chromatography document")]
    pub liquid_chromatography_document: Option<Vec<LiquidChromatographyDocument>>,
    #[serde(rename = "device system document")]
    pub device_system_document: Option<DeviceSystemDocument>,
}

impl InsertIntoGraph for LiquidChromatographyAggregateDocument {
    fn insert_into(&self, builder: &mut GraphBuilder, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &allores::AFR_0002524.as_simple() as &dyn InsertIntoGraph),
            (cat::hasLiquidChromatography, &self.liquid_chromatography_document),
            (allores::AFR_0002526, &self.device_system_document),
        ] {
            value.attach_into(
                builder,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LiquidChromatographyDocument {
    pub analyst: String,
    #[serde(rename = "measurement aggregate document")]
    pub measurement_aggregate_document: MeasurementAggregateDocument,
}

impl InsertIntoGraph for LiquidChromatographyDocument {
    fn insert_into(&self, builder: &mut GraphBuilder, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &allores::AFR_0002525.as_simple() as &dyn InsertIntoGraph),
            (allores::AFR_0001116, &self.analyst.as_simple()),
        ] {
            value.attach_into(
                builder,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        // NOTE: measurement_aggregate_document is not materliazed in the ontology -> we will attach measurement_document directly to LiquidChromatigraphyDocument
        let _ = &self.measurement_aggregate_document.insert_into(builder, iri)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MeasurementAggregateDocument {
    #[serde(rename = "measurement document")]
    pub measurement_documents: Vec<MeasurementDocument>,
}

impl InsertIntoGraph for MeasurementAggregateDocument {
    fn insert_into(&self, builder: &mut GraphBuilder, iri: SimpleTerm) -> anyhow::Result<()> {
        // NOTE: measurement_aggregate_document is not materliazed in the ontology -> we will attach measurement_document directly to LiquidChromatigraphyDocument
        let _ = &self.measurement_documents.attach_into(
            builder,
            Link {
                source_iri: iri.clone(),
                pred: allores::AFR_0002374.as_simple(),
                target_iri: None,
            },
        )?;
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MeasurementDocument {
    #[serde(rename = "measurement identifier")]
    pub measurement_identifier: String,
    // TO-DO: needs further definition to be integrated
    // #[serde(rename = "chromatography column document")]
    // pub chromatography_column_document: ChromatographyColumnDocument,
    #[serde(rename = "device control aggregate document")]
    pub device_control_aggregate_document: DeviceSystemDocument,
    #[serde(rename = "sample document")]
    pub sample_document: SampleDocument,
    #[serde(rename = "injection document")]
    pub injection_document: InjectionDocument,
    #[serde(rename = "detection type")]
    pub detection_type: String,
    #[serde(rename = "chromatogram data cube")]
    pub chromatogram_data_cube: Option<ChromatogramDataCube>,
    #[serde(rename = "three-dimensional ultraviolet spectrum data cube")]
    pub three_dimensional_ultraviolet_spectrum_data_cube:
        Option<ThreeDimensionalUltravioletSpectrumDataCube>,
    #[serde(rename = "three-dimensional mass spectrum data cube")]
    pub three_three_dimensional_mass_spectrum_data_cube:
        Option<ThreeDimensionalMassSpectrumDataCube>,
    #[serde(rename = "processed data document")]
    pub processed_data_document: Option<ProcessedDataDocument>,
}

impl InsertIntoGraph for MeasurementDocument {
    fn insert_into(&self, builder: &mut GraphBuilder, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &allores::AFR_0002375.as_simple() as &dyn InsertIntoGraph),
            (allores::AFR_0001121, &self.measurement_identifier.as_simple()),
            // TO-DO: needs further definition to be integrated
            // (allores::AFR_0002607, &self.chromatography_column_document),
            (allores::AFR_0002526, &self.device_control_aggregate_document),
            (allores::AFR_0002083, &self.sample_document),
            (allores::AFR_0002529, &self.injection_document),
            (allores::AFR_0002534, &self.detection_type.as_simple()),
            (allores::AFR_0002550, &self.chromatogram_data_cube),
            (allores::AFR_0002551, &self.three_dimensional_ultraviolet_spectrum_data_cube),
            (allores::AFR_0002878, &self.three_three_dimensional_mass_spectrum_data_cube),
            (allores::AFR_0002659, &self.processed_data_document),
        ] {
            value.attach_into(
                builder,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChromatographyColumnDocument {}

impl InsertIntoGraph for ChromatographyColumnDocument {
    fn insert_into(&self, builder: &mut GraphBuilder, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in
            [(rdf::type_, &cat::ChromatographyColumnDocument.as_simple() as &dyn InsertIntoGraph)]
        {
            value.attach_into(
                builder,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceSystemDocument {
    #[serde(alias = "device document", alias = "device control document")]
    pub device_document: Vec<DeviceDocument>,
    #[serde(rename = "asset management identifier")]
    pub asset_management_identifier: Option<String>,
}

impl InsertIntoGraph for DeviceSystemDocument {
    fn insert_into(&self, builder: &mut GraphBuilder, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::DeviceSystemDocument.as_simple() as &dyn InsertIntoGraph),
            (allores::AFR_0002722, &self.device_document),
            (
                allores::AFR_0001976,
                &self.asset_management_identifier.as_ref().clone().map(|s| s.as_simple()),
            ),
        ] {
            value.attach_into(
                builder,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceDocument {
    #[serde(rename = "device identifier")]
    pub device_identifier: String,
    #[serde(rename = "device type")]
    pub device_type: String,
    #[serde(rename = "product manufacturer")]
    pub product_manufacturer: String,
    #[serde(rename = "equipment serial number")]
    pub equipment_serial_number: String,
    #[serde(rename = "model number")]
    pub model_number: String,
    #[serde(rename = "firmware version")]
    pub firmware_version: String,
    #[serde(rename = "detection type")]
    pub detection_type: Option<String>,
    #[serde(rename = "@index")]
    pub index: Option<i64>,
}

impl InsertIntoGraph for DeviceDocument {
    fn insert_into(&self, builder: &mut GraphBuilder, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &allores::AFR_0002567.as_simple() as &dyn InsertIntoGraph),
            (allores::AFR_0002018, &self.device_identifier.as_simple()),
            (allores::AFR_0002568, &self.device_type.as_simple()),
            (allores::AFR_0001258, &self.product_manufacturer.as_simple()),
            (allores::AFR_0001119, &self.equipment_serial_number.as_simple()),
            (obo::IAO_0000017, &self.model_number.as_simple()),
            (allores::AFR_0001259, &self.firmware_version.as_simple()),
            (allores::AFR_0002534, &self.detection_type.as_ref().clone().map(|s| s.as_simple())),
            // TO-DO issue with unpacking the index
            //(allohdfcube::Index, &self.index)
        ] {
            value.attach_into(
                builder,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessedDataDocument {
    #[serde(rename = "peak list")]
    pub peak_list: PeakList,
}

impl InsertIntoGraph for ProcessedDataDocument {
    fn insert_into(&self, builder: &mut GraphBuilder, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::ProcessedDataDocument.as_simple() as &dyn InsertIntoGraph),
            (allores::AFR_0000432, &self.peak_list),
        ] {
            value.attach_into(
                builder,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SampleDocument {
    #[serde(rename = "sample identifier")]
    pub sample_identifier: String,
    #[serde(rename = "written name")]
    pub written_name: String,
}

impl InsertIntoGraph for SampleDocument {
    fn insert_into(&self, builder: &mut GraphBuilder, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::SampleDocument.as_simple() as &dyn InsertIntoGraph),
            (allores::AFR_0001118, &self.sample_identifier.as_simple()),
            (obo::IAO_0000590, &self.written_name.as_simple()),
        ] {
            value.attach_into(
                builder,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InjectionDocument {
    #[serde(rename = "autosampler injection volume setting (chromatography)")]
    pub autosampler_injection: AutosamplerInjectionVolumeSetting,
    #[serde(rename = "injection identifier")]
    pub injection_identifier: String,
    #[serde(rename = "injection time")]
    pub injection_time: String,
}

impl InsertIntoGraph for InjectionDocument {
    fn insert_into(&self, builder: &mut GraphBuilder, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::InjectionDocument.as_simple() as &dyn InsertIntoGraph),
            (allores::AFR_0001267, &self.autosampler_injection),
            (allores::AFR_0002535, &self.injection_identifier.as_simple()),
            (allores::AFR_0002536, &(self.injection_time.as_str() * xsd::dateTime).as_simple()),
        ] {
            value.attach_into(
                builder,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChromatogramDataCube {
    pub label: Option<String>,
    #[serde(rename = "cube-structure")]
    pub cube_structure: CubeStructure,
    pub identifier: Option<String>,
}

impl InsertIntoGraph for ChromatogramDataCube {
    fn insert_into(&self, builder: &mut GraphBuilder, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::ChromatogramDataCube.as_simple() as &dyn InsertIntoGraph),
            (obo::IAO_0000009, &self.label.as_ref().clone().map(|s| s.as_simple())),
            (qb::structure, &self.cube_structure),
            (allores::AFR_0000917, &self.identifier.as_ref().clone().map(|s| s.as_simple())),
        ] {
            value.attach_into(
                builder,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThreeDimensionalUltravioletSpectrumDataCube {
    pub label: String,
    #[serde(rename = "cube-structure")]
    pub cube_structure: CubeStructure,
    pub identifier: String,
}

impl InsertIntoGraph for ThreeDimensionalUltravioletSpectrumDataCube {
    fn insert_into(&self, builder: &mut GraphBuilder, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (
                rdf::type_,
                &cat::ThreeDimensionalUltravioletSpectrumDataCube.as_simple()
                    as &dyn InsertIntoGraph,
            ),
            (obo::IAO_0000009, &self.label.as_simple()),
            (qb::structure, &self.cube_structure),
            (allores::AFR_0000917, &self.identifier.as_simple()),
        ] {
            value.attach_into(
                builder,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThreeDimensionalMassSpectrumDataCube {
    pub label: String,
    #[serde(rename = "cube-structure")]
    pub cube_structure: CubeStructure,
    pub identifier: String,
}

impl InsertIntoGraph for ThreeDimensionalMassSpectrumDataCube {
    fn insert_into(&self, builder: &mut GraphBuilder, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (
                rdf::type_,
                &cat::ThreeDimensionalMassSpectrumDataCube.as_simple() as &dyn InsertIntoGraph,
            ),
            (obo::IAO_0000009, &self.label.as_simple()),
            (qb::structure, &self.cube_structure),
            (allores::AFR_0000917, &self.identifier.as_simple()),
        ] {
            value.attach_into(
                builder,
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
    fn insert_into(&self, builder: &mut GraphBuilder, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (
                rdf::type_,
                &cat::AutosamplerInjectionVolumeSetting.as_simple() as &dyn InsertIntoGraph,
            ),
            (qudt::value, &self.value.as_simple()),
            (qudt::unit, &self.unit.iri().as_simple()),
        ] {
            value.attach_into(
                builder,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CubeStructure {
    pub measures: Vec<Measure>,
    pub dimensions: Vec<Dimension>,
}

impl InsertIntoGraph for CubeStructure {
    fn insert_into(&self, builder: &mut GraphBuilder, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::CubeStructure.as_simple() as &dyn InsertIntoGraph),
            (cat::measure, &self.measures),
            (cat::dimension, &self.dimensions),
        ] {
            value.attach_into(
                builder,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Measure {
    #[serde(rename = "@componentDatatype")]
    pub component_data_type: String, //subject to change
    pub concept: String,
    pub unit: Unit,
}

impl InsertIntoGraph for Measure {
    fn insert_into(&self, builder: &mut GraphBuilder, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &allorole::AFRL_0000157.as_simple() as &dyn InsertIntoGraph),
            (allodc::componentDataType, &self.component_data_type.as_simple()),
            (rdfs::label, &self.concept.as_simple()),
            (qudt::unit, &self.unit.iri().as_simple()),
        ] {
            value.attach_into(
                builder,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dimension {
    #[serde(rename = "@componentDatatype")]
    pub component_data_type: String, //subject to change
    pub concept: String,
    pub unit: Unit,
}

impl InsertIntoGraph for Dimension {
    fn insert_into(&self, builder: &mut GraphBuilder, iri: SimpleTerm) -> anyhow::Result<()> {
        for (pred, value) in [
            (rdf::type_, &cat::Dimension.as_simple() as &dyn InsertIntoGraph),
            (allodc::componentDataType, &self.component_data_type.as_simple()),
            (rdfs::label, &self.concept.as_simple()),
            (qudt::unit, &self.unit.iri().as_simple()),
        ] {
            value.attach_into(
                builder,
                Link { source_iri: iri.clone(), pred: pred.as_simple(), target_iri: None },
            )?;
        }
        Ok(())
    }
}
