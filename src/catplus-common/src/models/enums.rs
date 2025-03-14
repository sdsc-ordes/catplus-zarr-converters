use crate::graph::namespaces::{cat, qudt, qudtext};
use serde::{Deserialize, Serialize};
use sophia::api::ns::Namespace;
use sophia_api::ns::NsTerm;
use std::fmt;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[allow(non_snake_case, non_camel_case_types)]
pub enum Unit {
    #[serde(rename = "bar")]
    Bar,
    #[serde(rename = "°C")]
    DegC,
    #[serde(rename = "mg")]
    MilliGM,
    #[serde(rename = "g/mL")]
    GMPerMilliL,
    #[serde(rename = "g/mol")]
    GMPerMol,
    #[serde(rename = "mol/L")]
    MolPerL,
    #[serde(rename = "rpm")]
    RevPerMin,
    #[serde(rename = "mm^3")]
    MilliM3,
    #[serde(rename = "nM")]
    NanoM,
    #[serde(rename = "s")]
    SEC,
    #[serde(rename = "min")]
    MIN,
    #[serde(rename = "%")]
    PERCENT,
    #[serde(alias = "unitless", alias = "(unitless)")]
    UNITLESS,
    #[serde(rename = "Counts.s")]
    CountsPerSec,
    #[serde(rename = "mAU")]
    mAU,
    #[serde(rename = "mAU.s")]
    mAUs,
}

impl Unit {
    pub fn display_name(&self) -> &'static str {
        match self {
            Unit::Bar => "Bar",
            Unit::DegC => "DEG-C",
            Unit::MilliGM => "MilliGM",
            Unit::GMPerMilliL => "GM-PER-MilliL",
            Unit::GMPerMol => "GM-PER-MOL",
            Unit::MolPerL => "MOL-PER-L",
            Unit::RevPerMin => "REV-PER-MIN",
            Unit::MilliM3 => "MilliM3",
            Unit::SEC => "SEC",
            Unit::MIN => "MIN",
            Unit::UNITLESS => "UNITLESS",
            Unit::PERCENT => "PERCENT",
            Unit::CountsPerSec => "NUM-PER-SEC",
            Unit::NanoM => "NanoM",
            Unit::mAU => "MilliAbsorbanceUnit",
            Unit::mAUs => "MilliAbsorbanceUnitTimesSecond",
        }
    }

    /// Determines whether the unit belongs to qudt or allotrope qudt-ext.
    fn namespace(&self) -> &Namespace<&'static str> {
        match self {
            // Standard QUDT units
            Unit::Bar
            | Unit::DegC
            | Unit::MilliGM
            | Unit::GMPerMilliL
            | Unit::GMPerMol
            | Unit::MolPerL
            | Unit::RevPerMin
            | Unit::MilliM3
            | Unit::SEC
            | Unit::MIN
            | Unit::PERCENT
            | Unit::NanoM
            | Unit::UNITLESS
            | Unit::CountsPerSec => &qudt::ns,

            // QUDT-EXT units
            Unit::mAU | Unit::mAUs | Unit::mAUs => &qudtext::ns,
        }
    }
    pub fn iri(&self) -> String {
        self.namespace().get(self.display_name()).unwrap().to_string()
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}>", self.iri().to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(non_snake_case, non_camel_case_types)]
pub enum ActionName {
    AddAction,
    setTemperatureAction,
    filtrateAction,
    shakeAction,
    setVacuumAction,
    setPressureAction,
}

impl fmt::Display for ActionName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}>", self.iri().to_string())
    }
}

impl ActionName {
    pub fn iri(&self) -> NsTerm {
        match self {
            Self::AddAction => cat::AddAction,
            Self::setTemperatureAction => cat::SetTemperatureAction,
            Self::setPressureAction => cat::SetPressureAction,
            Self::shakeAction => cat::ShakeAction,
            Self::setVacuumAction => cat::SetVacuumAction,
            Self::filtrateAction => cat::FiltrateAction,
        }
    }
}
