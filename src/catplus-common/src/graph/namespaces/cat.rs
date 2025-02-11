use lazy_static::lazy_static;
use sophia::api::ns::Namespace;
use sophia_api::namespace;
namespace! {
    "http://example.org/cat#",
    AddAction,
    Batch,
    Campaign,
    campaignClass,
    ContainerPositionAndQuantity,
    Experiment,
    FiltrateAction,
    genericObjective,
    Observation,
    Sample,
    SetPressureAction,
    SetTemperatureAction,
    SetVacuumAction,
    ShakeAction,
    speedTumbleStirrerShape,
    casNumber,
    chemicalName,
    containerBarcode,
    containerID,
    dispenseType,
    errorMargin,
    expectedDatum,
    hasBatch,
    hasCampaign,
    hasContainerPositionAndQuantity,
    hasSample,
    hasChemical,
    internalBarCode,
    measuredQuantity,
    Objective,
    role,
    setTemperatureAction,
    speedInRPM,
    subEquipmentName,
    swissCatNumber,
    temperatureShakerShape,
    temperatureTumbleStirrerShape,
    vialShape
}
lazy_static! {
    pub static ref ns: Namespace<&'static str> = Namespace::new(PREFIX.as_str()).unwrap();
}
